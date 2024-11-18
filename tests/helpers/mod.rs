#![allow(dead_code)]

/// Splits the outbut by space, newline, tab or null characters
pub fn split_ls_output(output: &[u8]) -> Vec<&str> {
    std::str::from_utf8(output)
        .expect("unable to convert to text /bin/ls output")
        .split([' ', '\n', '\t', '\0'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

pub fn split_ls_column_output<'a, T: Into<&'a str>>(output: T) -> Vec<&'a str> {
    let string = output.into();
    string
        .split([' ', '\n', '\t', '\0'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

pub fn split_ls_output_by_newline(output: &[u8]) -> Vec<&str> {
    std::str::from_utf8(output)
        .expect("unable to convert to text /bin/ls output")
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}
