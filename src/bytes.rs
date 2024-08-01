pub fn next_crlf(v: &[u8]) -> Option<usize> {
    for (i, s) in v.windows(2).enumerate() {
        if s == b"\r\n" {
            return Some(i)
        }
    }
    None
}
