// 仅将 ASCII 空格视为单词分隔符。
pub fn first_word(s: &str) -> Option<&str> {
    if s.is_empty() {
        return None;
    }

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return Some(&s[0..i]);
        }
    }
    Some(s)
}

#[derive(Debug, PartialEq)]
enum State {
    InWord,
    OutWord,
}
pub fn nth_word(s: &str, n: usize) -> Option<&str> {
    // 使用模式匹配进行字符串切片和查找，仅将 ASCII 空格视为分隔符。
    let bytes = s.as_bytes();
    let mut state = State::OutWord;
    let mut word_count = 0;
    let mut start = 0;

    for (i, &b) in bytes.iter().enumerate() {
        match (&state, b) {
            (State::OutWord, b' ') => {
                //空白区
            }

            (State::OutWord, _) => {
                //词开始
                if word_count == n {
                    start = i;
                }
                state = State::InWord;
            }

            (State::InWord, b' ') => {
                //词结束
                if word_count == n {
                    return Some(&s[start..i]);
                }
                word_count += 1;
                state = State::OutWord;
            }

            (State::InWord, _) => {
                //词内部
            }
        }
    }

    if state == State::InWord && word_count == n {
        Some(&s[start..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{first_word, nth_word};

    #[test]
    fn first_word_returns_none_for_empty_input() {
        assert_eq!(first_word(""), None);
    }

    #[test]
    fn returns_requested_word() {
        let input = "hello rust world";

        assert_eq!(nth_word(input, 0), Some("hello"));
        assert_eq!(nth_word(input, 1), Some("rust"));
    }

    #[test]
    fn returns_none_when_word_is_out_of_bounds() {
        assert_eq!(nth_word("hello rust world", 3), None);
    }

    #[test]
    fn returns_none_for_empty_input() {
        assert_eq!(nth_word("", 0), None);
    }
}
