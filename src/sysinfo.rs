use std::process::Command;
use std::thread;

pub fn run() -> String {
  // ANSI color codes
  const W: &str = "\x1B[0;39m";
  const G: &str = "\x1B[1;32m";
  const C: &str = "\x1B[1;36m";
  const BOLD: &str = "\x1B[1m";
  const RESET: &str = "\x1B[0m";

  // Spawn all independent operations in parallel
  let uptime_load_handle = thread::spawn(get_uptime_and_load_combined);
  let memory_handle = thread::spawn(get_memory_info_direct);
  let cpu_handle = thread::spawn(get_cpu_count_direct);
  let process_handle = thread::spawn(get_process_info);
  let android_handle = thread::spawn(get_android_info);
  let kernel_handle = thread::spawn(get_kernel_info);

  // Collect results
  let (uptime, load1, load5, load15) = uptime_load_handle.join().unwrap();
  let (used_mem, avail_mem, total_mem) = memory_handle.join().unwrap();
  let cpu_count = cpu_handle.join().unwrap();
  let (user_procs, total_procs, _root_procs) = process_handle.join().unwrap();
  let (distro, model) = android_handle.join().unwrap();
  let kernel_info = kernel_handle.join().unwrap();

  // Build and return output string
  format!(
    "
{W}{BOLD}System Info:
{C}  Distro    : {W}{distro}
{C}  Host      : {W}{model}
{C}  Kernel    : {W}{kernel_info}

{C}  Uptime    : {W}{uptime}
{C}  Load      : {G}{load1}{W} (1m), {G}{load5}{W} (5m), {G}{load15}{W} (15m)
{C}  Processes : {G}{user_procs}{W} (user), {G}{total_procs}{W} (total)

{C}  CPU       : {G}{cpu_count}{W} vCPU core(s)
{C}  Memory    : {G}{used_mem}{W} used, {G}{avail_mem}{W} avail, {G}{total_mem}{W} total{RESET}
\n"
  )
}

// Combined uptime and load average from single call
fn get_uptime_and_load_combined() -> (String, String, String, String) {
  // Get uptime seconds from /proc/uptime (more reliable than parsing uptime command)
  let uptime = get_formatted_uptime();

  // Get load averages from uptime command
  let (load1, load5, load15) = if let Ok(output) = Command::new("uptime").output() {
    let output = String::from_utf8_lossy(&output.stdout);
    if let Some(loads_part) = output.split("average: ").nth(1) {
      let loads: Vec<_> = loads_part.split(',').map(|s| s.trim()).collect();
      if loads.len() >= 3 {
        (
          loads[0].to_string(),
          loads[1].to_string(),
          loads[2].to_string(),
        )
      } else {
        ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
      }
    } else {
      ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
    }
  } else {
    ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
  };

  (uptime, load1, load5, load15)
}

fn get_formatted_uptime() -> String {
  // Try to read from /proc/uptime first (most accurate)
  if let Ok(uptime_content) = std::fs::read_to_string("/proc/uptime") {
    if let Some(uptime_str) = uptime_content.split_whitespace().next() {
      if let Ok(uptime_seconds) = uptime_str.parse::<f64>() {
        return format_uptime_duration(uptime_seconds as u64);
      }
    }
  }

  // Fallback: parse uptime command output and convert to seconds
  if let Ok(output) = Command::new("uptime").output() {
    let output_str = String::from_utf8_lossy(&output.stdout);
    if let Some(seconds) = parse_uptime_to_seconds(&output_str) {
      return format_uptime_duration(seconds);
    }
  }

  "N/A".to_string()
}

fn parse_uptime_to_seconds(uptime_output: &str) -> Option<u64> {
  // Parse formats like:
  // " 12:34:56 up 2 days,  4:17,  1 user,  load average: ..."
  // " 12:34:56 up  4:17,  1 user,  load average: ..."
  // " 12:34:56 up 23 min,  1 user,  load average: ..."

  if let Some(up_part) = uptime_output.split(" up ").nth(1) {
    if let Some(time_part) = up_part.split(",  load average:").nth(0) {
      // Remove user count part
      let time_clean = time_part.split(",").collect::<Vec<_>>();
      let time_parts = if time_clean.len() > 1 && time_clean.last().unwrap().contains("user") {
        &time_clean[..time_clean.len() - 1]
      } else {
        &time_clean
      };

      let mut total_seconds = 0u64;

      for part in time_parts {
        let part = part.trim();

        if part.contains(" day") {
          if let Some(days_str) = part.split(" day").next() {
            if let Ok(days) = days_str.trim().parse::<u64>() {
              total_seconds += days * 24 * 60 * 60;
            }
          }
        } else if part.contains(" min") {
          if let Some(mins_str) = part.split(" min").next() {
            if let Ok(mins) = mins_str.trim().parse::<u64>() {
              total_seconds += mins * 60;
            }
          }
        } else if part.contains(":") {
          // Format like "4:17" (hours:minutes)
          let time_parts: Vec<&str> = part.split(':').collect();
          if time_parts.len() == 2 {
            if let (Ok(hours), Ok(minutes)) = (
              time_parts[0].trim().parse::<u64>(),
              time_parts[1].trim().parse::<u64>(),
            ) {
              total_seconds += hours * 60 * 60 + minutes * 60;
            }
          }
        }
      }

      return Some(total_seconds);
    }
  }

  None
}

