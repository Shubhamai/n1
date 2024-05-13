use logos::Logos;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    #[default]
    Other,
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(error = LexingError)]
pub enum TokenType {
    // move an immediate value into a register
    #[token("mov")]
    Mov,

    #[token("store")]
    Store,

    #[token("load")]
    Load,

    ////////////////////////////////
    #[token("add")]
    Add,

    #[token("sub")]
    Sub,

    #[token("mul")]
    Mul,

    #[token("div")]
    Div,

    ////////////////////////////////
    /// Branching

    #[token("cmp")]
    Compare,

    #[token("jump")]
    Jump,

    #[token("jumpne")]
    JumpNotEqual,

    #[token("jumple")]
    JumpLessEqual,

    ////////////////////////////////
    #[token(".entry")]
    EntryFunction,

    #[token("print")]
    Print,

    #[token("end")]
    End,

    ////////////////////////////////

    // register names start with an 'r' followed by a number.
    #[regex(r"r[0-9]")]
    Register,

    // memory addresses are hexadecimal numbers starting with '0x'.
    #[regex(r"0x[0-9a-fA-F]+")]
    Memory,

    // immediate values are decimal numbers starting with # - eg. #123
    #[regex(r"#[0-9]+")]
    Immediate,

    // Comma and colon are literal tokens.
    #[token(",")]
    Comma,

    ////////////////////////////////

    // Comments start with a double slash and go until the end of the line. Ignore them.
    #[regex(r"//.*", logos::skip)]
    Comment,

    EndOfFile,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: std::ops::Range<usize>,
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut lexer = TokenType::lexer(&source);
        let mut tokens = Vec::new();

        loop {
            let token = match lexer.next() {
                Some(Ok(token)) => token,
                Some(Err(e)) => {
                    println!("Error: {:?}", e);
                    break;
                }
                None => break,
            };

            tokens.push(Token {
                token_type: token,
                lexeme: lexer.slice().to_string(),
                // literal: value,
                span: lexer.span(),
            });
        }

        tokens.reverse();

        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token {
            token_type: TokenType::EndOfFile,
            lexeme: String::new(),
            span: 0..0,
        })
    }

    pub fn peek(&self) -> Token {
        self.tokens
            .last()
            .clone()
            .unwrap_or(&Token {
                token_type: TokenType::EndOfFile,
                lexeme: String::new(),
                span: 0..0,
            })
            .clone()
    }
}
