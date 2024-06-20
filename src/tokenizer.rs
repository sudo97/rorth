use std::fmt::Display;

use crate::common;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Num(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    While,
    End, // TODO: If, Then, Else, EndIf, While, EndWhile
    If,
    Else,
    // Stack operations
    Dup,
    Swap,
    Rot,
    Over,
    Nip,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::While => "while".into(),
                TokenType::End => "end".into(),
                TokenType::Num(n) => n.to_string(),
                TokenType::Pop => "pop".into(),
                TokenType::Add => "+".into(),
                TokenType::Sub => "-".into(),
                TokenType::Mul => "*".into(),
                TokenType::Div => "/".into(),
                TokenType::Print => "print".into(),
                TokenType::Dup => "dup".into(),
                TokenType::Swap => "swap".into(),
                TokenType::Rot => "rot".into(),
                TokenType::Over => "over".into(),
                TokenType::Nip => "nip".into(),
                TokenType::If => "if".into(),
                TokenType::Else => "else".into(),
            }
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: usize,
    pub line: usize,
}

fn identifier(input: &str) -> Option<TokenType> {
    match input {
        "print" => Some(TokenType::Print),
        "pop" => Some(TokenType::Pop),
        "while" => Some(TokenType::While),
        "end" => Some(TokenType::End),
        "dup" => Some(TokenType::Dup),
        "swap" => Some(TokenType::Swap),
        "rot" => Some(TokenType::Rot),
        "over" => Some(TokenType::Over),
        "nip" => Some(TokenType::Nip),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        _ => None,
    }
}

fn is_identifier_char(c: &char) -> bool {
    c.is_ascii_alphanumeric() || *c == '_'
}

fn is_numeric_char(c: &char) -> bool {
    c.is_numeric()
}

