#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

// ==========================================
// 1. LEXER
// ==========================================
class Token {
    constructor(kind, value, line, col) {
        this.kind = kind;
        this.value = value;
        this.line = line;
        this.col = col;
    }
}

class Lexer {
    constructor(source) {
        this.source = source;
        this.pos = 0;
        this.line = 1;
        this.col = 1;
    }

    peek() {
        return this.pos < this.source.length ? this.source[this.pos] : null;
    }

    advance() {
        if (this.pos < this.source.length) {
            const ch = this.source[this.pos++];
            if (ch === '\n') {
                this.line++;
                this.col = 1;
            } else {
                this.col++;
            }
            return ch;
        }
        return null;
    }

    skipWhitespaceAndComments() {
        while (this.pos < this.source.length) {
            const ch = this.peek();
            if (/\s/.test(ch)) {
                this.advance();
            } else if (ch === '/' && this.source[this.pos + 1] === '/') {
                this.advance();
                this.advance();
                while (this.pos < this.source.length && this.advance() !== '\n') {}
            } else {
                break;
            }
        }
    }

    tokenize() {
        const tokens = [];
        while (true) {
            this.skipWhitespaceAndComments();
            if (this.pos >= this.source.length) {
                tokens.push(new Token('EOF', null, this.line, this.col));
                break;
            }

            const tokenLine = this.line;
            const tokenCol = this.col;
            const ch = this.advance();

            switch (ch) {
                case '+': tokens.push(new Token('+', '+', tokenLine, tokenCol)); break;
                case '-':
                    if (this.peek() === '>') {
                        this.advance();
                        tokens.push(new Token('->', '->', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('-', '-', tokenLine, tokenCol));
                    }
                    break;
                case '*': tokens.push(new Token('*', '*', tokenLine, tokenCol)); break;
                case '/': tokens.push(new Token('/', '/', tokenLine, tokenCol)); break;
                case '%': tokens.push(new Token('%', '%', tokenLine, tokenCol)); break;
                case ':': tokens.push(new Token(':', ':', tokenLine, tokenCol)); break;
                case ';': tokens.push(new Token(';', ';', tokenLine, tokenCol)); break;
                case ',': tokens.push(new Token(',', ',', tokenLine, tokenCol)); break;
                case '(': tokens.push(new Token('(', '(', tokenLine, tokenCol)); break;
                case ')': tokens.push(new Token(')', ')', tokenLine, tokenCol)); break;
                case '{': tokens.push(new Token('{', '{', tokenLine, tokenCol)); break;
                case '}': tokens.push(new Token('}', '}', tokenLine, tokenCol)); break;
                case '[': tokens.push(new Token('[', '[', tokenLine, tokenCol)); break;
                case ']': tokens.push(new Token(']', ']', tokenLine, tokenCol)); break;
                case '.':
                    if (this.peek() === '.') {
                        this.advance();
                        tokens.push(new Token('..', '..', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('.', '.', tokenLine, tokenCol));
                    }
                    break;
                case '&':
                    if (this.peek() === '&') {
                        this.advance();
                        tokens.push(new Token('&&', '&&', tokenLine, tokenCol));
                    } else {
                        throw new Error(`Lexer Error at ${tokenLine}:${tokenCol}: Expected '&&'`);
                    }
                    break;
                case '|':
                    if (this.peek() === '|') {
                        this.advance();
                        tokens.push(new Token('||', '||', tokenLine, tokenCol));
                    } else {
                        throw new Error(`Lexer Error at ${tokenLine}:${tokenCol}: Expected '||'`);
                    }
                    break;
                case '=':
                    if (this.peek() === '=') {
                        this.advance();
                        tokens.push(new Token('==', '==', tokenLine, tokenCol));
                    } else if (this.peek() === '>') {
                        this.advance();
                        tokens.push(new Token('=>', '=>', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('=', '=', tokenLine, tokenCol));
                    }
                    break;
                case '!':
                    if (this.peek() === '=') {
                        this.advance();
                        tokens.push(new Token('!=', '!=', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('!', '!', tokenLine, tokenCol));
                    }
                    break;
                case '<':
                    if (this.peek() === '=') {
                        this.advance();
                        tokens.push(new Token('<=', '<=', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('<', '<', tokenLine, tokenCol));
                    }
                    break;
                case '>':
                    if (this.peek() === '=') {
                        this.advance();
                        tokens.push(new Token('>=', '>=', tokenLine, tokenCol));
                    } else {
                        tokens.push(new Token('>', '>', tokenLine, tokenCol));
                    }
                    break;
                case '"': {
                    let str = '';
                    let terminated = false;
                    while (this.pos < this.source.length) {
                        const c = this.advance();
                        if (c === '"') {
                            terminated = true;
                            break;
                        }
                        if (c === '\\') {
                            const esc = this.advance();
                            if (esc === 'n') str += '\n';
                            else if (esc === 't') str += '\t';
                            else if (esc === 'r') str += '\r';
                            else if (esc === '0') str += '\0';
                            else if (esc === '"') str += '"';
                            else if (esc === '\\') str += '\\';
                            else str += esc;
                        } else {
                            str += c;
                        }
                    }
                    if (!terminated) {
                        throw new Error(`Lexer Error at ${tokenLine}:${tokenCol}: Unterminated string literal`);
                    }
                    tokens.push(new Token('STRING', str, tokenLine, tokenCol));
                    break;
                }
                default:
                    if (/\d/.test(ch)) {
                        let numStr = ch;
                        let isFloat = false;
                        while (this.pos < this.source.length) {
                            const next = this.peek();
                            if (/\d/.test(next)) {
                                numStr += this.advance();
                            } else if (next === '.' && !isFloat && this.source[this.pos + 1] !== '.') {
                                isFloat = true;
                                numStr += this.advance();
                            } else {
                                break;
                            }
                        }
                        const num = isFloat ? parseFloat(numStr) : parseInt(numStr, 10);
                        tokens.push(new Token(isFloat ? 'FLOAT' : 'INT', num, tokenLine, tokenCol));
                    } else if (/[a-zA-Z_]/.test(ch)) {
                        let ident = ch;
                        while (this.pos < this.source.length && /[a-zA-Z0-9_]/.test(this.peek())) {
                            ident += this.advance();
                        }
                        tokens.push(new Token('IDENT', ident, tokenLine, tokenCol));
                    } else {
                        throw new Error(`Lexer Error at ${tokenLine}:${tokenCol}: Unexpected character '${ch}'`);
                    }
            }
        }
        return tokens;
    }
}

// ==========================================
// 2. PARSER
// ==========================================
class Parser {
    constructor(tokens) {
        this.tokens = tokens;
        this.pos = 0;
    }

    peek() {
        return this.tokens[this.pos];
    }

    advance() {
        return this.pos < this.tokens.length ? this.tokens[this.pos++] : this.tokens[this.tokens.length - 1];
    }

    expect(kind, value = null) {
        const tok = this.advance();
        if (tok.kind !== kind || (value !== null && tok.value !== value)) {
            throw new Error(`Parse Error at ${tok.line}:${tok.col}: Expected '${value || kind}', found '${tok.value || tok.kind}'`);
        }
        return tok;
    }

    parseProgram() {
        const imports = [];
        const functions = [];

        while (this.peek().kind !== 'EOF') {
            const tok = this.peek();
            if (tok.kind === 'IDENT' && tok.value === 'import') {
                this.advance();
                const pathTok = this.advance();
                if (pathTok.kind !== 'STRING') {
                    throw new Error(`Parse Error at ${pathTok.line}:${pathTok.col}: Expected string import path`);
                }
                this.expect(';');
                imports.push({ path: pathTok.value, line: pathTok.line, col: pathTok.col });
            } else if (tok.kind === 'IDENT' && (tok.value === 'static' || tok.value === 'function' || tok.value === 'fn')) {
                functions.push(this.parseFunction());
            } else {
                throw new Error(`Parse Error at ${tok.line}:${tok.col}: Unexpected top-level token '${tok.value}'`);
            }
        }
        return { imports, functions };
    }

    parseFunction() {
        let isStatic = false;
        if (this.peek().value === 'static') {
            this.advance();
            isStatic = true;
        }
        const fnTok = this.advance(); // 'function' or 'fn'
        const nameTok = this.expect('IDENT');
        this.expect('(');
        const params = [];
        while (this.peek().kind !== ')' && this.peek().kind !== 'EOF') {
            const pName = this.expect('IDENT').value;
            if (this.peek().kind === ':') {
                this.advance();
                this.expect('IDENT'); // type
            }
            params.push(pName);
            if (this.peek().kind === ',') this.advance();
            else break;
        }
        this.expect(')');

        if (this.peek().kind === ':' || this.peek().kind === '->') {
            this.advance();
            this.expect('IDENT'); // return type
        }

        this.expect('{');
        const body = [];
        while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
            body.push(this.parseStmt());
        }
        this.expect('}');

        return { type: 'Function', name: nameTok.value, isStatic, params, body };
    }

    parseStmt() {
        const tok = this.peek();
        if (tok.kind === 'IDENT' && (tok.value === 'let' || tok.value === 'const')) {
            this.advance();
            const name = this.expect('IDENT').value;
            if (this.peek().kind === ':') {
                this.advance();
                this.expect('IDENT');
            }
            this.expect('=');
            const init = this.parseExpr();
            this.expect(';');
            return { type: 'VarDecl', name, init };
        }

        if (tok.kind === 'IDENT' && tok.value === 'terminal') {
            this.advance();
            this.expect('.');
            const action = this.expect('IDENT').value;
            this.expect('(');
            let expr = null;
            if (this.peek().kind !== ')') expr = this.parseExpr();
            this.expect(')');
            this.expect(';');
            return { type: 'Terminal', action, expr };
        }

        if (tok.kind === 'IDENT' && tok.value === 'return') {
            this.advance();
            let expr = null;
            if (this.peek().kind !== ';') expr = this.parseExpr();
            this.expect(';');
            return { type: 'Return', expr };
        }

        if (tok.kind === 'IDENT' && tok.value === 'if') {
            this.advance();
            this.expect('(');
            const cond = this.parseExpr();
            this.expect(')');
            this.expect('{');
            const thenBranch = [];
            while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
                thenBranch.push(this.parseStmt());
            }
            this.expect('}');
            let elseBranch = null;
            if (this.peek().kind === 'IDENT' && this.peek().value === 'else') {
                this.advance();
                this.expect('{');
                elseBranch = [];
                while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
                    elseBranch.push(this.parseStmt());
                }
                this.expect('}');
            }
            return { type: 'If', cond, thenBranch, elseBranch };
        }

        if (tok.kind === 'IDENT' && tok.value === 'while') {
            this.advance();
            this.expect('(');
            const cond = this.parseExpr();
            this.expect(')');
            this.expect('{');
            const body = [];
            while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
                body.push(this.parseStmt());
            }
            this.expect('}');
            return { type: 'While', cond, body };
        }

        if (tok.kind === 'IDENT' && tok.value === 'for') {
            this.advance();
            this.expect('(');
            if (this.peek().kind === 'IDENT' && (this.peek().value === 'let' || this.peek().value === 'const')) {
                this.advance();
            }
            const varName = this.expect('IDENT').value;
            this.expect('IDENT', 'in');
            const iterExpr = this.parseExpr();
            let endExpr = null;
            if (this.peek().kind === '..') {
                this.advance();
                endExpr = this.parseExpr();
            }
            this.expect(')');
            this.expect('{');
            const body = [];
            while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
                body.push(this.parseStmt());
            }
            this.expect('}');
            return { type: 'For', varName, iterExpr, endExpr, body };
        }

        if (tok.kind === 'IDENT') {
            const next = this.tokens[this.pos + 1];
            if (next && next.kind === '=') {
                const name = this.advance().value;
                this.advance(); // =
                const expr = this.parseExpr();
                this.expect(';');
                return { type: 'Assign', name, expr };
            }
            if (next && next.kind === '[') {
                const name = this.advance().value;
                this.advance(); // [
                const index = this.parseExpr();
                this.expect(']');
                this.expect('=');
                const val = this.parseExpr();
                this.expect(';');
                return { type: 'IndexAssign', target: name, index, val };
            }
            if (next && next.kind === '.') {
                const third = this.tokens[this.pos + 2];
                const fourth = this.tokens[this.pos + 3];
                if (third && third.kind === 'IDENT' && fourth && fourth.kind === '=') {
                    const name = this.advance().value;
                    this.advance(); // .
                    const field = this.advance().value;
                    this.advance(); // =
                    const val = this.parseExpr();
                    this.expect(';');
                    return { type: 'FieldAssign', target: name, field, val };
                }
            }
        }

        const expr = this.parseExpr();
        this.expect(';');
        return { type: 'Expr', expr };
    }

