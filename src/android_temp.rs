use std::fs;

pub fn run() -> String {
  // ANSI color codes
  const BOLD: &str = "\x1B[1m";
  const RED: &str = "\x1B[1;31m";
  const GREEN: &str = "\x1B[1;32m";
  const YELLOW: &str = "\x1B[1;33m";
  const NC: &str = "\x1B[0m";

  // Read temperature file
  let cpu_temp = match fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
    Ok(temp) => temp,
    Err(_) => {
      return format!(
        "{}Temperature:{} {}Error reading temperature{}",
        BOLD, NC, RED, NC
      );
    }
  };

  let temp_str = cpu_temp.trim();

  // Extract first 2 digits (convert millidegrees to degrees)
  let temp_c = if temp_str.len() >= 3 {
    &temp_str[0..2]
  } else {
    "00"
  };

  // Determine color based on temperature
  let temp_value: u32 = temp_c.parse().unwrap_or(0);
  let color = if temp_value < 60 {
    GREEN
  } else if temp_value <= 75 {
    YELLOW
  } else {
    RED
  };

  // Return formatted output
  format!("{}Temperature:{} {} {}°C{}\n", BOLD, NC, color, temp_c, NC)
}
