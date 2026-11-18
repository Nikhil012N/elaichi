#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    Import,

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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.char_indices().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((_, c)) = self.chars.next() {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' {
                let mut clone = self.chars.clone();
                clone.next();
                if let Some((_, '/')) = clone.peek() {
                    self.advance(); // /
                    self.advance(); // /
                    while let Some(ch) = self.advance() {
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

        let token_line = self.line;
        let token_col = self.col;

        let ch = match self.advance() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::Eof,
                    line: token_line,
                    col: token_col,
                })
            }
        };

        let kind = match ch {
            '+' => TokenKind::Plus,
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '.' => {
                if self.peek() == Some('.') {
                    self.advance();
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }
            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    TokenKind::AndAnd
                } else {
                    return Err(format!(
                        "Lexer Error at {}:{}: Expected '&&'",
                        token_line, token_col
                    ));
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    TokenKind::OrOr
                } else {
                    return Err(format!(
                        "Lexer Error at {}:{}: Expected '||'",
                        token_line, token_col
                    ));
                }
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::EqualEqual
                } else if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '"' => {
                let mut s = String::new();
                let mut terminated = false;
                while let Some(c) = self.advance() {
                    if c == '"' {
                        terminated = true;
                        break;
                    }
                    if c == '\\' {
                        match self.advance() {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some('r') => s.push('\r'),
                            Some('0') => s.push('\0'),
                            Some('"') => s.push('"'),
                            Some('\\') => s.push('\\'),
                            Some(other) => s.push(other),
                            None => {
                                return Err(format!(
                                    "Lexer Error at {}:{}: Unterminated string escape sequence",
                                    token_line, token_col
                                ))
                            }
                        }
                    } else {
                        s.push(c);
                    }
                }
                if !terminated {
                    return Err(format!(
                        "Lexer Error at {}:{}: Unterminated string literal",
                        token_line, token_col
                    ));
                }
                TokenKind::StringLit(s)
            }
            _ if ch.is_ascii_digit() => {
                let mut num_str = ch.to_string();
                let mut is_float = false;

                while let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        num_str.push(self.advance().unwrap());
                    } else if next_ch == '.' && !is_float {
                        let mut lookahead = self.chars.clone();
                        lookahead.next();
                        if let Some((_, '.')) = lookahead.peek() {
                            break;
                        }
                        is_float = true;
                        num_str.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }

                if is_float {
                    let val = num_str.parse::<f64>().map_err(|e| {
                        format!("Lexer Error at {}:{}: {}", token_line, token_col, e)
                    })?;
                    TokenKind::FloatLit(val)
                } else {
                    let val = num_str.parse::<i64>().map_err(|e| {
                        format!("Lexer Error at {}:{}: {}", token_line, token_col, e)
                    })?;
                    TokenKind::IntLit(val)
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut ident = ch.to_string();
                while let Some(next_ch) = self.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        ident.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "static" => TokenKind::Static,
                    "function" | "fn" => TokenKind::Function,
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "for" => TokenKind::For,
                    "in" => TokenKind::In,
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "struct" => TokenKind::Struct,
                    "return" => TokenKind::Return,
                    "as" => TokenKind::As,
                    "import" => TokenKind::Import,
                    "terminal" => TokenKind::Terminal,
                    "print" => TokenKind::Print,
                    "println" => TokenKind::Println,
                    "error" => TokenKind::Error,
                    "warn" => TokenKind::Warn,
                    "clear" => TokenKind::Clear,
                    "true" => TokenKind::BoolLit(true),
                    "false" => TokenKind::BoolLit(false),
                    "i64" => TokenKind::TypeI64,
                    "f64" => TokenKind::TypeF64,
                    "bool" => TokenKind::TypeBool,
                    "string" => TokenKind::TypeString,
                    _ => TokenKind::Ident(ident),
                }
            }
            _ => {
                return Err(format!(
                    "Lexer Error at {}:{}: Unexpected character '{}'",
                    token_line, token_col, ch
                ))
            }
        };

        Ok(Token {
            kind,
            line: token_line,
            col: token_col,
        })
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok.kind == TokenKind::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }
}