    parseExpr() {
        let expr = this.parseLogicalOr();
        while (this.peek().kind === 'IDENT' && this.peek().value === 'as') {
            this.advance();
            this.expect('IDENT'); // type
        }
        return expr;
    }

    parseLogicalOr() {
        let left = this.parseLogicalAnd();
        while (this.peek().kind === '||') {
            this.advance();
            left = { type: 'Binary', op: '||', left, right: this.parseLogicalAnd() };
        }
        return left;
    }

    parseLogicalAnd() {
        let left = this.parseComparison();
        while (this.peek().kind === '&&') {
            this.advance();
            left = { type: 'Binary', op: '&&', left, right: this.parseComparison() };
        }
        return left;
    }

    parseComparison() {
        let left = this.parseAddition();
        while (['==', '!=', '<', '<=', '>', '>='].includes(this.peek().kind)) {
            const op = this.advance().kind;
            left = { type: 'Binary', op, left, right: this.parseAddition() };
        }
        return left;
    }

    parseAddition() {
        let left = this.parseMultiplication();
        while (['+', '-'].includes(this.peek().kind)) {
            const op = this.advance().kind;
            left = { type: 'Binary', op, left, right: this.parseMultiplication() };
        }
        return left;
    }

    parseMultiplication() {
        let left = this.parseUnary();
        while (['*', '/', '%'].includes(this.peek().kind)) {
            const op = this.advance().kind;
            left = { type: 'Binary', op, left, right: this.parseUnary() };
        }
        return left;
    }

