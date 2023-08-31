use crate::keyword::Keyword;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    kind: TokenKind,
    literal: &'a str,
    span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    /* Single Char */
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Equals,
    Comma,
    Period,
    OpeningBracket,
    ClosingBracket,
    OpeningBrace,
    ClosingBrace,
    /* Other */
    Identifier,
    Keyword(Keyword),
    /* Numbers */
    IntegerLiteral,
    FloatLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, literal: &'a str, span: Span) -> Self {
        Self {
            kind,
            literal,
            span,
        }
    }

    pub fn eof(position: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            literal: "",
            span: Span::zero_width(position),
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn literal(&self) -> &'a str {
        self.literal
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl TokenKind {
    pub fn from_keyword_or_identifier(literal: &str) -> Self {
        if let Ok(keyword) = literal.parse::<Keyword>() {
            Self::Keyword(keyword)
        } else {
            Self::Identifier
        }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn single(position: usize) -> Self {
        Self {
            start: position,
            end: position + 1,
        }
    }

    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn zero_width(position: usize) -> Self {
        Self {
            start: position,
            end: position,
        }
    }
}
