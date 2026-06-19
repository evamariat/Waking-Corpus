#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Number(String),
    Symbol(char),
    Whitespace,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.pos;

        match self.peek() {
            None => Token { kind: TokenKind::EOF, pos: start },

            Some(ch) if ch.is_whitespace() => {
                self.bump();
                Token { kind: TokenKind::Whitespace, pos: start }
            }

            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                let mut s = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        s.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }
                Token { kind: TokenKind::Ident(s), pos: start }
            }

            Some(ch) if ch.is_ascii_digit() => {
                let mut s = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        s.push(c);
                        self.bump();
                    } else {
                        break;
                    }
                }
                Token { kind: TokenKind::Number(s), pos: start }
            }

            Some(ch) => {
                self.bump();
                Token { kind: TokenKind::Symbol(ch), pos: start }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let eof = tok.kind == TokenKind::EOF;
            tokens.push(tok);
            if eof { break; }
        }
        tokens
    }
}
