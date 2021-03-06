use crate::ast::Token;
use crate::MacroContext;
use runestick::Span;
use std::slice;

/// A token stream.
#[derive(Debug, Clone)]
pub struct TokenStream {
    stream: Vec<Token>,
    end: Span,
}

impl TokenStream {
    /// Construct a new token stream with the specified end span.
    pub fn new(stream: Vec<Token>, end: Span) -> Self {
        Self { stream, end }
    }

    /// Push the current token to the stream.
    pub fn push(&mut self, token: Token) {
        self.stream.push(token);
    }

    /// Extend the token stream with another iterator.
    pub fn extend<I>(&mut self, tokens: I)
    where
        I: IntoIterator,
        Token: From<I::Item>,
    {
        self.stream.extend(tokens.into_iter().map(Token::from));
    }

    /// Get the end span of the token stream.
    pub fn end(&self) -> Span {
        self.end
    }

    /// Create an iterator over the token stream.
    pub(crate) fn iter(&self) -> TokenStreamIter<'_> {
        TokenStreamIter {
            iter: self.stream.iter(),
            end: self.end,
        }
    }
}

/// A token stream iterator.
#[derive(Debug)]
pub struct TokenStreamIter<'a> {
    iter: slice::Iter<'a, Token>,
    end: Span,
}

impl TokenStreamIter<'_> {
    /// Get the end point of the token stream iterator.
    pub(crate) fn end(&self) -> Span {
        self.end
    }
}

impl Iterator for TokenStreamIter<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}

impl<'a> IntoIterator for &'a TokenStream {
    type Item = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.stream.iter()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.stream.into_iter()
    }
}

/// Trait for things that can be turned into tokens.
pub trait IntoTokens {
    /// Turn the current item into tokens.
    fn into_tokens(self, context: &mut MacroContext, stream: &mut TokenStream);
}

impl<T> IntoTokens for &T
where
    T: Copy + IntoTokens,
{
    fn into_tokens(self, context: &mut MacroContext, stream: &mut TokenStream) {
        IntoTokens::into_tokens(*self, context, stream);
    }
}

impl<T> IntoTokens for Option<T>
where
    T: IntoTokens,
{
    fn into_tokens(self, context: &mut MacroContext, stream: &mut TokenStream) {
        if let Some(this) = self {
            crate::IntoTokens::into_tokens(this, context, stream);
        }
    }
}
