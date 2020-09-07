use crate::ast;
use crate::{Parse, ParseError, Parser, Peek, Resolve};
use runestick::{Source, Span};

/// An identifier, like `foo` or `Hello`.".
#[derive(Debug, Clone, Copy)]
pub struct Ident {
    /// Associated token.
    pub token: ast::Token,
}

impl Ident {
    /// Access the span of the identifier.
    pub fn span(&self) -> Span {
        self.token.span
    }
}

impl Parse for Ident {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParseError> {
        let token = parser.token_next()?;

        match token.kind {
            ast::Kind::Ident => Ok(Self { token }),
            _ => Err(ParseError::TokenMismatch {
                expected: ast::Kind::Ident,
                actual: token.kind,
                span: token.span,
            }),
        }
    }
}

impl Peek for Ident {
    fn peek(p1: Option<ast::Token>, _: Option<ast::Token>) -> bool {
        match p1 {
            Some(p1) => matches!(p1.kind, ast::Kind::Ident),
            _ => false,
        }
    }
}

impl<'a> Resolve<'a> for Ident {
    type Output = &'a str;

    fn resolve(&self, source: &'a Source) -> Result<&'a str, ParseError> {
        let span = self.token.span;

        source
            .source(span)
            .ok_or_else(|| ParseError::BadSlice { span })
    }
}

impl crate::IntoTokens for Ident {
    fn into_tokens(self, _: &mut crate::MacroContext, stream: &mut crate::TokenStream) {
        stream.push(self.token);
    }
}
