use std::marker::Sized;

use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum LexerError {
    #[error("EOF Error")]
    EOF,
    #[error("regex error")]
    Regex(#[from] regex::Error),
    #[error("unknown token")]
    UNKNOWN,
}

pub trait Lex {
    fn lex<'a>(input: &'a str) -> Result<(&'a str, Self), LexerError>
    where
        Self: Sized;
}

pub trait Lexer {
    type TOKEN: Lex;
    fn next() -> Self::TOKEN;
}
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    LET, // let
    IDENT(&'a str),
    NUMBER(f64),
    ASSIGN, // =
    PLUS,   // +
}

impl Lex for Token<'_> {
    fn lex<'a>(input: &'a str) -> Result<(&'a str, Self), LexerError>
    where
        Self: Sized,
    {
        let let_regex = Regex::new(r#"let"#)?;
        Ok(if let Some(res) = let_regex.find(input) {
            (&input[res.end()..], Token::LET)
        } else {
            return Err(LexerError::UNKNOWN);
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Lex, Token};

    #[test]
    fn test_lex() {
        let (res_str, res_token) = Token::lex("let help").unwrap();
        println!("{}", res_str);
        assert_eq!(res_token, Token::LET);
        assert_eq!(res_str, " help")
    }
}