fn is_not_newline(c: &char) -> bool {
    *c != '\n'
}
macro_rules! collect_while {
    ($idx:expr, $pos:expr, $chars:expr, $cond:expr) => {{
        let mut buf = String::new();
        buf.push(*$chars.get($idx).unwrap());
        while let Some(c) = $chars.get($idx + 1) {
            if $cond(c) {
                buf.push(*c);
                $idx += 1;
                $pos += 1;
            } else {
                break;
            }
        }
        buf
    }};
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, common::Error> {
    let mut tokens = Vec::new();

    let mut line = 1;
    let mut pos = 0;
    let mut idx = 0;
    let chars: Vec<char> = input.chars().collect();

    while let Some(c) = chars.get(idx) {
        use TokenType::*;
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
                tokens.push(Token {
                    token_type: Add,
                    pos,
                    line,
                });
            }
            '-' => {
                tokens.push(Token {
                    token_type: Sub,
                    pos,
                    line,
                });
            }
            '*' => {
                tokens.push(Token {
                    token_type: Mul,
                    pos,
                    line,
                });
            }
            '/' => {
                tokens.push(Token {
                    token_type: Div,
                    pos,
                    line,
                });
            }
            '#' => {
                collect_while!(idx, pos, chars, is_not_newline);
                idx += 1;
                pos = 0;
                line += 1;
            }
            c if is_numeric_char(c) => {
                let buf = collect_while!(idx, pos, chars, is_numeric_char);
                tokens.push(Token {
                    token_type: Num(buf.parse::<i32>().unwrap()),
                    pos: pos - buf.len() + 1,
                    line,
                });
            }
            c if is_identifier_char(c) => {
                let buf = collect_while!(idx, pos, chars, is_identifier_char);
                let tok_begin_pos = pos - buf.len() + 1;
                let token_type = identifier(&buf).ok_or(common::Error::UnknownToken {
                    word: buf,
                    pos: tok_begin_pos,
                    line,
                })?;

                tokens.push(Token {
                    token_type,
                    pos: tok_begin_pos,
                    line,
                });
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
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Num(3),
                pos: 1,
                line: 1,
            }])
        );
    }

    #[test]
    fn multiple_digits() {
        let input = "123";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Num(123),
                pos: 1,
                line: 1,
            }])
        );
    }

    #[test]
    fn multiple_digits_and_print() {
        let input = "123 print";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::Num(123),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Print,
                    pos: 5,
                    line: 1,
                }
            ])
        );
    }

    #[test]
    fn multiple_digits_and_print_and_pop() {
        let input = "123 print pop";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::Num(123),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Print,
                    pos: 5,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Pop,
                    pos: 11,
                    line: 1,
                }
            ])
        );
    }

    #[test]
    fn two_numbers() {
        let input = "3 4";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::Num(3),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(4),
                    pos: 3,
                    line: 1,
                }
            ])
        );
    }

    #[test]
    fn two_plus_two() {
        let input = "2 2 +";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::Num(2),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(2),
                    pos: 3,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Add,
                    pos: 5,
                    line: 1,
                }
            ])
        );
    }

    #[test]
    fn two_plus_two_minus_three() {
        let input = "2 2 + 3 -";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::Num(2),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(2),
                    pos: 3,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Add,
                    pos: 5,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(3),
                    pos: 7,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Sub,
                    pos: 9,
                    line: 1,
                }
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
                Token {
                    token_type: TokenType::Num(2),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(2),
                    pos: 3,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Add,
                    pos: 5,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(3),
                    pos: 7,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Sub,
                    pos: 9,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(4),
                    pos: 11,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Mul,
                    pos: 13,
                    line: 1,
                }
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
                Token {
                    token_type: TokenType::Num(2),
                    pos: 1,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(2),
                    pos: 3,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Add,
                    pos: 5,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(3),
                    pos: 7,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Sub,
                    pos: 9,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(4),
                    pos: 11,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Mul,
                    pos: 13,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Num(5),
                    pos: 15,
                    line: 1,
                },
                Token {
                    token_type: TokenType::Div,
                    pos: 17,
                    line: 1,
                }
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

    #[test]
    fn while_end() {
        let input = "while end";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    token_type: TokenType::While,
                    pos: 1,
                    line: 1
                },
                Token {
                    token_type: TokenType::End,
                    pos: 7,
                    line: 1
                }
            ])
        )
    }

    #[test]
    fn dup() {
        let input = "dup";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Dup,
                pos: 1,
                line: 1
            }])
        );
    }

    #[test]
    fn test_only_comment() {
        let input = "# This is a comment";
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![]));
    }

    #[test]
    fn test_comment_plus() {
        let input = "# This is a comment\n+";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Add,
                pos: 1,
                line: 2,
            }])
        );
    }
}

#[cfg(test)]
mod test_identifier {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("print"), Some(TokenType::Print));
    }

    #[test]
    fn test_identifier_not_found() {
        assert_eq!(identifier("unknown"), None);
    }

    #[test]
    fn test_pop() {
        assert_eq!(identifier("pop"), Some(TokenType::Pop));
    }

    #[test]
    fn test_while() {
        assert_eq!(identifier("while"), Some(TokenType::While))
    }

    #[test]
    fn test_end() {
        assert_eq!(identifier("end"), Some(TokenType::End))
    }

    #[test]
    fn test_dup() {
        assert_eq!(identifier("dup"), Some(TokenType::Dup))
    }

    #[test]
    fn test_swap() {
        assert_eq!(identifier("swap"), Some(TokenType::Swap))
    }

    #[test]
    fn test_rot() {
        assert_eq!(identifier("rot"), Some(TokenType::Rot))
    }

    #[test]
    fn test_over() {
        assert_eq!(identifier("over"), Some(TokenType::Over))
    }

    #[test]
    fn test_nip() {
        assert_eq!(identifier("nip"), Some(TokenType::Nip))
    }

    #[test]
    fn test_if_else() {
        assert_eq!(identifier("if"), Some(TokenType::If));
        assert_eq!(identifier("else"), Some(TokenType::Else));
    }

    #[test]
    fn test_anything() {
        // TODO: change me to return and Identifier(str) once such thing exists
        assert_eq!(identifier("anything"), None);
    }
}
