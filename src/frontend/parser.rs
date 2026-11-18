use super::ast::*;
use super::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        } else {
            Token::Eof
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let tok = self.advance();
        if tok == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, tok))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut structs = Vec::new();
        let mut functions = Vec::new();

        while *self.peek() != Token::Eof {
            if *self.peek() == Token::Struct {
                structs.push(self.parse_struct()?);
            } else {
                functions.push(self.parse_function()?);
            }
        }
        Ok(Program { structs, functions })
    }

    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.expect(Token::Struct)?;
        let name = match self.advance() {
            Token::Ident(s) => s,
            other => return Err(format!("Expected struct name, found {:?}", other)),
        };

        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            let field_name = match self.advance() {
                Token::Ident(s) => s,
                other => return Err(format!("Expected field name, found {:?}", other)),
            };
            self.expect(Token::Colon)?;
            let ty = self.parse_type()?;
            fields.push(Param {
                name: field_name,
                ty,
            });

            if *self.peek() == Token::Comma || *self.peek() == Token::Semicolon {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(Token::RBrace)?;

        Ok(StructDef { name, fields })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let mut is_static = false;
        if *self.peek() == Token::Static {
            self.advance();
            is_static = true;
        }

        self.expect(Token::Function)?;

        let name = match self.advance() {
            Token::Ident(s) => s,
            other => return Err(format!("Expected function name, found {:?}", other)),
        };

        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        while *self.peek() != Token::RParen && *self.peek() != Token::Eof {
            let param_name = match self.advance() {
                Token::Ident(s) => s,
                other => return Err(format!("Expected parameter name, found {:?}", other)),
            };

            let ty = if *self.peek() == Token::Colon {
                self.advance();
                self.parse_type()?
            } else if is_static {
                return Err(format!(
                    "Static function '{}' parameter '{}' must have an explicit type",
                    name, param_name
                ));
            } else {
                ElaichiType::Dynamic
            };

            params.push(Param {
                name: param_name,
                ty,
            });

            if *self.peek() == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(Token::RParen)?;

        let return_ty = if *self.peek() == Token::Colon || *self.peek() == Token::Arrow {
            self.advance();
            self.parse_type()?
        } else if is_static {
            ElaichiType::I64
        } else {
            ElaichiType::Dynamic
        };

        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Function {
            name,
            is_static,
            params,
            return_ty,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<ElaichiType, String> {
        match self.advance() {
            Token::TypeI64 => Ok(ElaichiType::I64),
            Token::TypeF64 => Ok(ElaichiType::F64),
            Token::TypeBool => Ok(ElaichiType::Bool),
            Token::TypeString => Ok(ElaichiType::String),
            Token::Ident(ref s) if s == "dyn" || s == "any" => Ok(ElaichiType::Dynamic),
            Token::Ident(s) => Ok(ElaichiType::Custom(s)),
            other => Err(format!("Expected type, found {:?}", other)),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Const | Token::Let => {
                let is_const = *self.peek() == Token::Const;
                self.advance();

                let name = match self.advance() {
                    Token::Ident(s) => s,
                    other => return Err(format!("Expected variable name, found {:?}", other)),
                };

                if *self.peek() == Token::Colon {
                    self.advance();
                    let ty = self.parse_type()?;
                    self.expect(Token::Equal)?;
                    let init = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::LetStatic {
                        name,
                        is_mut: !is_const,
                        ty,
                        init,
                    })
                } else {
                    self.expect(Token::Equal)?;
                    let init = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::VarDynamic { name, init })
                }
            }
            Token::Terminal => {
                self.advance();
                self.expect(Token::Dot)?;
                let action = match self.advance() {
                    Token::Print => TerminalAction::Print,
                    Token::Println => TerminalAction::Println,
                    Token::Error => TerminalAction::Error,
                    Token::Warn => TerminalAction::Warn,
                    Token::Clear => TerminalAction::Clear,
                    other => return Err(format!("Expected terminal action, found {:?}", other)),
                };

                self.expect(Token::LParen)?;
                let expr = if action != TerminalAction::Clear && *self.peek() != Token::RParen {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::RParen)?;
                self.expect(Token::Semicolon)?;

                Ok(Stmt::TerminalCall { action, expr })
            }
            Token::Break => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Continue)
            }
            Token::Return => {
                self.advance();
                let expr = if *self.peek() != Token::Semicolon {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            Token::If => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;

                self.expect(Token::LBrace)?;
                let mut then_branch = Vec::new();
                while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                    then_branch.push(self.parse_stmt()?);
                }
                self.expect(Token::RBrace)?;

                let else_branch = if *self.peek() == Token::Else {
                    self.advance();
                    self.expect(Token::LBrace)?;
                    let mut eb = Vec::new();
                    while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                        eb.push(self.parse_stmt()?);
                    }
                    self.expect(Token::RBrace)?;
                    Some(eb)
                } else {
                    None
                };

                Ok(Stmt::If {
                    cond,
                    then_branch,
                    else_branch,
                })
            }
            Token::While => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;

                self.expect(Token::LBrace)?;
                let mut body = Vec::new();
                while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                    body.push(self.parse_stmt()?);
                }
                self.expect(Token::RBrace)?;
                Ok(Stmt::While { cond, body })
            }
            Token::For => {
                self.advance();
                self.expect(Token::LParen)?;
                if *self.peek() == Token::Let || *self.peek() == Token::Const {
                    self.advance();
                }
                let var_name = match self.advance() {
                    Token::Ident(s) => s,
                    other => return Err(format!("Expected loop variable name, found {:?}", other)),
                };
                self.expect(Token::In)?;
                let start = self.parse_expr()?;
                self.expect(Token::DotDot)?;
                let end = self.parse_expr()?;
                self.expect(Token::RParen)?;

                self.expect(Token::LBrace)?;
                let mut body = Vec::new();
                while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                    body.push(self.parse_stmt()?);
                }
                self.expect(Token::RBrace)?;

                Ok(Stmt::For {
                    var_name,
                    start,
                    end,
                    body,
                })
            }
            _ => {
                // Check assignments: ident = expr;, ident[index] = expr;, ident.field = expr;
                if let Token::Ident(name) = self.peek().clone() {
                    let next_tok = self.tokens.get(self.pos + 1).cloned();
                    if next_tok == Some(Token::Equal) {
                        self.advance();
                        self.advance();
                        let expr = self.parse_expr()?;
                        self.expect(Token::Semicolon)?;
                        return Ok(Stmt::Assign { name, expr });
                    } else if next_tok == Some(Token::LBracket) {
                        self.advance();
                        self.advance();
                        let index = self.parse_expr()?;
                        self.expect(Token::RBracket)?;
                        self.expect(Token::Equal)?;
                        let value = self.parse_expr()?;
                        self.expect(Token::Semicolon)?;
                        return Ok(Stmt::IndexAssign {
                            target: name,
                            index,
                            value,
                        });
                    } else if next_tok == Some(Token::Dot) {
                        let third_tok = self.tokens.get(self.pos + 2).cloned();
                        let fourth_tok = self.tokens.get(self.pos + 3).cloned();
                        if let (Some(Token::Ident(field)), Some(Token::Equal)) = (third_tok, fourth_tok) {
                            self.advance(); // consume target
                            self.advance(); // consume .
                            self.advance(); // consume field
                            self.advance(); // consume =
                            let value = self.parse_expr()?;
                            self.expect(Token::Semicolon)?;
                            return Ok(Stmt::FieldAssign {
                                target: name,
                                field,
                                value,
                            });
                        }
                    }
                }

                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logical_or()?;

        // Type casting: expr as Type
        while *self.peek() == Token::As {
            self.advance();
            let target_ty = self.parse_type()?;
            expr = Expr::Cast {
                expr: Box::new(expr),
                target_ty,
            };
        }

        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;

        while *self.peek() == Token::OrOr {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while *self.peek() == Token::AndAnd {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;

        while matches!(
            self.peek(),
            Token::EqualEqual
                | Token::BangEqual
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
        ) {
            let op = match self.advance() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_addition()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;

        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if *self.peek() == Token::Bang {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if *self.peek() == Token::Minus {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if *self.peek() == Token::LBracket {
                self.advance();
                let index = self.parse_expr()?;
                self.expect(Token::RBracket)?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else if *self.peek() == Token::Dot {
                self.advance();
                let field = match self.advance() {
                    Token::Ident(s) => s,
                    other => return Err(format!("Expected field name, found {:?}", other)),
                };
                expr = Expr::FieldAccess {
                    target: Box::new(expr),
                    field,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::IntLit(n) => Ok(Expr::IntLit(n)),
            Token::FloatLit(f) => Ok(Expr::FloatLit(f)),
            Token::StringLit(s) => Ok(Expr::StringLit(s)),
            Token::BoolLit(b) => Ok(Expr::BoolLit(b)),
            Token::LBracket => {
                let mut items = Vec::new();
                while *self.peek() != Token::RBracket && *self.peek() != Token::Eof {
                    items.push(self.parse_expr()?);
                    if *self.peek() == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(Token::RBracket)?;
                Ok(Expr::ListLit(items))
            }
            Token::LBrace => {
                // Object Map Literal: { key: value, ... }
                let mut pairs = Vec::new();
                while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                    let key = match self.advance() {
                        Token::Ident(s) => s,
                        Token::StringLit(s) => s,
                        other => return Err(format!("Expected map key, found {:?}", other)),
                    };
                    self.expect(Token::Colon)?;
                    let value = self.parse_expr()?;
                    pairs.push((key, value));

                    if *self.peek() == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(Token::RBrace)?;
                Ok(Expr::MapLit(pairs))
            }
            Token::Ident(name) => {
                if *self.peek() == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.peek() != Token::RParen && *self.peek() != Token::Eof {
                        args.push(self.parse_expr()?);
                        if *self.peek() == Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { callee: name, args })
                } else if *self.peek() == Token::FatArrow {
                    // Arrow function shorthand: x => x * 2
                    self.advance();
                    let body = self.parse_expr()?;
                    Ok(Expr::AnonymousFn {
                        params: vec![name],
                        body: Box::new(body),
                    })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }
}
