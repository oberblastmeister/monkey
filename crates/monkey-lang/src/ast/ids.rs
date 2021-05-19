use std::fmt;

use crate::{ast, ParseResult, Parser};

use la_arena::Arena as LaArena;
use la_arena::Idx;

#[derive(Default)]
pub struct Arena {
    pub stmts: LaArena<ast::Stmt>,
    pub exprs: LaArena<ast::Expr>,
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AstArena")
    }
}

impl Arena {}

macro_rules! impl_id {
    { $( $type:ident ),* $(,)? } => {
        $( impl_id! { @single $type } )*
    };
    (@single $type:ident) => {
        paste::paste! {
            impl_id!{ @impl ast::$type, [<$type Id>], [<$type:snake s>], [<alloc_ $type:snake>]  }
        }
    };
    (@impl $ty:ty, $id:tt, $field_multi:ident, $alloc_fn:ident) => {
        pub type $id = Idx<$ty>;

        impl std::ops::Index<$id> for Arena {
            type Output = $ty;

            fn index(&self, id: $id) -> &Self::Output {
                &self.$field_multi[id]
            }
        }

        impl std::ops::IndexMut<$id> for Arena {
            fn index_mut(&mut self, id: $id) -> &mut Self::Output {
                &mut self.$field_multi[id]
            }
        }

        impl crate::Parse for $id {
            fn parse(p: &mut Parser) -> ParseResult<Self> {
                let it = p.parse()?;
                Ok(p.$alloc_fn(it))
            }
        }
    };
}

impl_id! {
    Stmt,
    Expr,
}
