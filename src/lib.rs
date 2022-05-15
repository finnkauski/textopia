pub use parsers::tokens;
use rayon::prelude::*;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref REPLACEMENTS: HashMap<&'static str, &'static str> = {
        vec![
            ("mr", "mister"),
            ("mrs", "miss"),
            ("phd", "doctor"),
            ("bsc", "bachelor"),
        ].into_iter().collect()
    };
}

#[derive(Debug)]
pub enum PunctuationKind {
    Stop,
    Comma,
    Exclamation,
    Question,
    Unknown(char),
}

impl From<char> for PunctuationKind {
    fn from(raw: char) -> Self {
        match raw {
            '.' => PunctuationKind::Stop,
            ',' => PunctuationKind::Comma,
            '!' => PunctuationKind::Exclamation,
            '?' => PunctuationKind::Question,
            c => PunctuationKind::Unknown(c),
        }
    }
}

impl From<PunctuationKind> for char {
    fn from(raw: PunctuationKind) -> Self {
        match raw {
            PunctuationKind::Stop => '.',
            PunctuationKind::Comma => ',',
            PunctuationKind::Exclamation => '!',
            PunctuationKind::Question => '?',
            PunctuationKind::Unknown(c) => c,
        }
    }
}

#[derive(Debug)]
pub enum Token<'i> {
    Word(&'i str),
    WordPosessive { raw: &'i str, fixed: String },
    Punctuation(PunctuationKind),
    Whitespace,
    Replaced { raw: &'i str, fixed: &'static str },
    Unknown(char),
}

pub fn to_string(tokens: Vec<Token>) -> String {
    let mut s = String::new();
    for token in tokens {
        match token {
            Token::Word(word) => s.push_str(word),
            Token::WordPosessive { raw: _, fixed } => s.push_str(&fixed),
            Token::Punctuation(kind) => s.push_str(&char::from(kind).to_string()),
            Token::Whitespace => s.push(' '),
            Token::Replaced { raw: _, fixed } => s.push_str(&fixed),
            Token::Unknown(c) => s.push(c),
        }
    }
    s
}

fn check(raw: &str) -> Option<Token> {
    REPLACEMENTS
        .get(raw)
        .map(|t| Token::Replaced { raw, fixed: t })
}

pub fn replace(tokens: Vec<Token>) -> Vec<Token> {
    tokens
        .into_par_iter()
        .map(|token| match token {
            Token::Word(raw) => check(raw).unwrap_or(token),
            Token::WordPosessive { raw, fixed: _ } => check(raw).unwrap_or(token),
            _ => token,
        })
        .collect()
}

pub mod parsers {
    use crate::Token;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{alpha1, anychar, one_of, space1};
    use nom::character::is_alphabetic;
    use nom::combinator::{all_consuming, map, not};
    use nom::multi::many0;
    use nom::sequence::terminated;
    use nom::IResult;

    pub fn possessive(input: &str) -> IResult<&str, Token> {
        map(take_while1(|c| is_alphabetic(c as u8) || c == '\''), |t| {
            Token::WordPosessive {
                raw: t,
                fixed: t.replace("'", ""),
            }
        })(input)
    }

    fn word(input: &str) -> IResult<&str, Token> {
        map(terminated(alpha1, not(tag("'"))), Token::Word)(input)
    }

    fn whitespace(input: &str) -> IResult<&str, Token> {
        map(space1, |_| Token::Whitespace)(input)
    }

    fn punctuation(input: &str) -> IResult<&str, Token> {
        map(one_of("?,!."), |c| Token::Punctuation(c.into()))(input)
    }

    fn token(input: &str) -> IResult<&str, Token> {
        alt((
            word,
            possessive,
            whitespace,
            punctuation,
            map(anychar, Token::Unknown),
        ))(input)
    }

    pub fn tokens<'i>(input: &'i str) -> IResult<&str, Vec<Token>> {
        all_consuming(many0(token))(input)
    }
}
