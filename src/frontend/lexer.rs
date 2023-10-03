use core::fmt::Debug;
use std::{str::from_utf8, error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,

    Fn,
    Let,
    Var,
    Undefined,
    If,
    Elif,
    Else,
    Mod,
    Struct,

    Sign(&'static str),
    Paren(char),

    NumLit(String),
    CharLit(u8),
    StrLit(Vec<u8>),

    Ident(String),
}

pub struct LexingError {
    message: String,
}
impl Error for LexingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Debug for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexingError: {}", self.message)
    }
}

pub struct Lexer<'a> {
    src: &'a [u8],
    i: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            src: source.as_bytes(),
            i: 0,
        }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, LexingError> {
        let mut ret = vec![self.parse_token()?];
        while *ret.last().unwrap() != Token::EOF {
            ret.push(self.parse_token()?);
        }
        Ok(ret)
    }

    fn parse_token(&mut self) -> Result<Token, LexingError> {
        self.skip_ws();
        match self.ch() {
            b'a'..=b'z' |
            b'A'..=b'Z' | b'_'
            => Ok(self.parse_ident_like()),

            b'0'..=b'9' => Ok(self.parse_numeric_literal()),

            b'\"' => self.parse_string_literal(),
            b'\'' => self.parse_character_literal(),

            b'(' | b')' | b'[' | b']' | b'{' | b'}'
            => Ok({
                self.read_ch();
                Token::Paren(self.src[self.i - 1] as char)
            }),

            b'+' | b'-' | b'*' |
            b'/' | b'%' | b'!' |
            b':' | b'=' | b'&' |
            b'|' | b'~' | b'<' |
            b'>' | b'.' | b',' |
            b'?' | b'$' | b'@'
            => self.parse_starts_with_sign(),

            0 => Ok(Token::EOF),

            _ => Err(LexingError { message: "Illegal Character".into() }),
        }
    }
    fn parse_ident_like(&mut self) -> Token {
        let prev_i = self.i;
        while self.ch().is_ascii_alphanumeric() || self.ch() == b'_' {
            self.read_ch();
        }
        match &self.src[prev_i..self.i] {
            b"fn" => Token::Fn,
            b"let" => Token::Let,
            b"var" => Token::Var,
            b"undefined" => Token::Undefined,
            b"if" => Token::If,
            b"elif" => Token::Elif,
            b"else" => Token::Else,
            b"mod" => Token::Mod,
            b"struct" => Token::Struct,
            other => Token::Ident(from_utf8(other).unwrap().into())
        }
    }

    fn parse_numeric_literal(&mut self) -> Token {
        let prev_i = self.i;
        while self.ch().is_ascii_digit() {
            self.read_ch();
        }
        if self.ch() != b'.' || (self.ch() == b'.' && !self.src[self.i + 1].is_ascii_digit()) {
            return Token::NumLit(from_utf8(&self.src[prev_i..self.i]).unwrap().into());
        }
        self.read_ch();
        while self.ch().is_ascii_digit() {
            self.read_ch();
        }
        Token::NumLit(from_utf8(&self.src[prev_i..self.i]).unwrap().into())
    }
    fn parse_string_literal(&mut self) -> Result<Token, LexingError> {
        self.read_ch();
        let mut ret = Vec::new();
        loop {
            if self.ch() == b'\"' {
                break;
            }
            if self.ch() == b'\0' {
                return Err(LexingError { message: "String Literal Has No End".into() });
            }
            ret.push(self.parse_string_character());
        }
        Ok(Token::StrLit(ret))
    }
    fn parse_character_literal(&mut self) -> Result<Token, LexingError> {
        self.read_ch();
        let ret = Token::CharLit(self.parse_string_character());
        self.read_ch();
        if self.ch() != b'\'' {
            Err(LexingError { message: "Invalid Character Literal".into() })
        } else {
            Ok(ret)
        }
    }
    fn parse_string_character(&mut self) -> u8 {
        todo!()
    }

    fn parse_starts_with_sign(&mut self) -> Result<Token, LexingError> {
        match self.ch() {
            b'+' => self.parse_starts_with_plus(),
            b'-' => self.parse_starts_with_minus(),
            b'*' => self.parse_starts_with_star(),
            b'/' => self.parse_starts_with_divide(),
            b'%' => self.parse_starts_with_modulus(),
            b'!' => self.parse_starts_with_exclamation_mark(),
            b':' => self.parse_starts_with_colon(),
            b'=' => self.parse_starts_with_equal(),
            b'&' => self.parse_starts_with_ampersand(),
            b'|' => self.parse_starts_with_pipe(),
            b'~' => self.parse_starts_with_wavey(),
            b'<' => self.parse_starts_with_smaller(),
            b'>' => self.parse_starts_with_greater(),
            b'.' => self.parse_starts_with_dot(),
            b',' => Ok(Token::Sign(",")),
            b'?' => Ok(Token::Sign("?")),
            b'$' => Ok(Token::Sign("$")),
            b'@' => Ok(Token::Sign("@")),
            other => Err(LexingError {
                message: format!("Unreachable character {other} was reached in function 'parse_starts_with_sign()'"),
            }),
        }
    }
    fn parse_starts_with_plus(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"++" {
            self.read_chs(2);
            Token::Sign("++")
        } else if self.chs(2) == b"+=" {
            self.read_chs(2);
            Token::Sign("+=")
        } else {
            self.read_ch();
            Token::Sign("+")
        })
    }
    fn parse_starts_with_minus(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"--" {
            self.read_chs(2);
            Token::Sign("--")
        } else if self.chs(2) == b"-=" {
            self.read_chs(2);
            Token::Sign("-=")
        } else {
            self.read_ch();
            Token::Sign("-")
        })
    }
    fn parse_starts_with_star(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"*=" {
            self.read_chs(2);
            Token::Sign("*=")
        } else {
            self.read_ch();
            Token::Sign("*")
        })
    }
    fn parse_starts_with_divide(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"/=" {
            self.read_chs(2);
            Token::Sign("/=")
        } else {
            self.read_ch();
            Token::Sign("/")
        })
    }
    fn parse_starts_with_modulus(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"%=" {
            self.read_chs(2);
            Token::Sign("%=")
        } else {
            self.read_ch();
            Token::Sign("%")
        })
    }
    fn parse_starts_with_exclamation_mark(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"!=" {
            self.read_chs(2);
            Token::Sign("!=")
        } else {
            self.read_ch();
            Token::Sign("!")
        })
    }
    fn parse_starts_with_colon(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b":=" {
            self.read_chs(2);
            Token::Sign(":=")
        } else if self.chs(2) == b"::" {
            self.read_chs(2);
            Token::Sign("::")
        } else {
            Token::Sign(":")
        })
    }
    fn parse_starts_with_equal(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"==" {
            self.read_chs(2);
            Token::Sign("==")
        } else if self.chs(2) == b"=>" {
            self.read_chs(2);
            Token::Sign("=>")
        } else {
            self.read_ch();
            Token::Sign("=")
        })
    }
    fn parse_starts_with_ampersand(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"&&" {
            self.read_chs(2);
            Token::Sign("&&")
        } else {
            self.read_ch();
            Token::Sign("&")
        })
    }
    fn parse_starts_with_pipe(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"||" {
            self.read_chs(2);
            Token::Sign("||")
        } else {
            self.read_ch();
            Token::Sign("|")
        })
    }
    fn parse_starts_with_wavey(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"~=" {
            self.read_chs(2);
            Token::Sign("~=")
        } else {
            self.read_ch();
            Token::Sign("~")
        })
    }
    fn parse_starts_with_smaller(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b"<=" {
            self.read_chs(2);
            Token::Sign("<=")
        } else {
            self.read_ch();
            Token::Sign("<")
        })
    }
    fn parse_starts_with_greater(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(2) == b">=" {
            self.read_chs(2);
            Token::Sign(">=")
        } else {
            self.read_ch();
            Token::Sign(">")
        })
    }
    fn parse_starts_with_dot(&mut self) -> Result<Token, LexingError> {
        Ok(if self.chs(3) == b"..=" {
            self.read_chs(3);
            Token::Sign("..=")
        } else if self.chs(2) == b".." {
            self.read_chs(2);
            Token::Sign("..")
        } else {
            self.read_ch();
            Token::Sign(".")
        })
    }

    fn skip_ws(&mut self) {
        while self.ch().is_ascii_whitespace() {
            self.read_ch();
        }
    }
    fn read_chs(&mut self, count: usize) {
        self.i += count;
    }
    fn read_ch(&mut self) {
        self.i += 1;
    }
    fn chs(&self, count: usize) -> &'a [u8] {
        &self.src[self.i..self.i+count]
    }
    fn ch(&self) -> u8 {
        self.src[self.i]
    }
}