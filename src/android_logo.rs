pub fn run() -> String {
  // Define ANSI color codes
  const GREEN: &[u8] = b"\x1B[1;32m";
  const WHITE: &[u8] = b"\x1B[39m";
  const RESET: &[u8] = b"\x1B[0m";

  // Define logo sections
  const TOP: &[u8] = b"             -o          o-
              +hydNNNNdyh+
            +mMMMMMMMMMMMMm+
          `dMM";

  const MIDDLE: &[u8] = b"MMd`
          hMMMMMMMMMMMMMMMMMMh
      ..  yyyyyyyyyyyyyyyyyyyy  ..
    .mMMm`MMMMMMMMMMMMMMMMMMMM`mMMm.";

  const BODY: &[u8] = b"
    :MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
    :MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
    :MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
    :MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
    -MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM-";

  const BOTTOM: &[u8] = b"
     +yy+ MMMMMMMMMMMMMMMMMMMM +yy+
          mMMMMMMMMMMMMMMMMMMm
          `/++MMMMh++hMMMM++/`
              MMMMo  oMMMM
              MMMMo  oMMMM
              oNMm-  -mMNs";

  // Build output string
  let mut output = Vec::new();

  output.extend_from_slice(GREEN);
  output.extend_from_slice(TOP);

  // Eyes with white colons
  output.extend_from_slice(WHITE);
  output.extend_from_slice(b"m:");
  output.extend_from_slice(GREEN);
  output.extend_from_slice(b"NMMMMMMN");
  output.extend_from_slice(WHITE);
  output.extend_from_slice(b":m");

  // Rest of logo in green
  output.extend_from_slice(GREEN);
  output.extend_from_slice(MIDDLE);
  output.extend_from_slice(BODY);
  output.extend_from_slice(BOTTOM);
  output.extend_from_slice(RESET);

  String::from_utf8_lossy(&output).to_string()
}
