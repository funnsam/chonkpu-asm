use logos::*;

#[derive(Debug, Clone, Logos)]
#[logos(skip r"\s")]
#[logos(skip r"//.*\n")]
pub enum Token {
    #[regex(r"\n", priority = 20)]
    Newline,

    #[regex(r"[rR][\d]+", callback = |lex| lex.slice()[1..].parse::<usize>().unwrap())]
    Register(usize),

    #[regex(r"[\-]?[\d]+", callback = |lex| lex.slice().parse::<isize>().unwrap())]
    #[regex(r"0x[0-9a-fA-F]+", callback = |lex| isize::from_str_radix(&lex.slice()[2..], 16).unwrap())]
    Integer(isize),

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*\:")]
    LabelDef,

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Ident,
}
