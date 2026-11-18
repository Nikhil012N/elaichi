#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Static,
    Function,
    Const,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Struct,
    Return,
    As,

    // Terminal I/O
    Terminal,
    Print,
    Println,
    Error,
    Warn,
    Clear,

    // Types
    TypeI64,
    TypeF64,
    TypeBool,
    TypeString,

    // Identifiers and Literals
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    BoolLit(bool),

    // Symbols & Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent, // %
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd, // &&
    OrOr,   // ||
    Bang,   // !
    Dot,    // .
    DotDot, // ..
    Colon,
    Semicolon,
    Comma,
    Arrow,    // ->
    FatArrow, // =>
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Eof,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else if c == '/' {
                let mut clone = self.chars.clone();
                clone.next();
                if let Some('/') = clone.peek() {
                    self.chars.next(); // /
                    self.chars.next(); // /
                    while let Some(ch) = self.chars.next() {
                        if ch == '\n' {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        let ch = match self.chars.next() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        match ch {
            '+' => Ok(Token::Plus),
            '-' => {
                if let Some(&'>') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Minus)
                }
            }
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            '%' => Ok(Token::Percent),
            ':' => Ok(Token::Colon),
            ';' => Ok(Token::Semicolon),
            ',' => Ok(Token::Comma),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            '.' => {
                if let Some(&'.') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::DotDot)
                } else {
                    Ok(Token::Dot)
                }
            }
            '&' => {
                if let Some(&'&') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::AndAnd)
                } else {
                    Err("Expected '&&'".to_string())
                }
            }
            '|' => {
                if let Some(&'|') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::OrOr)
                } else {
                    Err("Expected '||'".to_string())
                }
            }
            '=' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::EqualEqual)
                } else if let Some(&'>') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::FatArrow)
                } else {
                    Ok(Token::Equal)
                }
            }
            '!' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::BangEqual)
                } else {
                    Ok(Token::Bang)
                }
            }
            '<' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::LessEqual)
                } else {
                    Ok(Token::Less)
                }
            }
            '>' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Ok(Token::GreaterEqual)
                } else {
                    Ok(Token::Greater)
                }
            }
            '"' => {
                let mut s = String::new();
                while let Some(c) = self.chars.next() {
                    if c == '"' {
                        return Ok(Token::StringLit(s));
                    }
                    if c == '\\' {
                        match self.chars.next() {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some('"') => s.push('"'),
                            Some('\\') => s.push('\\'),
                            Some(other) => s.push(other),
                            None => return Err("Unterminated string escape".to_string()),
                        }
                    } else {
                        s.push(c);
                    }
                }
                Err("Unterminated string literal".to_string())
            }
            _ if ch.is_ascii_digit() => {
                let mut num_str = ch.to_string();
                let mut is_float = false;

                while let Some(&next_ch) = self.chars.peek() {
                    if next_ch.is_ascii_digit() {
                        num_str.push(self.chars.next().unwrap());
                    } else if next_ch == '.' && !is_float {
                        let mut lookahead = self.chars.clone();
                        lookahead.next();
                        if let Some('.') = lookahead.peek() {
                            break;
                        }
                        is_float = true;
                        num_str.push(self.chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if is_float {
                    let val = num_str.parse::<f64>().map_err(|e| e.to_string())?;
                    Ok(Token::FloatLit(val))
                } else {
                    let val = num_str.parse::<i64>().map_err(|e| e.to_string())?;
                    Ok(Token::IntLit(val))
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut ident = ch.to_string();
                while let Some(&next_ch) = self.chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        ident.push(self.chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "static" => Ok(Token::Static),
                    "function" | "fn" => Ok(Token::Function),
                    "let" => Ok(Token::Let),
                    "const" => Ok(Token::Const),
                    "if" => Ok(Token::If),
                    "else" => Ok(Token::Else),
                    "while" => Ok(Token::While),
                    "for" => Ok(Token::For),
                    "in" => Ok(Token::In),
                    "break" => Ok(Token::Break),
                    "continue" => Ok(Token::Continue),
                    "struct" => Ok(Token::Struct),
                    "return" => Ok(Token::Return),
                    "as" => Ok(Token::As),
                    "terminal" => Ok(Token::Terminal),
                    "print" => Ok(Token::Print),
                    "println" => Ok(Token::Println),
                    "error" => Ok(Token::Error),
                    "warn" => Ok(Token::Warn),
                    "clear" => Ok(Token::Clear),
                    "true" => Ok(Token::BoolLit(true)),
                    "false" => Ok(Token::BoolLit(false)),
                    "i64" => Ok(Token::TypeI64),
                    "f64" => Ok(Token::TypeF64),
                    "bool" => Ok(Token::TypeBool),
                    "string" => Ok(Token::TypeString),
                    _ => Ok(Token::Ident(ident)),
                }
            }
            _ => Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }
}
