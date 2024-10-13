pub fn split_ls_output(output: &[u8]) -> Vec<&str> {
    std::str::from_utf8(output)
        .expect("unable to convert to text /bin/ls output")
        .split([' ', '\n', '\t', '\0'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}