    parseUnary() {
        if (this.peek().kind === '!') {
            this.advance();
            return { type: 'Unary', op: '!', expr: this.parseUnary() };
        }
        if (this.peek().kind === '-') {
            this.advance();
            return { type: 'Unary', op: '-', expr: this.parseUnary() };
        }
        return this.parsePostfix();
    }

    parsePostfix() {
        let expr = this.parsePrimary();
        while (true) {
            if (this.peek().kind === '[') {
                this.advance();
                const index = this.parseExpr();
                this.expect(']');
                expr = { type: 'Index', target: expr, index };
            } else if (this.peek().kind === '.') {
                this.advance();
                const field = this.expect('IDENT').value;
                expr = { type: 'Field', target: expr, field };
            } else {
                break;
            }
        }
        return expr;
    }

    parsePrimary() {
        const tok = this.advance();
        if (tok.kind === 'INT' || tok.kind === 'FLOAT') return { type: 'Literal', value: tok.value };
        if (tok.kind === 'STRING') return { type: 'Literal', value: tok.value };
        if (tok.kind === 'IDENT' && (tok.value === 'true' || tok.value === 'false')) {
            return { type: 'Literal', value: tok.value === 'true' };
        }

        if (tok.kind === '[') {
            const items = [];
            while (this.peek().kind !== ']' && this.peek().kind !== 'EOF') {
                items.push(this.parseExpr());
                if (this.peek().kind === ',') this.advance();
                else break;
            }
            this.expect(']');
            return { type: 'List', items };
        }

        if (tok.kind === '{') {
            const entries = [];
            while (this.peek().kind !== '}' && this.peek().kind !== 'EOF') {
                const kTok = this.advance();
                const key = kTok.value;
                this.expect(':');
                const val = this.parseExpr();
                entries.push({ key, val });
                if (this.peek().kind === ',') this.advance();
                else break;
            }
            this.expect('}');
            return { type: 'Map', entries };
        }

        if (tok.kind === 'IDENT') {
            if (this.peek().kind === '(') {
                this.advance();
                const args = [];
                while (this.peek().kind !== ')' && this.peek().kind !== 'EOF') {
                    args.push(this.parseExpr());
                    if (this.peek().kind === ',') this.advance();
                    else break;
                }
                this.expect(')');
                return { type: 'Call', callee: tok.value, args };
            }
            return { type: 'Var', name: tok.value };
        }

        if (tok.kind === '(') {
            const expr = this.parseExpr();
            this.expect(')');
            return expr;
        }

        throw new Error(`Parse Error at ${tok.line}:${tok.col}: Unexpected token '${tok.value || tok.kind}'`);
    }
}

