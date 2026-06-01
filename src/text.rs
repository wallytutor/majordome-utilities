use std::fmt;

pub fn dedent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();

    if lines.is_empty() {
        return String::new();
    }

    // Find minimum indentation (ignoring empty lines)
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    // Remove that indentation from all lines
    lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// A float wrapper that prints in scientific notation with:
/// - configurable total width and precision
/// - controlled exponent width and total width
/// - exponent always display a sign (+/-)
pub struct SciFmt {
    pub value: f64,
    pub precision: usize,
    pub total_width: usize,
    pub exponent_width: usize,
}

impl fmt::Display for SciFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1) Format using Rust's scientific notation
        let raw = format!("{:.*e}", self.precision, self.value);

        // 2) Split mantissa and exponent
        let (mant, exp) = raw.split_once('e').unwrap();

        // 3) Parse exponent and reformat with +02 / -05 / +10
        let exp_val: i32 = exp.parse().unwrap();
        let exp_fixed = format!("{:+0width$}", exp_val, width = 1 + self.exponent_width);

        // 4) Reassemble
        let final_str = format!("{mant}e{exp_fixed}");

        // 5) Apply width/alignment from the struct
        let width = std::cmp::max(self.total_width, final_str.len());

        if self.value >= 0.0 {
            write!(f, " {:>width$}", final_str, width = width)
        } else {
            write!(f, "{:>width$}", final_str, width = width)
        }
    }
}

pub fn exponential_fmt(value: f64) -> String {
    let fmt = SciFmt {
        value,
        precision: 8,
        total_width: 12,
        exponent_width: 2,
    };
    fmt.to_string()
}
