use std::process::Command;

pub fn run() -> String {
  // Execute uptime -p and handle output
  match Command::new("uptime").arg("-p").output() {
    Ok(output) => {
      if output.status.success() {
        format!("\n{}", String::from_utf8_lossy(&output.stdout))
      } else {
        "\nError getting uptime information".to_string()
      }
    }
    Err(_) => "\nError running uptime command".to_string(),
  }
}