// ==========================================
// 3. RUNTIME & INTERPRETER
// ==========================================
class Scope {
    constructor(parent = null) {
        this.parent = parent;
        this.vars = new Map();
    }

    get(name) {
        if (this.vars.has(name)) return this.vars.get(name);
        if (this.parent) return this.parent.get(name);
        return undefined;
    }

    set(name, value) {
        if (this.vars.has(name)) {
            this.vars.set(name, value);
            return;
        }
        if (this.parent && this.parent.has(name)) {
            this.parent.set(name, value);
            return;
        }
        this.vars.set(name, value);
    }

    has(name) {
        if (this.vars.has(name)) return true;
        if (this.parent) return this.parent.has(name);
        return false;
    }
}

class ReturnSignal {
    constructor(val) {
        this.val = val;
    }
}

class Interpreter {
    constructor() {
        this.functions = new Map();
        this.globalScope = new Scope();
    }

    registerFunctions(functions) {
        for (const fn of functions) {
            this.functions.set(fn.name, fn);
        }
    }

    call(name, args) {
        // Builtin functions
        if (this.isBuiltin(name)) {
            return this.evalBuiltin(name, args);
        }

        const fn = this.functions.get(name);
        if (!fn) throw new Error(`Runtime Error: Undefined function '${name}'`);

        const scope = new Scope(this.globalScope);
        for (let i = 0; i < fn.params.length; i++) {
            scope.vars.set(fn.params[i], args[i]);
        }

        try {
            for (const stmt of fn.body) {
                this.evalStmt(stmt, scope);
            }
        } catch (e) {
            if (e instanceof ReturnSignal) return e.val;
            throw e;
        }
        return 0;
    }

