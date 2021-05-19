//! Inspired by rustc_lexer and Rob Pike lexer

#![allow(clippy::unnecessary_wraps)]

use smol_str::SmolStr;
use text_size::{TextRange, TextSize};
use unicode_xid::UnicodeXID;

use super::{ParseError, ParseErrorKind, ParseResult, ToEof};
use crate::ast::{Token, TokenKind};
use crate::{Span, Spanned};
use std::convert::TryFrom;
use std::u32;
use std::{convert::TryInto, str::Chars};

#[derive(Debug)]
struct Source<'a> {
    start: u32,
    input_len: u32,
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Source<'a> {
    fn new(input: &'a str) -> Source<'a> {
        Source { start: 0, input, input_len: input.len().try_into().unwrap(), chars: input.chars() }
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn chars_len(&self) -> u32 {
        self.chars.as_str().len().try_into().unwrap()
    }

    fn start(&self) -> u32 {
        self.start
    }

    fn accept(&mut self, c: char) -> bool {
        match self.peek() {
            Some(peek_char) if peek_char == c => {
                self.next();
                true
            }
            _ => false,
        }
    }

    fn accept_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }

            self.next();
        }
    }

    /// Returns the amount of bytes consumed by the lexer
    fn pos(&self) -> u32 {
        (self.input_len - self.chars_len()).try_into().unwrap()
    }

    fn peek(&self) -> Option<char> {
        self.chars().next()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.chars().nth(n)
    }

    fn bump_pos(&mut self) {
        self.start = self.pos()
    }

    fn bump_span(&mut self) -> Span {
        self.bump_pos();
        self.span()
    }

    fn text(&mut self) -> SmolStr {
        self.input[TextRange::new(self.start().into(), self.pos().into())].into()
    }

    fn bump(&mut self) -> Span {
        let res = self.span();
        self.bump_pos();
        res
    }

    fn skip_next(&mut self) -> Option<char> {
        let next = self.next()?;
        self.bump_pos();
        Some(next)
    }
}

impl Spanned for Source<'_> {
    fn span(&self) -> Span {
        Span::new(self.start().into(), self.pos().into())
    }
}

impl Iterator for Source<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Source<'a>,
}

pub fn tokenize(input: &str) -> Vec<ParseResult<Token>> {
    Lexer::new(input).lex_until_eof()
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { source: Source::new(input) }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        let next_char = self.source.next().eof(|| self.source.bump_span())?;
        let peek_char = self.source.peek();

        let kind = match next_char {
            '/' => match peek_char {
                Some('/') => {
                    self.source.find(|c| *c == '\n');
                    self.source.bump_pos();
                    return self.next_token();
                }
                _ => {
                    self.source.next();
                    K![/]
                }
            },

            '=' => match peek_char {
                Some('=') => {
                    self.source.next();
                    K![==]
                }
                _ => K![=],
            },

            '!' => match peek_char {
                Some('=') => {
                    self.source.next();
                    K![!=]
                }
                _ => K![!],
            },

            '<' => match peek_char {
                Some('<') => {
                    self.source.next();
                    K![<<]
                }
                Some('=') => {
                    self.source.next();
                    K![<=]
                }
                _ => K![<],
            },

            '>' => match peek_char {
                Some('>') => {
                    self.source.next();
                    K![>>]
                }
                Some('=') => {
                    self.source.next();
                    K![>=]
                }
                _ => K![>],
            },

            ':' => match peek_char {
                Some('=') => {
                    self.source.next();
                    K![:=]
                }
                _ => {
                    K![:]
                }
            },

            ';' => K![;],
            ',' => K![,],
            '.' => K![.],

            '(' => K!['('],
            ')' => K![')'],
            '[' => K!['['],
            ']' => K![']'],
            '{' => K!['{'],
            '}' => K!['}'],

            // operators
            '^' => K![^],
            '*' => K![*],
            '%' => K![%],
            '+' => K![+],
            '-' => K![-],
            '~' => K![~],

            // literals
            c @ '"' => return self.string_lit(c),
            c @ '\'' => return self.string_lit(c),
            '0'..='9' => return self.number_lit(),

            // special
            _ if is_whitespace(next_char) => return self.whitespace(),
            _ if next_char.is_xid_start() => return self.maybe_ident(),
            _ => {
                let span = self.source.bump_span();
                return Err(ParseError::new(&span, ParseErrorKind::Unknown));
            }
        };

        let span = self.source.bump();

        Ok(Token { span, kind })
    }

    fn lex_until_eof(&mut self) -> Vec<ParseResult<Token>> {
        let mut results = Vec::new();

        loop {
            let result = self.next_token();
            match result {
                Err(e) if e.kind() == &ParseErrorKind::Eof => return results,
                t => results.push(t),
            }
        }
    }

    fn whitespace(&mut self) -> ParseResult<Token> {
        self.source.accept_while(is_whitespace);
        self.source.bump_pos();
        self.next_token()
    }

    fn string_lit(&mut self, delimit: char) -> ParseResult<Token> {
        self.source.bump_pos(); // get rid opening "
        self.source.accept_while(|c| c != delimit);
        let span = self.source.bump();
        if let Some(c) = self.source.skip_next() {
            if c != delimit {
                panic!("This  should not happen because of accept_while")
            }
        } else {
            panic!("Could not find end of string") // fix error reporting
        }
        Ok(Token { span, kind: K![string] })
    }

    fn number_lit(&mut self) -> ParseResult<Token> {
        self.source.accept_while(|c| ('0'..'9').contains(&c) || c == '_');
        let span = self.source.bump();
        Ok(Token { span, kind: K![number] })
    }

    fn maybe_ident(&mut self) -> ParseResult<Token> {
        self.source.accept_while(UnicodeXID::is_xid_continue);
        let span = self.source.bump();
        // let kind = keyword(&text).unwrap_or(K![ident]);
        let kind = K![ident];
        Ok(Token { span, kind })
    }
}

fn keyword(text: &str) -> Option<TokenKind> {
    Some(match text {
        "fn" => K![fn],
        "let" => K![let],
        "return" => K![return],
        "if" => K![if],
        "else" => K![else],
        "true" => K![true],
        "false" => K![false],
        _ => return None,
    })
}

/// Taken from rustc_lexer
/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos() {
        let mut lexer = Lexer::new("test");
        assert_eq!(lexer.source.pos(), 0);
        assert!(lexer.source.next().is_some());
        assert_eq!(lexer.source.pos(), 1);
        assert!(lexer.source.next().is_some());
        assert_eq!(lexer.source.pos(), 2);
        assert!(lexer.source.next().is_some());
        assert_eq!(lexer.source.pos(), 3);
        assert!(lexer.source.next().is_some());
        assert_eq!(lexer.source.pos(), 4);
        assert!(lexer.source.next().is_none());
        assert_eq!(lexer.source.pos(), 4);
        assert!(lexer.source.next().is_none());
        assert_eq!(lexer.source.pos(), 4);
    }

    #[test]
    fn nothing() {
        assert!(Lexer::new("").lex_until_eof().is_empty());
    }

    #[test]
    fn whitespace() {
        assert!(Lexer::new("\t\t\n              \t\n").lex_until_eof().is_empty());
    }
}
