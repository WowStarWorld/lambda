pub const PUNCTUATIONS: &[char] = &[
    '(', ')', '{', '}', '[', ']', ';', ':', ',', '.', '+', '-', '*', '/', '%', '=', '&', '|', '!', '<', '>', '?', '^', '~'
];

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NumberRadix {
    Decimal { integer: Option<String>, fraction: Option<String>, exponent: Option<String> },
    Octal,
    Binary,
    Hexadecimal,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    NumberLiteral { raw: String, radix: NumberRadix },
    StringLiteral { value: String, raw: String },
    TemplateString { raw: String, text: String },
    Punctuation(char),
    Whitespace(String),
    Unknown(char),
    End
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn get_raw(&self) -> String {
        match &self.kind {
            TokenKind::Identifier(s) => s.clone(),
            TokenKind::NumberLiteral { raw, .. } => raw.clone(),
            TokenKind::StringLiteral { raw, .. } => raw.clone(),
            TokenKind::TemplateString { raw, .. } => raw.clone(),
            TokenKind::Punctuation(c) => c.to_string(),
            TokenKind::Whitespace(s) => s.to_string(),
            TokenKind::Unknown(c) => c.to_string(),
            TokenKind::End => "".to_string(),
        }
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self.kind, TokenKind::Whitespace(_))
    }

    pub fn is_punctuation_of(&self, punctuation: char) -> bool {
        matches!(self.kind, TokenKind::Punctuation(value) if value == punctuation)
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self.kind, TokenKind::Punctuation(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }

    pub fn is_identifier_of(&self, identifier: &str) -> bool {
        matches!(&self.kind, TokenKind::Identifier(value) if value == identifier)
    }
}