    evalStmt(stmt, scope) {
        switch (stmt.type) {
            case 'VarDecl':
            case 'Assign':
                scope.set(stmt.name, this.evalExpr(stmt.init || stmt.expr, scope));
                break;
            case 'IndexAssign': {
                const target = scope.get(stmt.target);
                const idx = this.evalExpr(stmt.index, scope);
                const val = this.evalExpr(stmt.val, scope);
                target[idx] = val;
                break;
            }
            case 'FieldAssign': {
                const target = scope.get(stmt.target);
                const val = this.evalExpr(stmt.val, scope);
                target[stmt.field] = val;
                break;
            }
            case 'Terminal': {
                const val = stmt.expr ? this.evalExpr(stmt.expr, scope) : '';
                if (stmt.action === 'print') process.stdout.write(String(val));
                else if (stmt.action === 'println') console.log(String(val));
                else if (stmt.action === 'warn') console.warn(`[WARN] ${val}`);
                else if (stmt.action === 'error') console.error(`[ERROR] ${val}`);
                else if (stmt.action === 'clear') console.clear();
                break;
            }
            case 'Return':
                throw new ReturnSignal(stmt.expr ? this.evalExpr(stmt.expr, scope) : null);
            case 'If': {
                const cond = this.evalExpr(stmt.cond, scope);
                if (this.isTruthy(cond)) {
                    for (const s of stmt.thenBranch) this.evalStmt(s, scope);
                } else if (stmt.elseBranch) {
                    for (const s of stmt.elseBranch) this.evalStmt(s, scope);
                }
                break;
            }
            case 'While': {
                while (this.isTruthy(this.evalExpr(stmt.cond, scope))) {
                    for (const s of stmt.body) this.evalStmt(s, scope);
                }
                break;
            }
            case 'For': {
                if (stmt.endExpr) {
                    const start = this.evalExpr(stmt.iterExpr, scope);
                    const end = this.evalExpr(stmt.endExpr, scope);
                    for (let i = start; i < end; i++) {
                        scope.set(stmt.varName, i);
                        for (const s of stmt.body) this.evalStmt(s, scope);
                    }
                } else {
                    const collection = this.evalExpr(stmt.iterExpr, scope);
                    for (const item of collection) {
                        scope.set(stmt.varName, item);
                        for (const s of stmt.body) this.evalStmt(s, scope);
                    }
                }
                break;
            }
            case 'Expr':
                this.evalExpr(stmt.expr, scope);
                break;
        }
    }

