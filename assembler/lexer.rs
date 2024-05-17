use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")]
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
    // eg - .entry main, parse to EntryFunction("main")
    #[token(".entry[ \t]+", |lex| lex.slice()[7..].to_string())]
    EntryFunction(String),

    #[token("print")]
    Print,

    #[token("end")]
    End,

    ////////////////////////////////

    // register names start with an 'r' followed by a number. parse it to 4-bit string
    // r0 - 0000, r1 - 0001, r2 - 0010, r3 - 0011, r4 - 0100, r5 - 0101, r6 - 0110, r7 - 0111
    // #[regex(r"r[0-9]", |lex| lex.slice()[1..].parse::<usize>().unwrap())]
    #[regex(r"r[0-9]", |lex| {
        let register_number = lex.slice()[1..].parse::<usize>().unwrap();
        format!("{:03b}", register_number)
    })]
    Register(String),

    // memory addresses are hexadecimal numbers starting with '0x' - eg. 0x123, 0xd
    #[regex(r"0x[0-9a-fA-F]+", |lex| {
        // let memory_address = lex.slice()[2..].parse::<usize>().unwrap();
        let memory_address = usize::from_str_radix(&lex.slice()[2..], 16).unwrap();
        format!("{:08b}", memory_address)
    })]
    // usize::from_str_radix(&lex.slice()[2..], 16).unwrap()
    MemoryAddress(String),

    // relative memory addresses - eg. +5, -3
    #[regex(r"[\+\-][0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    RelativeMemoryAddress(i32),

    // immediate values are decimal numbers starting with # - eg. #123 , parse it to 8-bit string
    #[regex(r"#[0-9]+", |lex| {
        let immediate = lex.slice()[1..].parse::<usize>().unwrap();
        format!("{:08b}", immediate)
    })]
    Immediate(String),

    // label are alphanumeric strings starting with a ':' - eg. ':label'
    #[regex(r":[a-zA-Z0-9_]+", |lex| lex.slice()[1..].to_string())]
    Label(String),

    // Identifiers are alphanumeric strings not starting with a number or a colon.
    // #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    // Identifier(String),

    // Comma and colon are literal tokens.
    #[token(",")]
    Comma,

    ////////////////////////////////

    // Comments start with a double slash and go until the end of the line. Ignore them.
    #[regex(r"//.*", logos::skip)]
    Comment,

    EndOfFile,
}
