#[derive(Debug)]
pub enum PunctuationKind {
    Stop,
    Comma,
    Exclamation,
    Question,
    Unknown,
}

impl From<char> for PunctuationKind {
    fn from(raw: char) -> Self {
        match raw {
            '.' => PunctuationKind::Stop,
            ',' => PunctuationKind::Comma,
            '!' => PunctuationKind::Exclamation,
            '?' => PunctuationKind::Question,
            _ => PunctuationKind::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum Token<'i> {
    Word(&'i str),
    WordPosessive { raw: &'i str, fixed: String },
    Punctuation(PunctuationKind),
    Whitespace,
    Unknown(char),
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

    pub fn word(input: &str) -> IResult<&str, Token> {
        map(terminated(alpha1, not(tag("'"))), Token::Word)(input)
    }

    pub fn whitespace(input: &str) -> IResult<&str, Token> {
        map(space1, |_| Token::Whitespace)(input)
    }

    pub fn punctuation(input: &str) -> IResult<&str, Token> {
        map(one_of("?,!."), |c| Token::Punctuation(c.into()))(input)
    }

    pub fn token(input: &str) -> IResult<&str, Token> {
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
