/*
 *pub fn first_word(s: &String) -> usize {
 *    let bytes = s.as_bytes();
 *
 *    for (i, &item) in bytes.iter().enumerate() {
 *        if item == b' ' {
 *            return i;
 *        }
 *    }
 *
 *    s.len()
 *}
 */
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[derive(Debug, PartialEq)]
enum State {
    InWord,
    OutWord,
}
pub fn nth_word(s: &str, n: usize) -> Option<&str> {
    //使用模式匹配进行字符串的切片和查找
    if n == 0 {
        return None;
    }

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
                word_count += 1;
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
