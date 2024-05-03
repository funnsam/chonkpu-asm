use crate::lexer::*;
use logos::*;

pub type Label<'a> = (&'a str, usize);

pub fn parse<'a>(l: Lexer<'a, Token>) -> Result<RawProgram<'a>, Span> {
    let mut insts = Vec::new();
    let mut labels = Vec::new();
    let mut l = Peeking::from_iter(l);

    while let Some(t) = l.peek() {
        match t {
            Ok(Token::Newline) => { l.next(); },
            Ok(Token::Ident) => insts.push(parse_inst(&mut l)?),
            Ok(Token::LabelDef) => labels.push(parse_ldef(&mut l, &insts, &labels)?),
            _ => return Err(l.inner.span()),
        }
    }

    Ok(RawProgram { insts, labels })
}

fn parse_inst<'a>(l: &mut Peeking<Lexer<'a, Token>, Result<Token, ()>>) -> Result<RawInstruction<'a>, Span> {
    l.next();
    let inst = l.inner.slice();
    let mut oprs = Vec::new();
    let mut span = l.inner.span();

    while let Some(t) = l.peek() {
        match t {
            Ok(Token::Newline) => {
                l.next();
                break;
            },
            Ok(Token::Ident) => {
                l.next();
                oprs.push((RawOperands::Name(l.inner.slice()), l.inner.span()));
            },
            Ok(Token::Register(r)) => {
                l.next();
                oprs.push((RawOperands::Register(r), l.inner.span()));
            },
            Ok(Token::Integer(i)) => {
                l.next();
                oprs.push((RawOperands::Integer(i), l.inner.span()));
            },
            _ => return Err(l.inner.span()),
        }

        span.end = l.inner.span().end;
    }

    Ok(RawInstruction { inst, oprs, span })
}

fn parse_ldef<'a>(l: &mut Peeking<Lexer<'a, Token>, Result<Token, ()>>, insts: &[RawInstruction], labels: &[Label<'a>]) -> Result<Label<'a>, Span> {
    l.next();
    let lbl = l.inner.slice();
    let lbl = &lbl[..lbl.len()-1];

    if labels.iter().find(|a| a.0 == lbl).is_some() {
        return Err(l.inner.span());
    }

    Ok((lbl, insts.len()))
}

#[derive(Debug)]
pub struct RawProgram<'a> {
    pub insts: Vec<RawInstruction<'a>>,
    pub labels: Vec<Label<'a>>,
}

#[derive(Debug)]
pub struct RawInstruction<'a> {
    pub inst: &'a str,
    pub oprs: Vec<(RawOperands<'a>, Span)>,
    pub span: Span,
}

#[derive(Debug)]
pub enum RawOperands<'a> {
    Integer(isize),
    Register(usize),
    Name(&'a str),
}

struct Peeking<Inner, Item> {
    inner: Inner,
    peeked: Option<Item>,
}

impl<Inner: Iterator<Item = Item>, Item: Clone> Peeking<Inner, Item> {
    fn peek(&mut self) -> Option<Item> {
        if self.peeked.is_none() {
            let next = self.next();
            self.peeked = next.clone();
            next
        } else {
            self.peeked.clone()
        }
    }

    fn from_iter(inner: Inner) -> Self {
        Self {
            inner,
            peeked: None,
        }
    }
}

impl<Inner: Iterator<Item = Item>, Item> Iterator for Peeking<Inner, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        if self.peeked.is_some() {
            core::mem::take(&mut self.peeked)
        } else {
            self.inner.next()
        }
    }
}
