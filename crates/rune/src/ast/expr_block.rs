use crate::ast;
use crate::error::ParseError;
use crate::parser::Parser;
use crate::traits::Parse;
use runestick::Span;

/// A block of expressions.
#[derive(Debug, Clone)]
pub struct ExprBlock {
    /// If the block is async or not.
    pub async_: Option<ast::Async>,
    /// The close brace.
    pub open: ast::OpenBrace,
    /// Expressions in the block.
    pub exprs: Vec<(ast::Expr, Option<ast::SemiColon>)>,
    /// Test if the expression is trailing.
    pub trailing_expr: Option<Box<ast::Expr>>,
    /// The close brace.
    pub close: ast::CloseBrace,
}

impl ExprBlock {
    /// Get the span of the block.
    pub fn span(&self) -> Span {
        if let Some(async_) = self.async_ {
            async_.span().join(self.close.span())
        } else {
            self.open.span().join(self.close.span())
        }
    }

    /// Test if the block is empty.
    pub fn produces_nothing(&self) -> bool {
        match &self.trailing_expr {
            Some(trailing) => trailing.produces_nothing(),
            None => true,
        }
    }

    /// ExprBlock is constant if a trailing expression exists and is all literal.
    pub fn is_const(&self) -> bool {
        match &self.trailing_expr {
            Some(trailing) => trailing.is_const(),
            None => false,
        }
    }
}

/// Parse implementation for a block.
///
/// # Examples
///
/// ```rust
/// use rune::{parse_all, ast};
///
/// let block = parse_all::<ast::ExprBlock>("async {}").unwrap();
/// assert_eq!(block.exprs.len(), 0);
/// assert!(block.trailing_expr.is_none());
/// assert!(block.async_.is_some());
///
/// let block = parse_all::<ast::ExprBlock>("{}").unwrap();
/// assert_eq!(block.exprs.len(), 0);
/// assert!(block.trailing_expr.is_none());
///
/// let block = parse_all::<ast::ExprBlock>("{ foo }").unwrap();
/// assert_eq!(block.exprs.len(), 0);
/// assert!(block.trailing_expr.is_some());
///
/// let block = parse_all::<ast::ExprBlock>("{ foo; }").unwrap();
/// assert_eq!(block.exprs.len(), 1);
/// assert!(block.trailing_expr.is_none());
///
/// let block = parse_all::<ast::ExprBlock>(r#"
///     {
///         let foo = 42;
///         let bar = "string";
///         baz
///     }
/// "#).unwrap();
/// assert!(block.async_.is_none());
/// assert_eq!(block.exprs.len(), 2);
/// assert!(block.trailing_expr.is_some());
/// ```
impl Parse for ExprBlock {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParseError> {
        let mut exprs = Vec::new();

        let async_ = parser.parse()?;
        let open = parser.parse()?;
        let mut trailing_expr = None;

        while !parser.peek::<ast::CloseBrace>()? {
            let (expr, semi_colon) = if parser.peek::<ast::Decl>()? {
                let decl: ast::Decl = parser.parse()?;
                let semi_colon = decl.needs_semi_colon() || parser.peek::<ast::SemiColon>()?;
                (ast::Expr::Decl(decl), semi_colon)
            } else {
                let expr: ast::Expr = parser.parse()?;
                (expr, parser.peek::<ast::SemiColon>()?)
            };

            let semi_colon = if semi_colon {
                Some(parser.parse()?)
            } else {
                None
            };

            if parser.peek::<ast::CloseBrace>()? {
                if semi_colon.is_none() {
                    trailing_expr = Some(Box::new(expr));
                } else {
                    exprs.push((expr, semi_colon));
                }

                break;
            }

            exprs.push((expr, semi_colon));
        }

        let close = parser.parse()?;

        Ok(ExprBlock {
            async_,
            open,
            exprs,
            trailing_expr,
            close,
        })
    }
}
