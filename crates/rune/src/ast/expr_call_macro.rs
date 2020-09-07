use crate::ast;
use crate::error::ParseError;
use crate::parser::Parser;
use crate::{token_stream::TokenStream, Parse};
use runestick::Span;

/// A function call `<expr>!(<args>)`.
#[derive(Debug, Clone)]
pub struct ExprCallMacro {
    /// The expression being called over.
    pub path: ast::Path,
    /// Bang operator `!`.
    pub bang: ast::Bang,
    /// Opening paren.
    pub open: ast::Token,
    /// The tokens provided to the macro.
    pub stream: TokenStream,
    /// Closing paren.
    pub close: ast::Token,
}

impl ExprCallMacro {
    /// Access the span of expression.
    pub fn span(&self) -> Span {
        self.path.span().join(self.close.span)
    }

    /// Parse with an expression.
    pub fn parse_with_path(parser: &mut Parser, path: ast::Path) -> Result<Self, ParseError> {
        let bang: ast::Bang = parser.parse()?;

        let mut level = 1;
        let open = parser.token_next()?;

        let delim = match open.kind {
            ast::Kind::Open(delim) => delim,
            kind => {
                return Err(ParseError::ExpectedMacroDelimiter {
                    span: open.span,
                    actual: kind,
                })
            }
        };

        let close;

        let mut stream = Vec::new();
        let end;

        loop {
            let token = parser.token_next()?;

            match token.kind {
                ast::Kind::Open(..) => level += 1,
                ast::Kind::Close(actual) => {
                    level -= 1;

                    if level == 0 {
                        if actual != delim {
                            return Err(ParseError::ExpectedMacroCloseDelimiter {
                                span: open.span,
                                actual: token.kind,
                                expected: ast::Kind::Close(delim),
                            });
                        }

                        end = Span::point(token.span.start);
                        close = token;
                        break;
                    }
                }
                _ => (),
            }

            stream.push(token);
        }

        Ok(Self {
            bang,
            path,
            open,
            stream: TokenStream::new(stream, end),
            close,
        })
    }
}

impl Parse for ExprCallMacro {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let path = parser.parse()?;
        Self::parse_with_path(parser, path)
    }
}