use super::ast::*;
use super::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current_token(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            self.tokens.last().expect("Tokens should not be empty")
        }
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.current_token().kind
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        } else {
            self.tokens.last().cloned().unwrap()
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, String> {
        let tok = self.advance();
        if tok.kind == expected {
            Ok(tok)
        } else {
            Err(format!(
                "Parse Error at {}:{}: Expected '{:?}', found '{:?}'",
                tok.line, tok.col, expected, tok.kind
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut imports = Vec::new();
        let mut structs = Vec::new();
        let mut functions = Vec::new();

        while *self.peek_kind() != TokenKind::Eof {
            match self.peek_kind() {
                TokenKind::Import => {
                    imports.push(self.parse_import()?);
                }
                TokenKind::Struct => {
                    structs.push(self.parse_struct()?);
                }
                _ => {
                    functions.push(self.parse_function()?);
                }
            }
        }
        Ok(Program {
            imports,
            structs,
            functions,
        })
    }

    fn parse_import(&mut self) -> Result<ImportStmt, String> {
        let import_tok = self.expect(TokenKind::Import)?;
        let path_tok = self.advance();
        let path = match path_tok.kind {
            TokenKind::StringLit(s) => s,
            other => {
                return Err(format!(
                    "Parse Error at {}:{}: Expected string literal path after 'import', found '{:?}'",
                    path_tok.line, path_tok.col, other
                ))
            }
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(ImportStmt {
            path,
            line: import_tok.line,
            col: import_tok.col,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.expect(TokenKind::Struct)?;
        let name_tok = self.advance();
        let name = match name_tok.kind {
            TokenKind::Ident(s) => s,
            other => {
                return Err(format!(
                    "Parse Error at {}:{}: Expected struct name identifier, found '{:?}'",
                    name_tok.line, name_tok.col, other
                ))
            }
        };

        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
            let field_tok = self.advance();
            let field_name = match field_tok.kind {
                TokenKind::Ident(s) => s,
                other => {
                    return Err(format!(
                        "Parse Error at {}:{}: Expected field name, found '{:?}'",
                        field_tok.line, field_tok.col, other
                    ))
                }
            };
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            fields.push(Param {
                name: field_name,
                ty,
            });

            if *self.peek_kind() == TokenKind::Comma || *self.peek_kind() == TokenKind::Semicolon {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;

        Ok(StructDef { name, fields })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let mut is_static = false;
        if *self.peek_kind() == TokenKind::Static {
            self.advance();
            is_static = true;
        }

        self.expect(TokenKind::Function)?;

        let name_tok = self.advance();
        let name = match name_tok.kind {
            TokenKind::Ident(s) => s,
            other => {
                return Err(format!(
                    "Parse Error at {}:{}: Expected function name, found '{:?}'",
                    name_tok.line, name_tok.col, other
                ))
            }
        };

        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        while *self.peek_kind() != TokenKind::RParen && *self.peek_kind() != TokenKind::Eof {
            let param_tok = self.advance();
            let param_name = match param_tok.kind {
                TokenKind::Ident(s) => s,
                other => {
                    return Err(format!(
                        "Parse Error at {}:{}: Expected parameter name, found '{:?}'",
                        param_tok.line, param_tok.col, other
                    ))
                }
            };

            let ty = if *self.peek_kind() == TokenKind::Colon {
                self.advance();
                self.parse_type()?
            } else if is_static {
                return Err(format!(
                    "Parse Error at {}:{}: Static function '{}' parameter '{}' must have an explicit type",
                    param_tok.line, param_tok.col, name, param_name
                ));
            } else {
                ElaichiType::Dynamic
            };

            params.push(Param {
                name: param_name,
                ty,
            });

            if *self.peek_kind() == TokenKind::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::RParen)?;

        let return_ty = if *self.peek_kind() == TokenKind::Colon || *self.peek_kind() == TokenKind::Arrow {
            self.advance();
            self.parse_type()?
        } else if is_static {
            ElaichiType::I64
        } else {
            ElaichiType::Dynamic
        };

        self.expect(TokenKind::LBrace)?;
        let mut body = Vec::new();
        while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Function {
            name,
            is_static,
            params,
            return_ty,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<ElaichiType, String> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::TypeI64 => Ok(ElaichiType::I64),
            TokenKind::TypeF64 => Ok(ElaichiType::F64),
            TokenKind::TypeBool => Ok(ElaichiType::Bool),
            TokenKind::TypeString => Ok(ElaichiType::String),
            TokenKind::Ident(ref s) if s == "dyn" || s == "any" => Ok(ElaichiType::Dynamic),
            TokenKind::Ident(s) => Ok(ElaichiType::Custom(s)),
            other => Err(format!(
                "Parse Error at {}:{}: Expected type identifier, found '{:?}'",
                tok.line, tok.col, other
            )),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek_kind() {
            TokenKind::Const | TokenKind::Let => {
                let is_const = *self.peek_kind() == TokenKind::Const;
                self.advance();

                let name_tok = self.advance();
                let name = match name_tok.kind {
                    TokenKind::Ident(s) => s,
                    other => {
                        return Err(format!(
                            "Parse Error at {}:{}: Expected variable name, found '{:?}'",
                            name_tok.line, name_tok.col, other
                        ))
                    }
                };

                if *self.peek_kind() == TokenKind::Colon {
                    self.advance();
                    let ty = self.parse_type()?;
                    self.expect(TokenKind::Equal)?;
                    let init = self.parse_expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::LetStatic {
                        name,
                        is_mut: !is_const,
                        ty,
                        init,
                    })
                } else {
                    self.expect(TokenKind::Equal)?;
                    let init = self.parse_expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::VarDynamic { name, init })
                }
            }
            TokenKind::Terminal => {
                self.advance();
                self.expect(TokenKind::Dot)?;
                let action_tok = self.advance();
                let action = match action_tok.kind {
                    TokenKind::Print => TerminalAction::Print,
                    TokenKind::Println => TerminalAction::Println,
                    TokenKind::Error => TerminalAction::Error,
                    TokenKind::Warn => TerminalAction::Warn,
                    TokenKind::Clear => TerminalAction::Clear,
                    other => {
                        return Err(format!(
                            "Parse Error at {}:{}: Expected terminal action, found '{:?}'",
                            action_tok.line, action_tok.col, other
                        ))
                    }
                };

                self.expect(TokenKind::LParen)?;
                let expr = if action != TerminalAction::Clear && *self.peek_kind() != TokenKind::RParen {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::Semicolon)?;

                Ok(Stmt::TerminalCall { action, expr })
            }
            TokenKind::Break => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Break)
            }
            TokenKind::Continue => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Continue)
            }
            TokenKind::Return => {
                self.advance();
                let expr = if *self.peek_kind() != TokenKind::Semicolon {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            TokenKind::If => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;

                self.expect(TokenKind::LBrace)?;
                let mut then_branch = Vec::new();
                while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
                    then_branch.push(self.parse_stmt()?);
                }
                self.expect(TokenKind::RBrace)?;

                let else_branch = if *self.peek_kind() == TokenKind::Else {
                    self.advance();
                    self.expect(TokenKind::LBrace)?;
                    let mut eb = Vec::new();
                    while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
                        eb.push(self.parse_stmt()?);
                    }
                    self.expect(TokenKind::RBrace)?;
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
            TokenKind::While => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;

                self.expect(TokenKind::LBrace)?;
                let mut body = Vec::new();
                while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
                    body.push(self.parse_stmt()?);
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Stmt::While { cond, body })
            }
            TokenKind::For => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                if *self.peek_kind() == TokenKind::Let || *self.peek_kind() == TokenKind::Const {
                    self.advance();
                }
                let var_tok = self.advance();
                let var_name = match var_tok.kind {
                    TokenKind::Ident(s) => s,
                    other => {
                        return Err(format!(
                            "Parse Error at {}:{}: Expected loop variable name, found '{:?}'",
                            var_tok.line, var_tok.col, other
                        ))
                    }
                };
                self.expect(TokenKind::In)?;
                let start = self.parse_expr()?;
                self.expect(TokenKind::DotDot)?;
                let end = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;

                self.expect(TokenKind::LBrace)?;
                let mut body = Vec::new();
                while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
                    body.push(self.parse_stmt()?);
                }
                self.expect(TokenKind::RBrace)?;

                Ok(Stmt::For {
                    var_name,
                    start,
                    end,
                    body,
                })
            }
            _ => {
                // Check assignments: ident = expr;, ident[index] = expr;, ident.field = expr;
                if let TokenKind::Ident(name) = self.peek_kind().clone() {
                    let next_tok = self.tokens.get(self.pos + 1).map(|t| &t.kind);
                    if next_tok == Some(&TokenKind::Equal) {
                        self.advance();
                        self.advance();
                        let expr = self.parse_expr()?;
                        self.expect(TokenKind::Semicolon)?;
                        return Ok(Stmt::Assign { name, expr });
                    } else if next_tok == Some(&TokenKind::LBracket) {
                        self.advance();
                        self.advance();
                        let index = self.parse_expr()?;
                        self.expect(TokenKind::RBracket)?;
                        self.expect(TokenKind::Equal)?;
                        let value = self.parse_expr()?;
                        self.expect(TokenKind::Semicolon)?;
                        return Ok(Stmt::IndexAssign {
                            target: name,
                            index,
                            value,
                        });
                    } else if next_tok == Some(&TokenKind::Dot) {
                        let third_tok = self.tokens.get(self.pos + 2).map(|t| &t.kind);
                        let fourth_tok = self.tokens.get(self.pos + 3).map(|t| &t.kind);
                        if let (Some(TokenKind::Ident(field)), Some(TokenKind::Equal)) = (third_tok, fourth_tok) {
                            let field_clone = field.clone();
                            self.advance(); // consume target
                            self.advance(); // consume .
                            self.advance(); // consume field
                            self.advance(); // consume =
                            let value = self.parse_expr()?;
                            self.expect(TokenKind::Semicolon)?;
                            return Ok(Stmt::FieldAssign {
                                target: name,
                                field: field_clone,
                                value,
                            });
                        }
                    }
                }

                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logical_or()?;

        // Type casting: expr as Type
        while *self.peek_kind() == TokenKind::As {
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

        while *self.peek_kind() == TokenKind::OrOr {
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

        while *self.peek_kind() == TokenKind::AndAnd {
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
            self.peek_kind(),
            TokenKind::EqualEqual
                | TokenKind::BangEqual
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
        ) {
            let op = match self.advance().kind {
                TokenKind::EqualEqual => BinaryOp::Equal,
                TokenKind::BangEqual => BinaryOp::NotEqual,
                TokenKind::Less => BinaryOp::Less,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
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

        while matches!(self.peek_kind(), TokenKind::Plus | TokenKind::Minus) {
            let op = match self.advance().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
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

        while matches!(self.peek_kind(), TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let op = match self.advance().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
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
        if *self.peek_kind() == TokenKind::Bang {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if *self.peek_kind() == TokenKind::Minus {
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
            if *self.peek_kind() == TokenKind::LBracket {
                self.advance();
                let index = self.parse_expr()?;
                self.expect(TokenKind::RBracket)?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else if *self.peek_kind() == TokenKind::Dot {
                self.advance();
                let field_tok = self.advance();
                let field = match field_tok.kind {
                    TokenKind::Ident(s) => s,
                    other => {
                        return Err(format!(
                            "Parse Error at {}:{}: Expected field name, found '{:?}'",
                            field_tok.line, field_tok.col, other
                        ))
                    }
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
        let tok = self.advance();
        match tok.kind {
            TokenKind::IntLit(n) => Ok(Expr::IntLit(n)),
            TokenKind::FloatLit(f) => Ok(Expr::FloatLit(f)),
            TokenKind::StringLit(s) => Ok(Expr::StringLit(s)),
            TokenKind::BoolLit(b) => Ok(Expr::BoolLit(b)),
            TokenKind::LBracket => {
                let mut items = Vec::new();
                while *self.peek_kind() != TokenKind::RBracket && *self.peek_kind() != TokenKind::Eof {
                    items.push(self.parse_expr()?);
                    if *self.peek_kind() == TokenKind::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RBracket)?;
                Ok(Expr::ListLit(items))
            }
            TokenKind::LBrace => {
                // Object Map Literal: { key: value, ... }
                let mut pairs = Vec::new();
                while *self.peek_kind() != TokenKind::RBrace && *self.peek_kind() != TokenKind::Eof {
                    let key_tok = self.advance();
                    let key = match key_tok.kind {
                        TokenKind::Ident(s) => s,
                        TokenKind::StringLit(s) => s,
                        other => {
                            return Err(format!(
                                "Parse Error at {}:{}: Expected map key, found '{:?}'",
                                key_tok.line, key_tok.col, other
                            ))
                        }
                    };
                    self.expect(TokenKind::Colon)?;
                    let value = self.parse_expr()?;
                    pairs.push((key, value));

                    if *self.peek_kind() == TokenKind::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Expr::MapLit(pairs))
            }
            TokenKind::Ident(name) => {
                if *self.peek_kind() == TokenKind::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while *self.peek_kind() != TokenKind::RParen && *self.peek_kind() != TokenKind::Eof {
                        args.push(self.parse_expr()?);
                        if *self.peek_kind() == TokenKind::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Call { callee: name, args })
                } else if *self.peek_kind() == TokenKind::FatArrow {
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
            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            other => Err(format!(
                "Parse Error at {}:{}: Unexpected token in expression: '{:?}'",
                tok.line, tok.col, other
            )),
        }
    }
}
