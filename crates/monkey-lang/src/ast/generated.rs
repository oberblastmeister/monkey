//! Generated file, do not edit by hand, see `xtask/src/codegen`

use super::Token;
use crate::parsing;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    KwFn,
    KwLet,
    KwTrue,
    KwFalse,
    KwIf,
    KwElse,
    KwReturn,
    Number,
    String,
    Asterisk,
    Bang,
    BitAnd,
    BitOr,
    Caret,
    Colon,
    Comma,
    Dot,
    Eq,
    EqEq,
    Gt,
    GtEq,
    LBrace,
    LBracket,
    LParen,
    LogicalAnd,
    LogicalOr,
    Lt,
    LtEq,
    Minus,
    Modulo,
    NotEq,
    Plus,
    RBrace,
    RBracket,
    RParen,
    Semicolon,
    ShiftLeft,
    ShiftRight,
    Slash,
    Tilde,
    Walrus,
    Ident,
    Eof,
}
#[macro_export]
macro_rules ! Tok { [*] => { $ crate :: ast :: TokenKind :: Asterisk } ; [!] => { $ crate :: ast :: TokenKind :: Bang } ; [&] => { $ crate :: ast :: TokenKind :: BitAnd } ; [|] => { $ crate :: ast :: TokenKind :: BitOr } ; [^] => { $ crate :: ast :: TokenKind :: Caret } ; [:] => { $ crate :: ast :: TokenKind :: Colon } ; [,] => { $ crate :: ast :: TokenKind :: Comma } ; [.] => { $ crate :: ast :: TokenKind :: Dot } ; [=] => { $ crate :: ast :: TokenKind :: Eq } ; [==] => { $ crate :: ast :: TokenKind :: EqEq } ; [>] => { $ crate :: ast :: TokenKind :: Gt } ; [>=] => { $ crate :: ast :: TokenKind :: GtEq } ; ['{'] => { $ crate :: ast :: TokenKind :: LBrace } ; ['['] => { $ crate :: ast :: TokenKind :: LBracket } ; ['('] => { $ crate :: ast :: TokenKind :: LParen } ; [&&] => { $ crate :: ast :: TokenKind :: LogicalAnd } ; [||] => { $ crate :: ast :: TokenKind :: LogicalOr } ; [<] => { $ crate :: ast :: TokenKind :: Lt } ; [<=] => { $ crate :: ast :: TokenKind :: LtEq } ; [-] => { $ crate :: ast :: TokenKind :: Minus } ; [%] => { $ crate :: ast :: TokenKind :: Modulo } ; [!=] => { $ crate :: ast :: TokenKind :: NotEq } ; [+] => { $ crate :: ast :: TokenKind :: Plus } ; ['}'] => { $ crate :: ast :: TokenKind :: RBrace } ; [']'] => { $ crate :: ast :: TokenKind :: RBracket } ; [')'] => { $ crate :: ast :: TokenKind :: RParen } ; [;] => { $ crate :: ast :: TokenKind :: Semicolon } ; [<<] => { $ crate :: ast :: TokenKind :: ShiftLeft } ; [>>] => { $ crate :: ast :: TokenKind :: ShiftRight } ; [/] => { $ crate :: ast :: TokenKind :: Slash } ; [~] => { $ crate :: ast :: TokenKind :: Tilde } ; [:=] => { $ crate :: ast :: TokenKind :: Walrus } ; [fn] => { $ crate :: ast :: TokenKind :: KwFn } ; [let] => { $ crate :: ast :: TokenKind :: KwLet } ; [true] => { $ crate :: ast :: TokenKind :: KwTrue } ; [false] => { $ crate :: ast :: TokenKind :: KwFalse } ; [if] => { $ crate :: ast :: TokenKind :: KwIf } ; [else] => { $ crate :: ast :: TokenKind :: KwElse } ; [return] => { $ crate :: ast :: TokenKind :: KwReturn } ; [number] => { $ crate :: ast :: TokenKind :: Number } ; [string] => { $ crate :: ast :: TokenKind :: String } ; [ident] => { $ crate :: ast :: TokenKind :: Ident } ; [eof] => { $ crate :: ast :: TokenKind :: Eof } ; }
pub struct Number {
    pub token: Token,
}
impl parsing::Parse for Number {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Number => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct String {
    pub token: Token,
}
impl parsing::Parse for String {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::String => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Asterisk {
    pub token: Token,
}
impl parsing::Parse for Asterisk {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Asterisk => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Bang {
    pub token: Token,
}
impl parsing::Parse for Bang {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Bang => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct BitAnd {
    pub token: Token,
}
impl parsing::Parse for BitAnd {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::BitAnd => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct BitOr {
    pub token: Token,
}
impl parsing::Parse for BitOr {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::BitOr => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Caret {
    pub token: Token,
}
impl parsing::Parse for Caret {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Caret => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Colon {
    pub token: Token,
}
impl parsing::Parse for Colon {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Colon => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Comma {
    pub token: Token,
}
impl parsing::Parse for Comma {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Comma => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Dot {
    pub token: Token,
}
impl parsing::Parse for Dot {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Dot => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Eq {
    pub token: Token,
}
impl parsing::Parse for Eq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Eq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct EqEq {
    pub token: Token,
}
impl parsing::Parse for EqEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::EqEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Gt {
    pub token: Token,
}
impl parsing::Parse for Gt {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Gt => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct GtEq {
    pub token: Token,
}
impl parsing::Parse for GtEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::GtEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LBrace {
    pub token: Token,
}
impl parsing::Parse for LBrace {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LBrace => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LBracket {
    pub token: Token,
}
impl parsing::Parse for LBracket {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LBracket => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LParen {
    pub token: Token,
}
impl parsing::Parse for LParen {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LParen => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LogicalAnd {
    pub token: Token,
}
impl parsing::Parse for LogicalAnd {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LogicalAnd => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LogicalOr {
    pub token: Token,
}
impl parsing::Parse for LogicalOr {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LogicalOr => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Lt {
    pub token: Token,
}
impl parsing::Parse for Lt {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Lt => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct LtEq {
    pub token: Token,
}
impl parsing::Parse for LtEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::LtEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Minus {
    pub token: Token,
}
impl parsing::Parse for Minus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Minus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Modulo {
    pub token: Token,
}
impl parsing::Parse for Modulo {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Modulo => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct NotEq {
    pub token: Token,
}
impl parsing::Parse for NotEq {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::NotEq => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Plus {
    pub token: Token,
}
impl parsing::Parse for Plus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Plus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct RBrace {
    pub token: Token,
}
impl parsing::Parse for RBrace {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RBrace => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct RBracket {
    pub token: Token,
}
impl parsing::Parse for RBracket {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RBracket => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct RParen {
    pub token: Token,
}
impl parsing::Parse for RParen {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::RParen => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Semicolon {
    pub token: Token,
}
impl parsing::Parse for Semicolon {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Semicolon => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct ShiftLeft {
    pub token: Token,
}
impl parsing::Parse for ShiftLeft {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ShiftLeft => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct ShiftRight {
    pub token: Token,
}
impl parsing::Parse for ShiftRight {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::ShiftRight => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Slash {
    pub token: Token,
}
impl parsing::Parse for Slash {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Slash => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Tilde {
    pub token: Token,
}
impl parsing::Parse for Tilde {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Tilde => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct Walrus {
    pub token: Token,
}
impl parsing::Parse for Walrus {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::Walrus => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwFn {
    pub token: Token,
}
impl parsing::Parse for KwFn {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwFn => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwLet {
    pub token: Token,
}
impl parsing::Parse for KwLet {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwLet => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwTrue {
    pub token: Token,
}
impl parsing::Parse for KwTrue {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwTrue => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwFalse {
    pub token: Token,
}
impl parsing::Parse for KwFalse {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwFalse => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwIf {
    pub token: Token,
}
impl parsing::Parse for KwIf {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwIf => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwElse {
    pub token: Token,
}
impl parsing::Parse for KwElse {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwElse => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
pub struct KwReturn {
    pub token: Token,
}
impl parsing::Parse for KwReturn {
    fn parse(p: &mut parsing::Parser<'_>) -> Result<Self, parsing::ParseError> {
        let token = p.next()?;
        match token.kind {
            TokenKind::KwReturn => Ok(Self { token }),
            _ => Err(parsing::ParseError::expected(&token, "abstract")),
        }
    }
}
