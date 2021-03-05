//! Inspired by rustc_lexer and Rob Pike lexer

#![allow(clippy::unnecessary_wraps)]

use smol_str::SmolStr;
use unicode_xid::UnicodeXID;

use super::{ParseError, ParseErrorKind, ParseResult, ToEof};
use crate::ast::{Token, TokenKind};
use crate::{Span, Spanned};
use std::str::Chars;

#[derive(Debug)]
struct Source<'a> {
    start: usize,
    input_len: usize,
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Source<'a> {
    fn new(input: &'a str) -> Source<'a> {
        Source {
            start: 0,
            input,
            input_len: input.len(),
            chars: input.chars(),
        }
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn chars_len(&self) -> usize {
        self.chars.as_str().len()
    }

    fn start(&self) -> usize {
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
    fn pos(&self) -> usize {
        self.input_len - self.chars_len()
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
        self.input[self.start()..self.pos()].into()
    }

    fn bump(&mut self) -> (Span, SmolStr) {
        let res = (self.span(), self.text());
        self.bump_pos();
        res
    }
}

impl Spanned for Source<'_> {
    fn span(&self) -> Span {
        Span::new(self.start(), self.pos())
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

pub fn lex(input: &str) -> Vec<ParseResult<Token>> {
    Lexer::new(input).lex_until_eof()
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            source: Source::new(input),
        }
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

        let (span, text) = self.source.bump();

        Ok(Token { span, kind, text })
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
        self.source
            .find(|c| *c == delimit)
            .eof(|| self.source.bump_span())?;
        let (span, text) = self.source.bump();
        Ok(Token {
            span,
            text,
            kind: K![string],
        })
    }

    fn number_lit(&mut self) -> ParseResult<Token> {
        self.source
            .accept_while(|c| ('0'..'9').contains(&c) || c == '_');
        let (span, text) = self.source.bump();
        Ok(Token {
            span,
            text,
            kind: K![number],
        })
    }

    fn maybe_ident(&mut self) -> ParseResult<Token> {
        self.source.accept_while(UnicodeXID::is_xid_continue);
        let (span, text) = self.source.bump();
        let kind = keyword(&text).unwrap_or(K![ident]);
        Ok(Token { span, text, kind })
    }
}

fn keyword(text: &str) -> Option<TokenKind> {
    Some(match text {
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

    use std::fs;

    #[test]
    fn lexer() {
        insta::glob!("snapshot_inputs/lexer/*.txt", |path| {
            let input = fs::read_to_string(path).unwrap();
            let suffix = path.file_stem().unwrap().to_str().unwrap();
            insta::with_settings!({snapshot_path => "snapshots/lexer", snapshot_suffix => suffix}, {
                insta::assert_debug_snapshot!(lex(&input))
            })
        })
    }

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
        assert!(Lexer::new("\t\t\n              \t\n")
            .lex_until_eof()
            .is_empty());
    }
}
