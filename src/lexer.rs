use std::iter::Peekable;

type Result<T = ()> = std::result::Result<T, ErrorKind>;

const SINGLE_CHAR_TOKENS: [char; 7] = ['(', ')', '+', '-', '*', '/', '='];

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Number(i32),
    Ident(String),
    Let,
    LBracket,
    RBracket,
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ErrorKind {
    UnexpectedToken(char),
}

pub(crate) struct Lexer {
    input: String,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
        }
    }

    pub(crate) fn run(&mut self) -> Result<Vec<Token>> {
        let mut tokens = self.input.chars().peekable();

        let mut result = vec![];

        while let Some(&lexeme) = tokens.peek() {
            // Easy handling of single char tokens
            if SINGLE_CHAR_TOKENS.contains(&lexeme) {
                let token = match lexeme {
                    '(' => Ok(Token::LBracket),
                    ')' => Ok(Token::RBracket),
                    '+' => Ok(Token::Plus),
                    '-' => Ok(Token::Minus),
                    '*' => Ok(Token::Times),
                    '/' => Ok(Token::Divide),
                    '=' => Ok(Token::Equal),
                    _ => Err(ErrorKind::UnexpectedToken(lexeme)),
                };

                result.push(token?);

                tokens.next();

                continue;
            }

            match lexeme {
                num @ '0'..='9' => {
                    tokens.next();

                    result.push(self.lex_number(num, &mut tokens)?);
                }
                ident @ 'a'..='z' | ident @ 'A'..='Z' => {
                    tokens.next();

                    let ident = self.lex_ident(ident, &mut tokens)?;

                    let token = match ident.as_str() {
                        "let" => Token::Let,
                        _ => Token::Ident(ident),
                    };

                    result.push(token);
                }
                ' ' | '\t' | '\r' => {
                    tokens.next();

                    continue;
                }
                _ => return Err(ErrorKind::UnexpectedToken(lexeme)),
            }
        }

        Ok(result)
    }

    fn lex_number<T: Iterator<Item = char>>(
        &self,
        num: char,
        tokens: &mut Peekable<T>,
    ) -> Result<Token> {
        let mut num = num
            .to_string()
            .parse::<i32>()
            .map_err(|_| ErrorKind::UnexpectedToken(num))?;

        while let Some(Ok(digit)) = tokens
            .peek()
            .map(|lexeme| lexeme.to_string().parse::<i32>())
        {
            num = (num * 10) + digit;

            tokens.next();
        }

        Ok(Token::Number(num))
    }

    fn lex_ident<T: Iterator<Item = char>>(
        &self,
        ident: char,
        tokens: &mut Peekable<T>,
    ) -> Result<String> {
        // Something like take_while would be here but it's not inclusive.
        let mut results = vec![];

        while let Some(lexeme) = tokens.peek() {
            if lexeme.is_alphanumeric() || *lexeme == '_' {
                results.push(*lexeme);
            } else {
                break;
            }

            tokens.next();
        }

        Ok(format!(
            "{}{}",
            ident,
            results.into_iter().collect::<String>()
        ))
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token, Result};

    macro_rules! lex_single_char_token {
        ($token_name:ident, $token:expr, $token_symbol:expr) => {
            #[test]
            fn $token_name() -> Result {
                let mut lexer = Lexer::new($token_symbol); 

                assert_eq!(lexer.run()?, vec![$token]);

                Ok(())
            }
        };
    }

    lex_single_char_token!(lexing_lbracket_works, Token::LBracket, "(");

    lex_single_char_token!(lexing_rbracket_works, Token::RBracket, ")");

    lex_single_char_token!(lexing_plus_works, Token::Plus, "+");

    lex_single_char_token!(lexing_minus_works, Token::Minus, "-");

    lex_single_char_token!(lexing_times_works, Token::Times, "*");

    lex_single_char_token!(lexing_divide_works, Token::Divide, "/");

    lex_single_char_token!(lexing_eq_works, Token::Equal, "=");

    #[test]
    fn lexing_numbers_works() -> Result {
        let mut single_num_lexer = Lexer::new(&"1");

        let mut multi_num_lexer = Lexer::new(&"3213");

        assert_eq!(single_num_lexer.run()?, vec![Token::Number(1)]);

        assert_eq!(multi_num_lexer.run()?, vec![Token::Number(3213)]);

        Ok(())
    }

    #[test]
    fn lexing_ident_works() -> Result {
        let mut single_char_lexer = Lexer::new(&"x");

        let mut multi_char_lexer = Lexer::new(&"heLLo");

        let mut must_not_start_with_num_lexer = Lexer::new(&"3ff");

        let mut can_contain_underscores = Lexer::new("x_y_z");

        assert_eq!(single_char_lexer.run()?, vec![Token::Ident("x".to_string())]);

        assert_eq!(multi_char_lexer.run()?, vec![Token::Ident("heLLo".to_string())]);

        assert_eq!(must_not_start_with_num_lexer.run()?, vec![Token::Number(3), Token::Ident("ff".to_string())]); 

        assert_eq!(can_contain_underscores.run()?, vec![Token::Ident("x_y_z".to_string())]); 

        Ok(())
    }
}