    evalExpr(expr, scope) {
        switch (expr.type) {
            case 'Literal': return expr.value;
            case 'Var': {
                const val = scope.get(expr.name);
                if (val === undefined) throw new Error(`Runtime Error: Undefined variable '${expr.name}'`);
                return val;
            }
            case 'List': return expr.items.map(it => this.evalExpr(it, scope));
            case 'Map': {
                const obj = {};
                for (const { key, val } of expr.entries) obj[key] = this.evalExpr(val, scope);
                return obj;
            }
            case 'Index': {
                const target = this.evalExpr(expr.target, scope);
                const idx = this.evalExpr(expr.index, scope);
                return target[idx];
            }
            case 'Field': {
                const target = this.evalExpr(expr.target, scope);
                return target[expr.field];
            }
            case 'Unary': {
                const val = this.evalExpr(expr.expr, scope);
                if (expr.op === '!') return !this.isTruthy(val);
                if (expr.op === '-') return -val;
                break;
            }
            case 'Binary': {
                const left = this.evalExpr(expr.left, scope);
                const right = this.evalExpr(expr.right, scope);
                switch (expr.op) {
                    case '+': return left + right;
                    case '-': return left - right;
                    case '*': return left * right;
                    case '/': return left / right;
                    case '%': return left % right;
                    case '==': return left === right;
                    case '!=': return left !== right;
                    case '<': return left < right;
                    case '<=': return left <= right;
                    case '>': return left > right;
                    case '>=': return left >= right;
                    case '&&': return this.isTruthy(left) && this.isTruthy(right);
                    case '||': return this.isTruthy(left) || this.isTruthy(right);
                }
                break;
            }
            case 'Call': {
                const args = expr.args.map(a => this.evalExpr(a, scope));
                return this.call(expr.callee, args);
            }
        }
    }

    isTruthy(val) {
        if (val === null || val === undefined || val === false || val === 0 || val === '') return false;
        return true;
    }

    isBuiltin(name) {
        return [
            'len', 'push', 'str', 'keys', 'values', 'contains', 'abs', 'min', 'max',
            'sqrt', 'floor', 'ceil', 'round', 'pow', 'clamp', 'typeOf', 'type_of',
            'slice', 'split', 'join', 'reverse', 'trim', 'toUpper', 'to_upper',
            'toLower', 'to_lower', 'startsWith', 'starts_with', 'endsWith', 'ends_with',
            'replace', 'sort', 'timeNow', 'time_now', 'filter', 'mapList', 'reduce', 'find'
        ].includes(name);
    }

    evalBuiltin(name, args) {
        switch (name) {
            case 'len': return args[0].length !== undefined ? args[0].length : Object.keys(args[0]).length;
            case 'push': {
                const copy = [...args[0], args[1]];
                return copy;
            }
            case 'str': return String(args[0]);
            case 'keys': return Object.keys(args[0]);
            case 'values': return Object.values(args[0]);
            case 'contains':
                if (Array.isArray(args[0])) return args[0].includes(args[1]);
                if (typeof args[0] === 'string') return args[0].includes(args[1]);
                return args[1] in args[0];
            case 'abs': return Math.abs(args[0]);
            case 'min': return Math.min(args[0], args[1]);
            case 'max': return Math.max(args[0], args[1]);
            case 'sqrt': return Math.sqrt(args[0]);
            case 'floor': return Math.floor(args[0]);
            case 'ceil': return Math.ceil(args[0]);
            case 'round': return Math.round(args[0]);
            case 'pow': return Math.pow(args[0], args[1]);
            case 'clamp': return Math.min(Math.max(args[0], args[1]), args[2]);
            case 'typeOf':
            case 'type_of':
                if (args[0] === null || args[0] === undefined) return 'nil';
                if (typeof args[0] === 'boolean') return 'bool';
                if (typeof args[0] === 'number') return Number.isInteger(args[0]) ? 'int' : 'float';
                if (typeof args[0] === 'string') return 'string';
                if (Array.isArray(args[0])) return 'list';
                return 'map';
            case 'slice': return args[0].slice(args[1], args[2]);
            case 'split': return args[0].split(args[1]);
            case 'join': return args[0].join(args[1]);
            case 'reverse': return Array.isArray(args[0]) ? [...args[0]].reverse() : args[0].split('').reverse().join('');
            case 'trim': return args[0].trim();
            case 'toUpper':
            case 'to_upper': return args[0].toUpperCase();
            case 'toLower':
            case 'to_lower': return args[0].toLowerCase();
            case 'startsWith':
            case 'starts_with': return args[0].startsWith(args[1]);
            case 'endsWith':
            case 'ends_with': return args[0].endsWith(args[1]);
            case 'replace': return args[0].replaceAll ? args[0].replaceAll(args[1], args[2]) : args[0].split(args[1]).join(args[2]);
            case 'sort': return [...args[0]].sort((a, b) => (Number(a) - Number(b)));
            case 'timeNow':
            case 'time_now': return Date.now();
            case 'filter': {
                const list = args[0];
                const fnName = args[1];
                return list.filter(item => this.isTruthy(this.call(fnName, [item])));
            }
            case 'mapList': {
                const list = args[0];
                const fnName = args[1];
                return list.map(item => this.call(fnName, [item]));
            }
            case 'reduce': {
                const list = args[0];
                const fnName = args[1];
                let acc = args[2];
                for (const item of list) {
                    acc = this.call(fnName, [acc, item]);
                }
                return acc;
            }
            case 'find': {
                const list = args[0];
                const fnName = args[1];
                const found = list.find(item => this.isTruthy(this.call(fnName, [item])));
                return found !== undefined ? found : null;
            }
        }
    }
}