fn format_uptime_duration(total_seconds: u64) -> String {
  const MINUTE: u64 = 60;
  const HOUR: u64 = MINUTE * 60;
  const DAY: u64 = HOUR * 24;
  const WEEK: u64 = DAY * 7;
  const MONTH: u64 = DAY * 30; // Approximate

  let mut remaining = total_seconds;
  let mut parts = Vec::new();

  if remaining >= MONTH {
    let months = remaining / MONTH;
    remaining %= MONTH;
    parts.push(format!(
      "{} month{}",
      months,
      if months == 1 { "" } else { "s" }
    ));
  }

  if remaining >= WEEK {
    let weeks = remaining / WEEK;
    remaining %= WEEK;
    parts.push(format!(
      "{} week{}",
      weeks,
      if weeks == 1 { "" } else { "s" }
    ));
  }

  if remaining >= DAY {
    let days = remaining / DAY;
    remaining %= DAY;
    parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
  }

  if remaining >= HOUR {
    let hours = remaining / HOUR;
    remaining %= HOUR;
    parts.push(format!(
      "{} hour{}",
      hours,
      if hours == 1 { "" } else { "s" }
    ));
  }

  if remaining >= MINUTE {
    let minutes = remaining / MINUTE;
    parts.push(format!(
      "{} minute{}",
      minutes,
      if minutes == 1 { "" } else { "s" }
    ));
  }

  if parts.is_empty() {
    "less than a minute".to_string()
  } else {
    format!("up {}", parts.join(", "))
  }
}

fn get_memory_info_direct() -> (String, String, String) {
  if let Ok(output) = Command::new("free").arg("-htm").output() {
    let output = String::from_utf8_lossy(&output.stdout);
    if let Some(line) = output.lines().find(|l| l.starts_with("Mem:")) {
      let parts: Vec<_> = line.split_whitespace().collect();
      if parts.len() >= 7 {
        return (
          parts[2].to_string(),
          parts[6].to_string(),
          parts[1].to_string(),
        );
      }
    }
  }
  ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
}

fn get_cpu_count_direct() -> String {
  Command::new("nproc")
    .arg("--all")
    .output()
    .or_else(|_| {
      Command::new("grep")
        .args(["-c", "^processor", "/proc/cpuinfo"])
        .output()
    })
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or_else(|_| "N/A".to_string())
}

fn get_process_info() -> (String, String, String) {
  match Command::new("ps").arg("-eo").arg("user=").output() {
    Ok(output) => {
      let output = String::from_utf8_lossy(&output.stdout);
      let mut user = 0;
      let mut root = 0;
      for line in output.lines() {
        if line == "root" {
          root += 1;
        } else if !line.is_empty() {
          user += 1;
        }
      }
      (
        user.to_string(),
        (user + root).to_string(),
        root.to_string(),
      )
    }
    Err(_) => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string()),
  }
}

fn get_android_info() -> (String, String) {
  // Get all properties in a single getprop call (no arguments = dump all properties)
  match Command::new("getprop").output() {
    Ok(output) => {
      let output_str = String::from_utf8_lossy(&output.stdout);

      let mut version = None;
      let mut brand = None;
      let mut model = None;

      // Parse the output looking for our specific properties
      for line in output_str.lines() {
        if let Some(value) = extract_prop_value(line, "ro.build.version.release") {
          version = Some(value);
        } else if let Some(value) = extract_prop_value(line, "ro.product.brand") {
          brand = Some(value);
        } else if let Some(value) = extract_prop_value(line, "ro.product.model") {
          model = Some(value);
        }

        // Early exit if we have all values
        if version.is_some() && brand.is_some() && model.is_some() {
          break;
        }
      }

      let android_version = version
        .map(|v| format!("Android {}", v))
        .unwrap_or_else(|| "Android".to_string());

      let device = match (brand, model) {
        (Some(b), Some(m)) => format!("{} {}", b, m),
        (Some(b), None) => b,
        (None, Some(m)) => m,
        (None, None) => "Unknown Device".to_string(),
      };

      (android_version, device)
    }
    Err(_) => ("Android".to_string(), "Unknown Device".to_string()),
  }
}

// Helper function to extract property value from getprop output line
// Format: [ro.property.name]: [value]
fn extract_prop_value(
  line: &str,
  prop_name: &str,
) -> Option<String> {
  if line.starts_with(&format!("[{}]:", prop_name)) {
    // Find the value part between the second set of brackets
    if let Some(start) = line.rfind(": [") {
      let value_part = &line[start + 3..];
      if let Some(end) = value_part.rfind(']') {
        let value = value_part[..end].trim();
        if !value.is_empty() {
          return Some(value.to_string());
        }
      }
    }
  }
  None
}

fn get_kernel_info() -> String {
  Command::new("uname")
    .args(["-sr"])
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or_else(|_| "N/A".to_string())
}
