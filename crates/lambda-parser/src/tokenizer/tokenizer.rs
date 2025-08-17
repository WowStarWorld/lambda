use crate::tokenizer::token::{NumberRadix, PUNCTUATIONS, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct SrcInfo {
    pub filename: String,
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    pub src: Vec<char>,
    pub current_index: usize,
    pub src_info: SrcInfo,
}

// 基础部分
impl Tokenizer {
    pub fn new(src: &str, src_info: SrcInfo) -> Self {
        Self {
            src: src.chars().collect(),
            src_info,
            current_index: 0,
        }
    }

    pub fn next(&mut self) {
        self.current_index += 1;
    }

    fn has_next(&self) -> bool {
        self.current_index < self.src.len()
    }

    pub fn get(&mut self) -> Option<char> {
        if self.has_next() {
            let character = Some(self.src[self.current_index]);
            self.current_index += 1;
            character
        } else {
            None
        }
    }

    pub fn peek_n(&self, n: usize) -> Option<char> {
        if self.current_index + n < self.src.len() {
            Some(self.src[self.current_index + n])
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.current_index < self.src.len() {
            Some(self.src[self.current_index])
        } else {
            None
        }
    }
}

// 分析部分
impl Tokenizer {
    pub fn skip_comment(&mut self) {
        if self.peek() == Some('/') {
            if self.peek_n(1) == Some('/') {
                self.next(); // 跳过 '/'
                self.next(); // 跳过第二个 '/'
                while self.has_next() && self.peek() != Some('\n') {
                    self.next();
                }
                if self.has_next() && self.peek() == Some('\n') {
                    self.next(); // 跳过换行符
                }
            } else if self.peek_n(1) == Some('*') {
                self.next(); // 跳过 '/'
                self.next(); // 跳过 '*'
                while self.has_next() {
                    if self.peek() == Some('*') {
                        self.next(); // 跳过 '*'
                        if self.has_next() && self.peek() == Some('/') {
                            self.next(); // 跳过 '/'
                            break; // 结束
                        }
                    } else {
                        self.next();
                    }
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_comment();
        if !self.has_next() {
            Ok(Token {
                kind: TokenKind::End,
                start: self.current_index,
                end: self.current_index,
            })
        } else if self.is_whitespace() {
            Ok(self.get_whitespace())
        } else if self.is_string_start() {
            self.get_string(true)
        } else if self.is_octal_number() {
            self.get_octal_number()
        } else if self.is_hexadecimal_number() {
            self.get_hexadecimal_number()
        } else if self.is_binary_number() {
            self.get_binary_number()
        } else if self.is_decimal_number() {
            self.get_decimal_number()
        } else if self.is_punctuation() {
            Ok(self.get_punctuation())
        } else if self.is_identifier_start() {
            self.get_identifier()
        } else {
            let start = self.current_index;
            let next = self.get().unwrap();
            let end = self.current_index;
            Ok(Token {
                kind: TokenKind::Unknown(next),
                start,
                end,
            })
        }
    }

    fn is_whitespace(&self) -> bool {
        self.peek().map_or(false, |c| c.is_whitespace())
    }

    fn get_whitespace(&mut self) -> Token {
        let start = self.current_index;
        let mut whitespace = String::new();
        while self.is_whitespace() {
            whitespace.push(self.get().unwrap());
        }
        let end = self.current_index;
        Token {
            kind: TokenKind::Whitespace(whitespace),
            start,
            end,
        }
    }

    fn get_punctuation(&mut self) -> Token {
        let start = self.current_index;
        let punctuation = self.get().unwrap();
        let end = self.current_index;
        Token {
            kind: TokenKind::Punctuation(punctuation),
            start,
            end,
        }
    }

    fn get_identifier(&mut self) -> Result<Token, String> {
        if self.peek().map_or(false, |t| t == '`') {
            let result = self.get_string(false)?;
            if let TokenKind::StringLiteral { value, raw, .. } = result.kind {
                Ok(Token {
                    kind: TokenKind::Identifier { raw, value },
                    start: result.start,
                    end: result.end,
                })
            } else {
                Err("Invalid identifier with backticks".to_string())
            }
        } else {
            let start = self.current_index;
            let mut identifier = String::new();
            identifier.push(self.get().unwrap());
            while self.is_identifier_part() {
                identifier.push(self.get().unwrap());
            }
            Ok(Token {
                kind: TokenKind::Identifier {
                    raw: identifier.to_string(),
                    value: identifier.to_string(),
                },
                start,
                end: self.current_index,
            })
        }
    }

    fn is_octal_number(&self) -> bool {
        self.peek().map_or(false, |c| c == '0')
            && self.peek_n(1).map_or(false, |c| c == 'o' || c == 'O')
    }

    fn is_hexadecimal_number(&self) -> bool {
        self.peek().map_or(false, |c| c == '0')
            && self.peek_n(1).map_or(false, |c| c == 'x' || c == 'X')
    }

    fn is_binary_number(&self) -> bool {
        self.peek().map_or(false, |c| c == '0')
            && self.peek_n(1).map_or(false, |c| c == 'b' || c == 'B')
    }

    fn is_decimal_number(&self) -> bool {
        self.peek().map_or(false, |c| c.is_digit(10))
            || (self.peek() == Some('.') && self.peek_n(1).map_or(false, |c| c.is_digit(10)))
    }

    fn get_octal_number(&mut self) -> Result<Token, String> {
        let start = self.current_index;
        let mut raw = String::new();
        raw.push(self.get().unwrap()); // 跳过 '0'
        raw.push(self.get().unwrap()); // 跳过 'o' 或 'O'
        while self.peek().map_or(false, |c| c.is_digit(8)) {
            raw.push(self.get().unwrap());
        }
        if raw.len() <= 2 {
            return Err("Invalid octal number".to_string());
        }
        Ok(Token {
            kind: TokenKind::NumberLiteral {
                raw: raw.clone(),
                radix: NumberRadix::Octal,
            },
            start,
            end: self.current_index,
        })
    }

    fn get_hexadecimal_number(&mut self) -> Result<Token, String> {
        let start = self.current_index;
        let mut raw = String::new();
        raw.push(self.get().unwrap()); // 跳过 '0'
        raw.push(self.get().unwrap()); // 跳过 'x' 或 'X'
        let mut integer = String::new();
        while self.peek().map_or(false, |c| c.is_digit(16)) {
            let ch = self.get().unwrap();
            raw.push(ch);
            integer.push(ch);
        }
        if raw.len() <= 2 {
            return Err("Invalid hexadecimal number".to_string());
        }
        Ok(Token {
            kind: TokenKind::NumberLiteral {
                raw: raw.clone(),
                radix: NumberRadix::Hexadecimal,
            },
            start,
            end: self.current_index,
        })
    }

    fn get_binary_number(&mut self) -> Result<Token, String> {
        let start = self.current_index;
        let mut raw = String::new();
        raw.push(self.get().unwrap()); // 跳过 '0'
        raw.push(self.get().unwrap()); // 跳过 'b' 或 'B'
        while self.peek().map_or(false, |c| c == '0' || c == '1') {
            raw.push(self.get().unwrap());
        }
        if raw.len() <= 2 {
            return Err("Invalid binary number".to_string());
        }
        Ok(Token {
            kind: TokenKind::NumberLiteral {
                raw: raw.clone(),
                radix: NumberRadix::Binary,
            },
            start,
            end: self.current_index,
        })
    }

    fn get_decimal_number(&mut self) -> Result<Token, String> {
        let start = self.current_index;
        let mut raw = String::new();
        let mut integer = String::new();
        let mut fraction = String::new();
        let mut exponent = String::new();
        // 整数部分或小数点前
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                let ch = self.get().unwrap();
                raw.push(ch);
                integer.push(ch);
            } else if c == '_' {
                let ch = self.get().unwrap();
                raw.push(ch);
                integer.push(ch);
            } else {
                break;
            }
        }
        // 小数点和小数部分
        if self.peek() == Some('.') {
            raw.push(self.get().unwrap()); // 跳过 '.'
            // 小数部分允许为空（如1.e1234），但如果下一个是数字或下划线则继续解析
            while let Some(c) = self.peek() {
                if c.is_digit(10) {
                    let ch = self.get().unwrap();
                    raw.push(ch);
                    fraction.push(ch);
                } else if c == '_' {
                    let ch = self.get().unwrap();
                    raw.push(ch);
                    fraction.push(ch);
                } else {
                    break;
                }
            }
        }
        // 科学计数法
        if let Some(e) = self.peek() {
            if e == 'e' || e == 'E' {
                let ch = self.get().unwrap();
                raw.push(ch); // 跳过 'e' 或 'E'
                exponent.push(ch);
                // 可选正负号
                if let Some(sign) = self.peek() {
                    if sign == '+' || sign == '-' {
                        let s = self.get().unwrap();
                        raw.push(s);
                        exponent.push(s);
                    }
                }
                let mut exp_digits = false;
                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        exp_digits = true;
                        let d = self.get().unwrap();
                        raw.push(d);
                        exponent.push(d);
                    } else if c == '_' {
                        let d = self.get().unwrap();
                        raw.push(d);
                        exponent.push(d);
                    } else {
                        break;
                    }
                }
                if !exp_digits {
                    return Err("Invalid exponent in decimal number".to_string());
                }
            }
        }
        // 允许如1.e1234，整数部分和指数部分必须至少有一位数字
        if integer.is_empty() && fraction.is_empty() {
            return Err("Invalid decimal number: missing integer and fraction".to_string());
        }
        Ok(Token {
            kind: TokenKind::NumberLiteral {
                raw: raw.clone(),
                radix: NumberRadix::Decimal {
                    integer: if integer.is_empty() {
                        None
                    } else {
                        Some(integer)
                    },
                    fraction: if fraction.is_empty() {
                        None
                    } else {
                        Some(fraction)
                    },
                    exponent: if exponent.is_empty() {
                        None
                    } else {
                        Some(exponent)
                    },
                },
            },
            start,
            end: self.current_index,
        })
    }

    fn get_string(&mut self, new_line: bool) -> Result<Token, String> {
        let start = self.current_index;
        let quote = self.get().unwrap(); // 跳过引号
        let mut content = String::new();
        let mut raw = String::new();
        raw.push(quote);
        while let Some(c) = self.peek() {
            if c == quote {
                raw.push(self.get().unwrap()); // 跳过引号
                return Ok(Token {
                    kind: TokenKind::StringLiteral {
                        value: content.clone(),
                        raw: raw.clone(),
                    },
                    start,
                    end: self.current_index,
                });
            } else if c == '\\' {
                raw.push(self.get().unwrap()); // 跳过 '\\'
                let esc = match self.get() {
                    Some(e) => {
                        raw.push(e);
                        e
                    }
                    None => return Err("Unterminated escape".to_string()),
                };
                match esc {
                    '\\' | '/' => content.push(esc),
                    '"' | '\'' => content.push(esc),
                    'b' => content.push('\u{0008}'),
                    'f' => content.push('\u{000C}'),
                    'n' => content.push('\n'),
                    'r' => content.push('\r'),
                    't' => content.push('\t'),
                    'v' => content.push('\u{000B}'),
                    '0' => content.push('\0'),
                    'x' => {
                        let mut hex = String::new();
                        for _ in 0..2 {
                            match self.get() {
                                Some(h) => {
                                    raw.push(h);
                                    hex.push(h);
                                }
                                None => return Err("Invalid \\x escape".to_string()),
                            }
                        }
                        match u8::from_str_radix(&hex, 16) {
                            Ok(val) => content.push(val as char),
                            Err(_) => return Err("Invalid hex in \\x escape".to_string()),
                        }
                    }
                    'u' => {
                        let mut hex = String::new();
                        for _ in 0..4 {
                            match self.get() {
                                Some(h) => {
                                    raw.push(h);
                                    hex.push(h);
                                }
                                None => return Err("Invalid \\u escape".to_string()),
                            }
                        }
                        match u16::from_str_radix(&hex, 16) {
                            Ok(val) => match char::from_u32(val as u32) {
                                Some(ch) => content.push(ch),
                                None => return Err("Invalid unicode in \\u escape".to_string()),
                            },
                            Err(_) => return Err("Invalid hex in \\u escape".to_string()),
                        }
                    }
                    _ => return Err(format!("Invalid escape: \\{}", esc)),
                }
            } else if (c == '\n' || c == '\r') && !new_line {
                return Err("String literal cannot contain line breaks".to_string());
            } else {
                content.push(self.get().unwrap());
                raw.push(c);
            }
        }
        Err(format!("Unterminated string literal: expected {}", quote))
    }

    fn is_string_start(&self) -> bool {
        self.peek().map_or(false, |c| c == '\'' || c == '"')
    }

    fn is_identifier_start(&self) -> bool {
        self.peek()
            .map_or(false, |c| c.is_alphabetic() || c == '_' || c == '`')
    }

    fn is_identifier_part(&self) -> bool {
        self.peek()
            .map_or(false, |c| c.is_alphanumeric() || c == '_')
    }

    fn is_punctuation(&self) -> bool {
        self.peek().map_or(false, |c| PUNCTUATIONS.contains(&c))
    }

    pub fn collect(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let result = self.next_token();
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let token = result?;
            match &token.kind {
                TokenKind::End => break,
                TokenKind::Unknown(c) => {
                    return Err(format!("Invalid character: '{}' at {}", c, token.start));
                }
                _ => tokens.push(token),
            }
        }
        Ok(tokens)
    }
}
