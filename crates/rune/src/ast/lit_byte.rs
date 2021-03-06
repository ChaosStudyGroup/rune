use crate::ast;
use crate::error::ParseError;
use crate::parser::Parser;
use crate::traits::{Parse, Resolve};
use runestick::{Source, Span};

/// A byte literal.
#[derive(Debug, Clone)]
pub struct LitByte {
    /// The token corresponding to the literal.
    pub token: ast::Token,
}

impl LitByte {
    /// Access the span of the expression.
    pub fn span(&self) -> Span {
        self.token.span
    }
}

/// Parse a byte literal.
///
/// # Examples
///
/// ```rust
/// use rune::{parse_all, ast};
///
/// parse_all::<ast::LitByte>("b'a'").unwrap();
/// parse_all::<ast::LitByte>("b'\\0'").unwrap();
/// parse_all::<ast::LitByte>("b'\\n'").unwrap();
/// parse_all::<ast::LitByte>("b'\\r'").unwrap();
/// parse_all::<ast::LitByte>("b'\\\\''").unwrap();
/// ```
impl Parse for LitByte {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParseError> {
        let token = parser.token_next()?;

        Ok(match token.kind {
            ast::Kind::LitByte => LitByte { token },
            _ => {
                return Err(ParseError::ExpectedByte {
                    actual: token.kind,
                    span: token.span,
                })
            }
        })
    }
}

impl<'a> Resolve<'a> for LitByte {
    type Output = u8;

    fn resolve(&self, source: &'a Source) -> Result<u8, ParseError> {
        let span = self.token.span;

        let string = source
            .source(span.trim_start(2).trim_end(1))
            .ok_or_else(|| ParseError::BadSlice { span })?;

        let mut it = string
            .char_indices()
            .map(|(n, c)| (span.start + n, c))
            .peekable();

        let (n, c) = match it.next() {
            Some(c) => c,
            None => {
                return Err(ParseError::BadByteLiteral { span });
            }
        };

        let c = match c {
            '\\' => ast::utils::parse_byte_escape(span.with_start(n), &mut it)?,
            c if c.is_ascii() && !c.is_control() => c as u8,
            _ => {
                return Err(ParseError::BadByteLiteral { span });
            }
        };

        // Too many characters in literal.
        if it.next().is_some() {
            return Err(ParseError::BadByteLiteral { span });
        }

        Ok(c)
    }
}
