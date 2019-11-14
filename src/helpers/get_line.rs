pub fn get_line(file: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut from_last_eol = 0;

    for (chr, _) in file.chars().zip(0..pos) {
        if chr == '\n' {
            line += 1;
            from_last_eol = 0;
        }

        from_last_eol += 1;
    }

    (line, from_last_eol)
}