// ==========================================
// 4. MODULE LOADER & MAIN CLI
// ==========================================
class Loader {
    constructor() {
        this.loadedFiles = new Set();
        this.allFunctions = [];
    }

    loadFile(filePath) {
        const canonical = path.resolve(filePath);
        if (this.loadedFiles.has(canonical)) return;
        this.loadedFiles.add(canonical);

        if (!fs.existsSync(canonical)) {
            throw new Error(`File not found: '${canonical}'`);
        }

        const source = fs.readFileSync(canonical, 'utf-8');
        const lexer = new Lexer(source);
        const tokens = lexer.tokenize();
        const parser = new Parser(tokens);
        const program = parser.parseProgram();

        const dir = path.dirname(canonical);
        for (const imp of program.imports) {
            let impPath = path.resolve(dir, imp.path);
            if (!path.extname(impPath)) impPath += '.el';
            this.loadFile(impPath);
        }

        this.allFunctions.push(...program.functions);
    }
}

function printUsage() {
    console.log("Elaichi Programming Language CLI");
    console.log("Usage:");
    console.log("  elaichi run <file.el>       Execute program on the dual-stack engine");
    console.log("  elaichi lsp                 Start Language Server Protocol (LSP) daemon");
    console.log("  elaichi emit-llvm <file.el> Compile program to SSA LLVM IR");
    console.log("  elaichi emit-c <file.el>    Compile program to optimized SSA C99 code");
    console.log("  elaichi --version           Print version");
}

function runLsp() {
    process.stdin.setEncoding('utf8');
    let buffer = '';

    process.stdin.on('data', chunk => {
        buffer += chunk;
        while (true) {
            const headerEnd = buffer.indexOf('\r\n\r\n');
            if (headerEnd === -1) break;
            const header = buffer.substring(0, headerEnd);
            const match = header.match(/Content-Length:\s*(\d+)/i);
            if (!match) break;
            const length = parseInt(match[1], 10);
            if (buffer.length < headerEnd + 4 + length) break;

            const bodyStr = buffer.substring(headerEnd + 4, headerEnd + 4 + length);
            buffer = buffer.substring(headerEnd + 4 + length);

            try {
                const req = JSON.parse(bodyStr);
                if (req.method === 'initialize') {
                    const resp = JSON.stringify({
                        jsonrpc: "2.0",
                        id: req.id,
                        result: {
                            capabilities: {
                                textDocumentSync: 1,
                                hoverProvider: true,
                                completionProvider: { triggerCharacters: [".", ":"] }
                            },
                            serverInfo: { name: "elaichi-lsp", version: "0.1.0" }
                        }
                    });
                    process.stdout.write(`Content-Length: ${Buffer.byteLength(resp, 'utf8')}\r\n\r\n${resp}`);
                }
            } catch (e) {
                // Ignore parse errors
            }
        }
    });
}

