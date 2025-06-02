use std::process::Command;

pub fn run() -> String {
  // Check if figlet exists by trying to run it
  if !Command::new("figlet")
    .arg("--version")
    .status()
    .is_ok_and(|s| s.success())
  {
    return "figlet not found.\nPlease install figlet first".to_string();
  }

  // Define green color code
  const GREEN: &str = "\x1B[1;32m";
  const RESET: &str = "\x1B[0m";

  // Execute figlet and capture output
  match Command::new("figlet").arg("Termux").output() {
    Ok(output) => {
      if output.status.success() {
        let figlet_text = String::from_utf8_lossy(&output.stdout);
        format!("{}{}{}", GREEN, figlet_text, RESET)
      } else {
        "Error generating Termux banner".to_string()
      }
    }
    Err(_) => "Error running figlet command".to_string(),
  }
}
