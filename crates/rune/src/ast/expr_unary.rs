use crate::ast;
use crate::ast::expr::{EagerBrace, ExprChain};
use crate::error::ParseError;
use crate::parser::Parser;
use crate::traits::Parse;
use runestick::Span;
use std::fmt;

/// A unary expression.
#[derive(Debug, Clone)]
pub struct ExprUnary {
    /// The operation to apply.
    pub op: UnaryOp,
    /// Token associated with operator.
    pub token: ast::Token,
    /// The expression of the operation.
    pub expr: Box<ast::Expr>,
}

impl ExprUnary {
    /// Access the span of the expression.
    pub fn span(&self) -> Span {
        self.token.span.join(self.expr.span())
    }
}

/// Parse a unary statement.
///
/// # Examples
///
/// ```rust
/// use rune::{parse_all, ast};
///
/// parse_all::<ast::ExprUnary>("!0").unwrap();
/// parse_all::<ast::ExprUnary>("*foo").unwrap();
/// parse_all::<ast::ExprUnary>("&foo").unwrap();
/// ```
impl Parse for ExprUnary {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.token_next()?;
        let op = UnaryOp::from_token(token)?;

        Ok(Self {
            op,
            token,
            expr: Box::new(ast::Expr::parse_primary(
                parser,
                EagerBrace(true),
                ExprChain(true),
            )?),
        })
    }
}

/// A unary operation.
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    /// Not `!<thing>`.
    Not,
    /// Reference `&<thing>`.
    BorrowRef,
    /// Dereference `*<thing>`.
    Deref,
}

impl UnaryOp {
    /// Convert a unary operator from a token.
    pub fn from_token(token: ast::Token) -> Result<Self, ParseError> {
        Ok(match token.kind {
            ast::Kind::Bang => Self::Not,
            ast::Kind::Amp => Self::BorrowRef,
            ast::Kind::Star => Self::Deref,
            actual => {
                return Err(ParseError::ExpectedUnaryOperator {
                    span: token.span,
                    actual,
                })
            }
        })
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Not => write!(fmt, "!")?,
            Self::BorrowRef => write!(fmt, "&")?,
            Self::Deref => write!(fmt, "*")?,
        }

        Ok(())
    }
}