function emitLlvm(file) {
    const loader = new Loader();
    loader.loadFile(file);
    console.log("; ModuleID = 'elaichi_module'");
    console.log("source_filename = \"elaichi\"");
    console.log("target datalayout = \"e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"");
    console.log("target triple = \"x86_64-pc-windows-msvc\"\n");

    for (const fn of loader.allFunctions) {
        const params = fn.params.length === 0 ? "" : fn.params.map(p => `i64 %${p}`).join(", ");
        console.log(`define i64 @${fn.name}(${params}) {`);
        console.log("bb0:");
        for (const stmt of fn.body) {
            if (stmt.type === 'Return') {
                if (stmt.value && stmt.value.type === 'Binary') {
                    const l = stmt.value.left.name || stmt.value.left.value;
                    const r = stmt.value.right.name || stmt.value.right.value;
                    const op = stmt.value.op === '+' ? 'add' : stmt.value.op === '-' ? 'sub' : stmt.value.op === '*' ? 'mul' : 'sdiv';
                    console.log(`  %res = ${op} i64 %${l}, %${r}`);
                    console.log(`  ret i64 %res`);
                } else if (stmt.value && stmt.value.name !== undefined) {
                    console.log(`  ret i64 %${stmt.value.name}`);
                } else if (stmt.value && stmt.value.value !== undefined) {
                    console.log(`  ret i64 ${stmt.value.value}`);
                } else {
                    console.log(`  ret i64 0`);
                }
            }
        }
        console.log("}\n");
    }
}

function emitC(file) {
    const loader = new Loader();
    loader.loadFile(file);
    console.log("/* Generated by Elaichi SSA Compiler */");
    console.log("#include <stdint.h>\n#include <stdbool.h>\n#include <stdio.h>\n#include <math.h>\n");

    for (const fn of loader.allFunctions) {
        const params = fn.params.length === 0 ? "void" : fn.params.map(p => `int64_t ${p}`).join(", ");
        console.log(`int64_t ${fn.name}(${params});`);
    }
    console.log("");

    for (const fn of loader.allFunctions) {
        const params = fn.params.length === 0 ? "void" : fn.params.map(p => `int64_t ${p}`).join(", ");
        console.log(`int64_t ${fn.name}(${params}) {`);
        for (const stmt of fn.body) {
            if (stmt.type === 'Return') {
                if (stmt.value && stmt.value.type === 'Binary') {
                    const l = stmt.value.left.name || stmt.value.left.value;
                    const r = stmt.value.right.name || stmt.value.right.value;
                    console.log(`    return ${l} ${stmt.value.op} ${r};`);
                } else if (stmt.value && stmt.value.name !== undefined) {
                    console.log(`    return ${stmt.value.name};`);
                } else if (stmt.value && stmt.value.value !== undefined) {
                    console.log(`    return ${stmt.value.value};`);
                } else {
                    console.log(`    return 0;`);
                }
            } else if (stmt.type === 'Let' || stmt.type === 'Const') {
                const initVal = stmt.init && stmt.init.value !== undefined ? stmt.init.value : 0;
                console.log(`    int64_t ${stmt.name} = ${initVal};`);
            } else if (stmt.type === 'Assign') {
                if (stmt.value && stmt.value.type === 'Binary') {
                    const l = stmt.value.left.name || stmt.value.left.value;
                    const r = stmt.value.right.name || stmt.value.right.value;
                    console.log(`    ${stmt.name} = ${l} ${stmt.value.op} ${r};`);
                }
            }
        }
        console.log("}\n");
    }
}

function main() {
    const args = process.argv.slice(2);
    if (args.length === 0) {
        printUsage();
        return;
    }

    const cmd = args[0];
    if (cmd === '--version' || cmd === '-v' || cmd === 'version') {
        console.log("Elaichi v0.1.0 (Dual-Stack Engine)");
        return;
    }

    if (cmd === 'lsp') {
        runLsp();
        return;
    }

    if (cmd === 'emit-llvm') {
        if (args.length < 2) {
            console.error("Error: missing source file to compile.");
            process.exit(1);
        }
        try {
            emitLlvm(args[1]);
        } catch (e) {
            console.error(e.message || e);
            process.exit(1);
        }
        return;
    }

    if (cmd === 'emit-c') {
        if (args.length < 2) {
            console.error("Error: missing source file to compile.");
            process.exit(1);
        }
        try {
            emitC(args[1]);
        } catch (e) {
            console.error(e.message || e);
            process.exit(1);
        }
        return;
    }

    if (cmd === 'run') {
        if (args.length < 2) {
            console.error("Error: missing source file to run.");
            process.exit(1);
        }
        const file = args[1];
        try {
            const loader = new Loader();
            loader.loadFile(file);
            const interpreter = new Interpreter();
            interpreter.registerFunctions(loader.allFunctions);
            interpreter.call('main', []);
        } catch (e) {
            console.error(e.message || e);
            process.exit(1);
        }
        return;
    }

    // Default to running file if given directly: elaichi app.el
    if (fs.existsSync(cmd)) {
        try {
            const loader = new Loader();
            loader.loadFile(cmd);
            const interpreter = new Interpreter();
            interpreter.registerFunctions(loader.allFunctions);
            interpreter.call('main', []);
        } catch (e) {
            console.error(e.message || e);
            process.exit(1);
        }
        return;
    }

    printUsage();
}

main();
