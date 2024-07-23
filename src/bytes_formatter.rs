pub struct BytesFormatter;

impl BytesFormatter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print_bytes(&self, bytes: &[u8], n: usize) {
        const COLUMN: usize = 16;
        eprintln!("    {}", ansi_term::Colour::Green.bold().paint(format!("Size: {}", n)));
        for byte_row in bytes[..n].chunks(COLUMN) {
            eprint!("      ");
            for b in byte_row {
                eprint!("{}", ansi_term::Colour::Cyan.paint(format!("{:02x} ", b)));
            }
            eprint!("{}", String::from_utf8(vec![b' '; 10 + (COLUMN - byte_row.len()) * 3]).unwrap());
            for b in byte_row {
                eprint!("{}", if b.is_ascii_graphic() {
                    format!("{}", *b as char)
                } else {
                    ".".to_string()
                });
            }
            eprintln!();
        }
    }
}


