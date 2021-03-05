use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtIf {
    if_token: T![if],
    condition: Condition,
    block: ast::Block,
    next: Option<IfBranch>,
}

impl Parse for StmtIf {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(StmtIf {
            if_token: p.parse()?,
            condition: p.parse()?,
            block: p.parse()?,
            next: p.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfBranch {
    ElseIf(Box<ElseIfBranch>),
    Else(ElseBranch),
}

impl Parse for IfBranch {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        assert_eq!(p.nth(0).unwrap(), K![else]);

        Ok(match p.nth(1)? {
            K![else] => IfBranch::ElseIf(p.parse()?),
            _ => IfBranch::Else(p.parse()?),
        })
    }
}

impl Peek for IfBranch {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![else])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfBranch {
    else_token: T![else],
    if_token: T![if],
    condition: Condition,
    block: ast::Block,
    next: Option<IfBranch>,
}

impl Parse for ElseIfBranch {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(ElseIfBranch {
            else_token: p.parse()?,
            if_token: p.parse()?,
            condition: p.parse()?,
            block: p.parse()?,
            next: p.parse()?,
        })
    }
}

impl Peek for ElseIfBranch {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![else]) && matches!(p.nth(1), K![if])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch {
    else_token: T![else],
    block: ast::Block,
}

impl Parse for ElseBranch {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(ElseBranch {
            else_token: p.parse()?,
            block: p.parse()?,
        })
    }
}

impl Peek for ElseBranch {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![else]) && !matches!(p.nth(1), K![if])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    open_paren: T!['('],
    expr: ast::Expr,
    close_paren: T![')'],
}

impl Parse for Condition {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(Condition {
            open_paren: p.parse()?,
            expr: p.parse()?,
            close_paren: p.parse()?,
        })
    }
}
