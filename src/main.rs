use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

// Import all your module files
mod android_logo;
mod android_logo_small;
mod android_temp;
mod disk_space;
mod sysinfo;
mod termux_banner;
mod uptime;

// Default enabled modules
const DEFAULT_MODULES: &[&str] = &[
  "android-logo-small",
  "sysinfo",
  "android-temp",
  "disk-space",
];

struct Config {
  enabled_modules: HashSet<String>,
}

impl Config {
  fn new() -> Self {
    let mut enabled_modules = HashSet::new();

    // Enable default modules if no args provided
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
      for module in DEFAULT_MODULES {
        enabled_modules.insert(module.to_string());
      }
    }

    Config { enabled_modules }
  }

  fn parse_args(&mut self) {
    let args: Vec<String> = env::args().collect();

    // If flags are provided, clear defaults and only use specified modules
    if args.len() > 1 {
      self.enabled_modules.clear();
    }

    for arg in &args[1..] {
      match arg.as_str() {
        "-l" => {
          self
            .enabled_modules
            .insert("android-logo-small".to_string());
        }
        "-L" => {
          self.enabled_modules.insert("android-logo".to_string());
        }
        "-b" => {
          self.enabled_modules.insert("termux-banner".to_string());
        }
        "-s" => {
          self.enabled_modules.insert("sysinfo".to_string());
        }
        "-u" => {
          self.enabled_modules.insert("uptime".to_string());
        }
        "-t" => {
          self.enabled_modules.insert("android-temp".to_string());
        }
        "-d" => {
          self.enabled_modules.insert("disk-space".to_string());
        }
        "-h" | "--help" => {
          self.show_help();
          std::process::exit(0);
        }
        _ => {
          eprintln!("Unknown flag: {}", arg);
          eprintln!("Use -h or --help for usage information");
          std::process::exit(1);
        }
      }
    }
  }

  fn show_help(&self) {
    println!("System Information Display");
    println!();
    println!(
      "Usage: {} [FLAGS]",
      env::args().next().unwrap_or_else(|| "program".to_string())
    );
    println!();
    println!("Available modules:");
    println!("  -l    Android logo (small)");
    println!("  -L    Android logo (big)");
    println!("  -b    Termux banner");
    println!("  -s    System information");
    println!("  -u    Uptime");
    println!("  -t    Android temperature");
    println!("  -d    Disk space");
    println!();
    println!("  -h, --help    Show this help message");
    println!();
    println!(
      "Default modules (when no flags provided): android-logo-small, sysinfo, android-temp, disk-space"
    );
  }

  fn get_enabled_modules(&self) -> Vec<String> {
    // Return modules in categorical order
    let module_categories = [
      // Display/Branding (always first)
      vec!["android-logo-small", "android-logo", "termux-banner"],
      // System Info (second)
      vec!["sysinfo", "uptime"],
      // Temperature (third)
      vec!["android-temp"],
      // Storage (fourth)
      vec!["disk-space"],
    ];

    let mut ordered_modules = Vec::new();

    for category in &module_categories {
      for &module in category {
        if self.enabled_modules.contains(module) {
          ordered_modules.push(module.to_string());
        }
      }
    }

    ordered_modules
  }
}

fn main() -> io::Result<()> {
  let mut config = Config::new();
  config.parse_args();

  let enabled_modules = config.get_enabled_modules();

  if enabled_modules.is_empty() {
    return Ok(());
  }

  // Create thread-safe output collection
  let output_map = Arc::new(Mutex::new(HashMap::new()));
  let mut handles = Vec::new();

  // Execute modules in parallel
  for module_name in &enabled_modules {
    let module_name = module_name.clone();
    let output_map = Arc::clone(&output_map);

    let handle = thread::spawn(move || {
      let output = match module_name.as_str() {
        "android-logo-small" => android_logo_small::run(),
        "android-logo" => android_logo::run(),
        "termux-banner" => termux_banner::run(),
        "sysinfo" => sysinfo::run(),
        "uptime" => uptime::run(),
        "android-temp" => android_temp::run(),
        "disk-space" => disk_space::run(),
        _ => String::new(),
      };

      if !output.is_empty() {
        let mut map = output_map.lock().unwrap();
        map.insert(module_name, output);
      }
    });

    handles.push(handle);
  }

  // Wait for all threads to complete
  for handle in handles {
    let _ = handle.join();
  }

  // Get the output map
  let output_map = match Arc::try_unwrap(output_map) {
    Ok(mutex) => mutex.into_inner().unwrap(),
    Err(_) => HashMap::new(),
  };

  // Clear screen and display all collected output in order
  print!("\x1B[2J\x1B[1;1H");

  for module_name in &enabled_modules {
    if let Some(output) = output_map.get(module_name) {
      print!("{}", output);
    }
  }

  io::stdout().flush()?;
  Ok(())
}
