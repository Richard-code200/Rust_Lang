/// 返回首个 ASCII 空格的字节索引；若无空格，则返回字符串的字节长度。
fn first_word_end_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[cfg(test)]
mod tests {
    use super::first_word_end_index;

    #[test]
    fn returns_ascii_space_index() {
        assert_eq!(first_word_end_index("hello world"), 5);
    }

    #[test]
    fn returns_byte_length_without_space() {
        assert_eq!(first_word_end_index("hello"), 5);
    }

    #[test]
    fn returns_zero_for_empty_string() {
        assert_eq!(first_word_end_index(""), 0);
    }
}
