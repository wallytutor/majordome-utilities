use std::io::Write;
use std::process::{Child, Command, Stdio};

// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum OsPlatform {
    Windows,
    Linux,
    Osx,
    Unknown,
}

pub fn get_os_platform() -> OsPlatform {
    if cfg!(target_os = "windows") {
        OsPlatform::Windows
    } else if cfg!(target_os = "linux") {
        OsPlatform::Linux
    } else if cfg!(target_os = "macos") {
        OsPlatform::Osx
    } else {
        OsPlatform::Unknown
    }
}

pub fn executable_name() -> Result<&'static str, &'static str> {
    match get_os_platform() {
        OsPlatform::Windows => Ok("gnuplot.exe"),
        OsPlatform::Linux | OsPlatform::Osx => Ok("gnuplot"),
        OsPlatform::Unknown => Err("not supported OS"),
    }
}

// ---------------------------------------------------------------------------

pub struct LineStyle {
    pub color: String,
    pub width: f64,
    pub point_type: i32,
    pub dash_type: i32,
}

impl LineStyle {
    pub fn to_string(&self, id: i32) -> String {
        format!(
            "set linestyle {} dt {} lw {} pt {} lc '{}'",
            id, self.dash_type, self.width, self.point_type, self.color
        )
    }
}

// ---------------------------------------------------------------------------

pub struct GnuplotInteractive {
    child: Option<Child>,
}

impl Default for GnuplotInteractive {
    fn default() -> Self {
        Self::new()
    }
}

impl GnuplotInteractive {
    pub fn new() -> Self {
        let mut child = None;

        match executable_name() {
            Ok(exe) => {
                match Command::new(exe)
                    .arg("-persist")
                    .stdin(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()
                {
                    Ok(c) => child = Some(c),
                    Err(e) => eprintln!("Failed to start gnuplot ({}): {}", exe, e),
                }
            }
            Err(e) => eprintln!("Failed to start gnuplot: {}", e),
        }

        Self { child }
    }

    pub fn write(&mut self, command: &str) {
        if let Some(stdin) = self.child.as_mut().and_then(|c| c.stdin.as_mut()) {
            let mut cmd = command.to_string();

            if !cmd.ends_with('\n') {
                cmd.push('\n');
            }

            if let Err(e) = stdin.write_all(cmd.as_bytes()) {
                eprintln!("Error writing to gnuplot: {}", e);
            }

            let _ = stdin.flush();
        }
    }

    pub fn png_cairo_output(&mut self, save_as: &str, options: Option<&str>) {
        let options_str = options.unwrap_or("font 'Sans,10' size 400,600");
        self.write(&format!("set term pngcairo enhanced color {}", options_str));
        self.write(&format!("set output '{}.png'", save_as));
    }

    pub fn data_block(&mut self, tag: &str, arrays: &[&[f64]]) {
        if arrays.is_empty() {
            return;
        }

        let length = arrays.iter().map(|a| a.len()).max().unwrap_or(0);
        self.write(&format!("{} << EOD", tag));

        for i in 0..length {
            let mut line = String::new();

            for (j, arr) in arrays.iter().enumerate() {
                if i < arr.len() {
                    line.push_str(&format!("{}", arr[i]));
                }

                if j < arrays.len() - 1 {
                    line.push_str("  ");
                }
            }

            self.write(&line);
        }

        self.write("EOD");
    }
}

impl Drop for GnuplotInteractive {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(b"unset output\nexit\n");
                let _ = stdin.flush();
            }

            let _ = child.wait();
        }
    }
}

// ---------------------------------------------------------------------------
