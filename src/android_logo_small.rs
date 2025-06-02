pub fn run() -> String {
  // Define ANSI color codes
  const GREEN: &str = "\x1b[1;32m";
  const WHITE: &str = "\x1b[39m";
  const RESET: &str = "\x1b[0m";

  // Build the ASCII art with color codes
  let mut output = String::new();

  // First part in green
  output.push_str(GREEN);
  output.push_str("  ;,           ,;\n   ';,.-----.,;'\n  ,'           ',\n /    ");

  // Eyes in white
  output.push_str(WHITE);
  output.push_str("O     O");

  // Back to green for the rest
  output.push_str(GREEN);
  output.push_str("    \\\n|                 |\n'-----------------'\n");

  // Reset colors
  output.push_str(RESET);

  output
}
