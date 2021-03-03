//! Generated file, do not edit by hand, see `xtask/src/codegen`

use super::Token;
use crate::parsing;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[doc = "fn keyword"]
    FnKw,
    #[doc = "let keyword"]
    LetKw,
    #[doc = "true keyword"]
    TrueKw,
    #[doc = "false keyword"]
    FalseKw,
    #[doc = "if keyword"]
    IfKw,
    #[doc = "else keyword"]
    ElseKw,
    #[doc = "return keyword"]
    ReturnKw,
    #[doc = "number literal"]
    Number,
    #[doc = "string literal"]
    String,
    #[doc = "equality operator"]
    EqEq,
    #[doc = "inequality operator"]
    NotEq,
    #[doc = "left parenthesis delimiter"]
    LParen,
    #[doc = "right parenthesis delimiter"]
    RParen,
    #[doc = "left brace delimiter"]
    LBrace,
    #[doc = "right brace delimiter"]
    RBrace,
    #[doc = "left bracket delimiter"]
    LBracket,
    #[doc = "right bracket delimiter"]
    RBracket,
    #[doc = "colon punctuation"]
    Colon,
    #[doc = "assignment operator"]
    Eq,
    #[doc = "walrus operator"]
    Walrus,
    #[doc = "comma puncuation"]
    Comma,
    #[doc = "dot puncuation"]
    Dot,
    #[doc = "semicolon punctuation"]
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Caret,
    ShiftLeft,
    ShiftRight,
    Tilde,
    BitAnd,
    BitOr,
    Lt,
    Gt,
    LtEq,
    GtEq,
    LogicalAnd,
    LogicalOr,
    Bang,
    Ident,
    Eof,
    Error,
}
#[doc = r" A helper macro to get the token kind"]
#[macro_export]
macro_rules ! K { [fn] => { $ crate :: ast :: TokenKind :: FnKw } ; [let] => { $ crate :: ast :: TokenKind :: LetKw } ; [true] => { $ crate :: ast :: TokenKind :: TrueKw } ; [false] => { $ crate :: ast :: TokenKind :: FalseKw } ; [if] => { $ crate :: ast :: TokenKind :: IfKw } ; [else] => { $ crate :: ast :: TokenKind :: ElseKw } ; [return] => { $ crate :: ast :: TokenKind :: ReturnKw } ; [number] => { $ crate :: ast :: TokenKind :: Number } ; [string] => { $ crate :: ast :: TokenKind :: String } ; [==] => { $ crate :: ast :: TokenKind :: EqEq } ; [!=] => { $ crate :: ast :: TokenKind :: NotEq } ; ['('] => { $ crate :: ast :: TokenKind :: LParen } ; [')'] => { $ crate :: ast :: TokenKind :: RParen } ; ['{'] => { $ crate :: ast :: TokenKind :: LBrace } ; ['}'] => { $ crate :: ast :: TokenKind :: RBrace } ; ['['] => { $ crate :: ast :: TokenKind :: LBracket } ; [']'] => { $ crate :: ast :: TokenKind :: RBracket } ; [:] => { $ crate :: ast :: TokenKind :: Colon } ; [=] => { $ crate :: ast :: TokenKind :: Eq } ; [:=] => { $ crate :: ast :: TokenKind :: Walrus } ; [,] => { $ crate :: ast :: TokenKind :: Comma } ; [.] => { $ crate :: ast :: TokenKind :: Dot } ; [;] => { $ crate :: ast :: TokenKind :: Semicolon } ; [+] => { $ crate :: ast :: TokenKind :: Plus } ; [-] => { $ crate :: ast :: TokenKind :: Minus } ; [*] => { $ crate :: ast :: TokenKind :: Asterisk } ; [/] => { $ crate :: ast :: TokenKind :: Slash } ; [%] => { $ crate :: ast :: TokenKind :: Modulo } ; [^] => { $ crate :: ast :: TokenKind :: Caret } ; [<<] => { $ crate :: ast :: TokenKind :: ShiftLeft } ; [>>] => { $ crate :: ast :: TokenKind :: ShiftRight } ; [~] => { $ crate :: ast :: TokenKind :: Tilde } ; [&] => { $ crate :: ast :: TokenKind :: BitAnd } ; [|] => { $ crate :: ast :: TokenKind :: BitOr } ; [<] => { $ crate :: ast :: TokenKind :: Lt } ; [>] => { $ crate :: ast :: TokenKind :: Gt } ; [<=] => { $ crate :: ast :: TokenKind :: LtEq } ; [>=] => { $ crate :: ast :: TokenKind :: GtEq } ; [&&] => { $ crate :: ast :: TokenKind :: LogicalAnd } ; [||] => { $ crate :: ast :: TokenKind :: LogicalOr } ; [!] => { $ crate :: ast :: TokenKind :: Bang } ; [ident] => { $ crate :: ast :: TokenKind :: Ident } ; [eof] => { $ crate :: ast :: TokenKind :: Eof } ; }
#[macro_export]
#[doc = r" A helper macro to get the terminal type"]
macro_rules ! T { [fn] => { $ crate :: ast :: generated :: FnKw } ; [let] => { $ crate :: ast :: generated :: LetKw } ; [true] => { $ crate :: ast :: generated :: TrueKw } ; [false] => { $ crate :: ast :: generated :: FalseKw } ; [if] => { $ crate :: ast :: generated :: IfKw } ; [else] => { $ crate :: ast :: generated :: ElseKw } ; [return] => { $ crate :: ast :: generated :: ReturnKw } ; [number] => { $ crate :: ast :: generated :: Number } ; [string] => { $ crate :: ast :: generated :: String } ; [==] => { $ crate :: ast :: generated :: EqEq } ; [!=] => { $ crate :: ast :: generated :: NotEq } ; ['('] => { $ crate :: ast :: generated :: LParen } ; [')'] => { $ crate :: ast :: generated :: RParen } ; ['{'] => { $ crate :: ast :: generated :: LBrace } ; ['}'] => { $ crate :: ast :: generated :: RBrace } ; ['['] => { $ crate :: ast :: generated :: LBracket } ; [']'] => { $ crate :: ast :: generated :: RBracket } ; [:] => { $ crate :: ast :: generated :: Colon } ; [=] => { $ crate :: ast :: generated :: Eq } ; [:=] => { $ crate :: ast :: generated :: Walrus } ; [,] => { $ crate :: ast :: generated :: Comma } ; [.] => { $ crate :: ast :: generated :: Dot } ; [;] => { $ crate :: ast :: generated :: Semicolon } ; [+] => { $ crate :: ast :: generated :: Plus } ; [-] => { $ crate :: ast :: generated :: Minus } ; [*] => { $ crate :: ast :: generated :: Asterisk } ; [/] => { $ crate :: ast :: generated :: Slash } ; [%] => { $ crate :: ast :: generated :: Modulo } ; [^] => { $ crate :: ast :: generated :: Caret } ; [<<] => { $ crate :: ast :: generated :: ShiftLeft } ; [>>] => { $ crate :: ast :: generated :: ShiftRight } ; [~] => { $ crate :: ast :: generated :: Tilde } ; [&] => { $ crate :: ast :: generated :: BitAnd } ; [|] => { $ crate :: ast :: generated :: BitOr } ; [<] => { $ crate :: ast :: generated :: Lt } ; [>] => { $ crate :: ast :: generated :: Gt } ; [<=] => { $ crate :: ast :: generated :: LtEq } ; [>=] => { $ crate :: ast :: generated :: GtEq } ; [&&] => { $ crate :: ast :: generated :: LogicalAnd } ; [||] => { $ crate :: ast :: generated :: LogicalOr } ; [!] => { $ crate :: ast :: generated :: Bang } ; }
impl TokenKind {
    #[doc = r" Get the display of the TokenKind"]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FnKw => "fn",
            Self::LetKw => "let",
            Self::TrueKw => "true",
            Self::FalseKw => "false",
            Self::IfKw => "if",
            Self::ElseKw => "else",
            Self::ReturnKw => "return",
            Self::Number => "number",
            Self::String => "string",
            Self::EqEq => "==",
            Self::NotEq => "!=",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::Colon => ":",
            Self::Eq => "=",
            Self::Walrus => ":=",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Semicolon => ";",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Asterisk => "*",
            Self::Slash => "/",
            Self::Modulo => "%",
            Self::Caret => "^",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::Tilde => "~",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::LtEq => "<=",
            Self::GtEq => ">=",
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::Bang => "!",
            Self::Ident => "ident",
            Self::Eof => "eof",
            Self::Error => "error",
        }
    }
}
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnKw {
    pub token: Token,
}
impl parsing::Parse for FnKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::FnKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::FnKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for FnKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::FnKw)
    }
}
impl fmt::Display for FnKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetKw {
    pub token: Token,
}
impl parsing::Parse for LetKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LetKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LetKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LetKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LetKw)
    }
}
impl fmt::Display for LetKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrueKw {
    pub token: Token,
}
impl parsing::Parse for TrueKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::TrueKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::TrueKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for TrueKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::TrueKw)
    }
}
impl fmt::Display for TrueKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "true")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalseKw {
    pub token: Token,
}
impl parsing::Parse for FalseKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::FalseKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::FalseKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for FalseKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::FalseKw)
    }
}
impl fmt::Display for FalseKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "false")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfKw {
    pub token: Token,
}
impl parsing::Parse for IfKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::IfKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::IfKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for IfKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::IfKw)
    }
}
impl fmt::Display for IfKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseKw {
    pub token: Token,
}
impl parsing::Parse for ElseKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ElseKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::ElseKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for ElseKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::ElseKw)
    }
}
impl fmt::Display for ElseKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "else")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnKw {
    pub token: Token,
}
impl parsing::Parse for ReturnKw {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ReturnKw => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::ReturnKw.as_str(),
            )),
        }
    }
}
impl parsing::Peek for ReturnKw {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::ReturnKw)
    }
}
impl fmt::Display for ReturnKw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    pub token: Token,
}
impl parsing::Parse for Number {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Number => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Number.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Number {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Number)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    pub token: Token,
}
impl parsing::Parse for String {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::String => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::String.as_str(),
            )),
        }
    }
}
impl parsing::Peek for String {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::String)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqEq {
    pub token: Token,
}
impl parsing::Parse for EqEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::EqEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::EqEq.as_str(),
            )),
        }
    }
}
impl parsing::Peek for EqEq {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::EqEq)
    }
}
impl fmt::Display for EqEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "==")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEq {
    pub token: Token,
}
impl parsing::Parse for NotEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::NotEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::NotEq.as_str(),
            )),
        }
    }
}
impl parsing::Peek for NotEq {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::NotEq)
    }
}
impl fmt::Display for NotEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "!=")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LParen {
    pub token: Token,
}
impl parsing::Parse for LParen {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LParen => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LParen.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LParen {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LParen)
    }
}
impl fmt::Display for LParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RParen {
    pub token: Token,
}
impl parsing::Parse for RParen {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RParen => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::RParen.as_str(),
            )),
        }
    }
}
impl parsing::Peek for RParen {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::RParen)
    }
}
impl fmt::Display for RParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ")")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LBrace {
    pub token: Token,
}
impl parsing::Parse for LBrace {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LBrace => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LBrace.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LBrace {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LBrace)
    }
}
impl fmt::Display for LBrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBrace {
    pub token: Token,
}
impl parsing::Parse for RBrace {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RBrace => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::RBrace.as_str(),
            )),
        }
    }
}
impl parsing::Peek for RBrace {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::RBrace)
    }
}
impl fmt::Display for RBrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "}}")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LBracket {
    pub token: Token,
}
impl parsing::Parse for LBracket {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LBracket => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LBracket.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LBracket {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LBracket)
    }
}
impl fmt::Display for LBracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBracket {
    pub token: Token,
}
impl parsing::Parse for RBracket {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RBracket => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::RBracket.as_str(),
            )),
        }
    }
}
impl parsing::Peek for RBracket {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::RBracket)
    }
}
impl fmt::Display for RBracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "]")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Colon {
    pub token: Token,
}
impl parsing::Parse for Colon {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Colon => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Colon.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Colon {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Colon)
    }
}
impl fmt::Display for Colon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Eq {
    pub token: Token,
}
impl parsing::Parse for Eq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Eq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Eq.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Eq {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Eq)
    }
}
impl fmt::Display for Eq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "=")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Walrus {
    pub token: Token,
}
impl parsing::Parse for Walrus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Walrus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Walrus.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Walrus {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Walrus)
    }
}
impl fmt::Display for Walrus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":=")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comma {
    pub token: Token,
}
impl parsing::Parse for Comma {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Comma => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Comma.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Comma {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Comma)
    }
}
impl fmt::Display for Comma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ",")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dot {
    pub token: Token,
}
impl parsing::Parse for Dot {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Dot => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Dot.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Dot {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Dot)
    }
}
impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ".")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semicolon {
    pub token: Token,
}
impl parsing::Parse for Semicolon {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Semicolon => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Semicolon.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Semicolon {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Semicolon)
    }
}
impl fmt::Display for Semicolon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ";")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plus {
    pub token: Token,
}
impl parsing::Parse for Plus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Plus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Plus.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Plus {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Plus)
    }
}
impl fmt::Display for Plus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Minus {
    pub token: Token,
}
impl parsing::Parse for Minus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Minus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Minus.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Minus {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Minus)
    }
}
impl fmt::Display for Minus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asterisk {
    pub token: Token,
}
impl parsing::Parse for Asterisk {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Asterisk => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Asterisk.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Asterisk {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Asterisk)
    }
}
impl fmt::Display for Asterisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slash {
    pub token: Token,
}
impl parsing::Parse for Slash {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Slash => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Slash.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Slash {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Slash)
    }
}
impl fmt::Display for Slash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modulo {
    pub token: Token,
}
impl parsing::Parse for Modulo {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Modulo => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Modulo.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Modulo {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Modulo)
    }
}
impl fmt::Display for Modulo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caret {
    pub token: Token,
}
impl parsing::Parse for Caret {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Caret => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Caret.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Caret {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Caret)
    }
}
impl fmt::Display for Caret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftLeft {
    pub token: Token,
}
impl parsing::Parse for ShiftLeft {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ShiftLeft => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::ShiftLeft.as_str(),
            )),
        }
    }
}
impl parsing::Peek for ShiftLeft {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::ShiftLeft)
    }
}
impl fmt::Display for ShiftLeft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<<")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftRight {
    pub token: Token,
}
impl parsing::Parse for ShiftRight {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ShiftRight => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::ShiftRight.as_str(),
            )),
        }
    }
}
impl parsing::Peek for ShiftRight {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::ShiftRight)
    }
}
impl fmt::Display for ShiftRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">>")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tilde {
    pub token: Token,
}
impl parsing::Parse for Tilde {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Tilde => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Tilde.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Tilde {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Tilde)
    }
}
impl fmt::Display for Tilde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "~")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitAnd {
    pub token: Token,
}
impl parsing::Parse for BitAnd {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::BitAnd => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::BitAnd.as_str(),
            )),
        }
    }
}
impl parsing::Peek for BitAnd {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::BitAnd)
    }
}
impl fmt::Display for BitAnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitOr {
    pub token: Token,
}
impl parsing::Parse for BitOr {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::BitOr => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::BitOr.as_str(),
            )),
        }
    }
}
impl parsing::Peek for BitOr {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::BitOr)
    }
}
impl fmt::Display for BitOr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lt {
    pub token: Token,
}
impl parsing::Parse for Lt {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Lt => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Lt.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Lt {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Lt)
    }
}
impl fmt::Display for Lt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gt {
    pub token: Token,
}
impl parsing::Parse for Gt {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Gt => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Gt.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Gt {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Gt)
    }
}
impl fmt::Display for Gt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LtEq {
    pub token: Token,
}
impl parsing::Parse for LtEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LtEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LtEq.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LtEq {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LtEq)
    }
}
impl fmt::Display for LtEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<=")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtEq {
    pub token: Token,
}
impl parsing::Parse for GtEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::GtEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::GtEq.as_str(),
            )),
        }
    }
}
impl parsing::Peek for GtEq {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::GtEq)
    }
}
impl fmt::Display for GtEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ">=")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalAnd {
    pub token: Token,
}
impl parsing::Parse for LogicalAnd {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LogicalAnd => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LogicalAnd.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LogicalAnd {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LogicalAnd)
    }
}
impl fmt::Display for LogicalAnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&&")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOr {
    pub token: Token,
}
impl parsing::Parse for LogicalOr {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LogicalOr => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::LogicalOr.as_str(),
            )),
        }
    }
}
impl parsing::Peek for LogicalOr {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::LogicalOr)
    }
}
impl fmt::Display for LogicalOr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "||")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bang {
    pub token: Token,
}
impl parsing::Parse for Bang {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Bang => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(
                &token,
                TokenKind::Bang.as_str(),
            )),
        }
    }
}
impl parsing::Peek for Bang {
    fn peek(peeker: &mut parsing::Peeker<'_>) -> bool {
        matches!(peeker.nth(0), TokenKind::Bang)
    }
}
impl fmt::Display for Bang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "!")
    }
}
