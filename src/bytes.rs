pub fn next_crlf(v: &[u8]) -> Option<usize> {
    for (i, s) in v.windows(2).enumerate() {
        if s == b"\r\n" {
            return Some(i)
        }
    }
    None
}

pub fn parse_usize(v: &[u8]) -> Option<usize> {
    const SIZE: usize = std::mem::size_of::<usize>();
    if v.len() > SIZE {
        None
    } else {
        let mut expanded: [u8; SIZE];
        for (i, x) in v.iter().enumerate() {
            expanded[i] = *x;
        }
        Some(usize::from_be_bytes(expanded))
    }
}
