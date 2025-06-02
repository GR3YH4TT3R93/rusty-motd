use std::io::{self, BufRead};
use std::process::{Command, Stdio};

pub fn run() -> String {
  // Configuration
  const MAX_USAGE: u32 = 95;
  const ALERT_USAGE: u32 = 75;
  const BAR_WIDTH: usize = 50;

  // ANSI color codes
  const GREEN: &str = "\x1B[1;32m";
  const RED: &str = "\x1B[1;31m";
  const YELLOW: &str = "\x1B[1;33m";
  const BOLD: &str = "\x1B[1m";
  const NC: &str = "\x1B[0m";

  // Unicode bar components
  const BAR_START: &str = "\u{ee03}";
  const BAR_FILLED: &str = "\u{ee04}";
  const BAR_EMPTY: &str = "\u{ee01}";
  const BAR_END_FULL: &str = "\u{ee05}";
  const BAR_END_EMPTY: &str = "\u{ee02}";

  let mut output = String::new();
  output.push_str(&format!("\n{}Disk Usage:{}\n", BOLD, NC));

  // Get disk usage information with more compatible df command
  let child = match Command::new("df")
    .args(["-H", "-t", "fuse"])
    .stdout(Stdio::piped())
    .spawn()
  {
    Ok(child) => child,
    Err(_) => {
      output.push_str(&format!("  {}Error running df command{}\n", RED, NC));
      return output;
    }
  };

  let reader = io::BufReader::new(child.stdout.unwrap());

  for line in reader.lines().skip(1) {
    let line = match line {
      Ok(line) => line,
      Err(_) => continue,
    };

    let fields: Vec<&str> = line.split_whitespace().collect();

    if fields.len() < 6 {
      continue;
    }

    // Different df versions have different column orders
    let usage_percent = if fields[4].ends_with('%') {
      fields[4]
    } else if fields[5].ends_with('%') {
      fields[5]
    } else {
      continue;
    };

    let usage = usage_percent
      .trim_end_matches('%')
      .parse::<u32>()
      .unwrap_or(0);
    let used_space = fields[2];
    let total_space = fields[1];
    let mount_point = fields.last().unwrap();

    // Calculate bar width
    let used_width = (usage as usize * BAR_WIDTH) / 100;

    // Determine color
    let color = if usage >= MAX_USAGE {
      RED
    } else if usage >= ALERT_USAGE {
      YELLOW
    } else {
      GREEN
    };

    // Build the bar
    let mut bar = String::new();
    bar.push_str(color);
    bar.push_str(BAR_START);
    bar.push_str(&BAR_FILLED.repeat(used_width));
    bar.push_str(&BAR_EMPTY.repeat(BAR_WIDTH - used_width));
    bar.push_str(if usage == MAX_USAGE {
      BAR_END_FULL
    } else {
      BAR_END_EMPTY
    });
    bar.push_str(NC);

    // Add to output
    output.push_str(&format!(
      "  {:<31}{:>3} used out of {:>4}\n",
      mount_point, used_space, total_space
    ));
    output.push_str(&format!("  {}\n", bar));
  }

  output
}
