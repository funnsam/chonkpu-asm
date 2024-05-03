use crate::parser::*;

pub fn post_proc(p: RawProgram<'_>) -> Result<Program, logos::Span> {
    let mut insts = Vec::new();

    for (pc, i) in p.insts.into_iter().enumerate() {
        macro_rules! expect_ops {
            ($n: tt) => {
                if i.oprs.len() != $n {
                    return Err(i.span);
                }
            };
        }

        macro_rules! expect_op {
            ($n: tt Register) => {
                if let RawOperands::Register(r) = i.oprs[$n].0 {
                    r as Register
                } else {
                    return Err(i.oprs[$n].1.clone());
                }
            };
            ($n: tt Integer) => {
                match i.oprs[$n].0 {
                    RawOperands::Integer(i) => i as Integer,
                    RawOperands::Name(n) => p.labels.iter().find(|a| a.0 == n).map_or_else(|| Err(i.oprs[$n].1.clone()), |a| Ok(a.1))? as Integer,
                    _ => return Err(i.oprs[$n].1.clone()),
                }
            };
        }

        macro_rules! offset {
            ($n: tt) => {{
                let addr = expect_op!($n Integer);
                let off = (addr as isize - pc as isize - 2) as i8;

                if off < -8 || off > 7 {
                    return Err(i.oprs[$n].1.clone());
                }

                off as u8
            }};
        }

        match i.inst.to_lowercase().as_str() {
            "ldw" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Ldw(a, b));
            },
            "stw" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Stw(a, b));
            },
            "ldwr" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Ldwr(a, b));
            },
            "stwr" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Stwr(a, b));
            },
            "bp" => insts.push(Instruction::Breakpt),
            "jmpa" => {
                expect_ops!(1);
                let a = expect_op!(0 Register);
                insts.push(Instruction::Jmpa(a));
            },
            "bnsr" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = offset!(1);
                insts.push(Instruction::Bnsr(a, b));
            },
            "jmpr" => {
                expect_ops!(1);
                let a = offset!(0);
                insts.push(Instruction::Jmpr(a));
            },
            "add" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Add(a, b));
            },
            "nor" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Nor(a, b));
            },
            "addi" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Addi(a, b));
            },
            "nori" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Nori(a, b));
            },
            "addr" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Addr(a, b));
            },
            "norr" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Register);
                insts.push(Instruction::Norr(a, b));
            },
            "addir" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Addir(a, b));
            },
            "norira" => {
                expect_ops!(2);
                let a = expect_op!(0 Register);
                let b = expect_op!(1 Integer);
                insts.push(Instruction::Norira(a, b));
            },
            _ => return Err(i.span),
        }
    }

    Ok(Program { insts })
}

#[derive(Debug)]
pub struct Program {
    pub insts: Vec<Instruction>,
}

pub type Register = u8;
pub type Integer = u8;

#[derive(Debug)]
pub enum Instruction {
    Ldw(Register, Register),
    Stw(Register, Register),
    Ldwr(Register, Integer),
    Stwr(Register, Integer),
    Breakpt,
    Jmpa(Register),
    Bnsr(Register, Integer),
    Jmpr(Integer),

    Add(Register, Register),
    Nor(Register, Register),
    Addi(Register, Integer),
    Nori(Register, Integer),
    Addr(Register, Register),
    Norr(Register, Register),
    Addir(Register, Integer),
    Norira(Register, Integer),
}
