use crate::common;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Num(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, common::Error> {
    let mut tokens = Vec::new();

    let mut line = 1;
    let mut pos = 0;
    let mut idx = 0;
    let chars: Vec<char> = input.chars().collect();

    while let Some(c) = chars.get(idx) {
        pos += 1;
        match c {
            ' ' => {}
            '\n' => {
                line += 1;
                pos = 0;
            }
            '\r' => {
                pos = 0;
            }
            '\t' => {}
            '+' => {
                tokens.push(Token::Add);
            }
            '-' => {
                tokens.push(Token::Sub);
            }
            '*' => {
                tokens.push(Token::Mul);
            }
            '/' => {
                tokens.push(Token::Div);
            }
            c if c.is_numeric() => {
                let mut buf = String::new();
                buf.push(*c);
                while let Some(c) = chars.get(idx + 1) {
                    if c.is_numeric() {
                        buf.push(*c);
                        idx += 1;
                        pos += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Num(buf.parse::<i32>().unwrap()));
            }
            c if c.is_ascii_alphanumeric() || *c == '_' => {
                let mut buf = String::new();
                buf.push(*c);
                while let Some(c) = chars.get(idx + 1) {
                    if c.is_ascii_alphanumeric() || *c == '_' {
                        buf.push(*c);
                        idx += 1;
                        pos += 1;
                    } else {
                        break;
                    }
                }
                if buf == "print" {
                    tokens.push(Token::Print);
                } else if buf == "pop" {
                    tokens.push(Token::Pop);
                } else {
                    let len = buf.len();
                    return Err(common::Error::UnknownToken {
                        word: buf,
                        pos: pos - len + 1,
                        line,
                    });
                }
            }
            _ => {
                return Err(common::Error::UnknownToken {
                    word: c.to_string(),
                    pos,
                    line,
                })
            }
        }
        idx += 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn empty_input() {
        let input = "";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![]));
    }

    #[test]
    fn empty_line() {
        let input = "\n";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![]));
    }

    #[test]
    fn skips_whitespace() {
        let input = "   \n\t\r";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![]));
    }

    #[test]
    fn single_number() {
        let input = "3";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(3)]));
    }

    #[test]
    fn multiple_digits() {
        let input = "123";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(123)]));
    }

    #[test]
    fn multiple_digits_and_print() {
        let input = "123 print";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(123), Token::Print]));
    }

    #[test]
    fn multiple_digits_and_print_and_pop() {
        let input = "123 print pop";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(123), Token::Print, Token::Pop]));
    }

    #[test]
    fn two_numbers() {
        let input = "3 4";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(3), Token::Num(4)]));
    }

    #[test]
    fn two_plus_two() {
        let input = "2 2 +";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Token::Num(2), Token::Num(2), Token::Add]));
    }

    #[test]
    fn two_plus_two_minus_three() {
        let input = "2 2 + 3 -";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Num(2),
                Token::Num(2),
                Token::Add,
                Token::Num(3),
                Token::Sub
            ])
        );
    }

    #[test]
    fn two_plus_two_minus_three_times_four() {
        let input = "2 2 + 3 - 4 *";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Num(2),
                Token::Num(2),
                Token::Add,
                Token::Num(3),
                Token::Sub,
                Token::Num(4),
                Token::Mul
            ])
        );
    }

    #[test]
    fn two_plus_two_minus_three_times_four_divided_by_five() {
        let input = "2 2 + 3 - 4 * 5 /";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Num(2),
                Token::Num(2),
                Token::Add,
                Token::Num(3),
                Token::Sub,
                Token::Num(4),
                Token::Mul,
                Token::Num(5),
                Token::Div
            ])
        );
    }

    #[test]
    fn fails_if_word_is_not_a_number() {
        let input = "a";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Err(common::Error::UnknownToken {
                word: "a".to_string(),
                pos: 1,
                line: 1
            })
        );
    }

    #[test]
    fn produces_proper_position() {
        let input = "   a";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Err(common::Error::UnknownToken {
                word: "a".to_string(),
                pos: 4,
                line: 1
            })
        );
    }

    #[test]
    fn fails_if_unknown_token_not_at_line_3_pos_5() {
        let input = "2 3 +\n4 5 *\n    a 7 -";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Err(common::Error::UnknownToken {
                word: "a".to_string(),
                pos: 5,
                line: 3
            })
        );
    }

    #[test]
    fn points_at_the_beginning_of_the_unknown_token() {
        let input = "2 3 +\n4 5 *\n    abc 7 -";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Err(common::Error::UnknownToken {
                word: "abc".to_string(),
                pos: 5,
                line: 3
            })
        );
    }

    #[test]
    fn fails_for_unknown_symbol() {
        let input = " ^";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Err(common::Error::UnknownToken {
                word: "^".to_string(),
                pos: 2,
                line: 1
            })
        );
    }
}
