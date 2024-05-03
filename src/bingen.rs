use crate::postproc::*;

pub fn generate(p: Program) -> Vec<u16> {
    let mut r = Vec::new();

    for i in p.insts.into_iter() {
        let (op, a, b) = match i {
            Instruction::Ldw(a, b) => (0x0, a, b),
            Instruction::Stw(a, b) => (0x1, a, b),
            Instruction::Ldwr(a, b) => (0x2, a, b),
            Instruction::Stwr(a, b) => (0x3, a, b),
            Instruction::Breakpt => (0x4, 0, 0),
            Instruction::Jmpa(b) => (0x5, 0, b),
            Instruction::Bnsr(a, b) => (0x6, a, b),
            Instruction::Jmpr(b) => (0x7, 0, b),
            Instruction::Add(a, b) => (0x8, a, b),
            Instruction::Nor(a, b) => (0x9, a, b),
            Instruction::Addi(a, b) => (0xA, a, b),
            Instruction::Nori(a, b) => (0xB, a, b),
            Instruction::Addr(a, b) => (0xC, a, b),
            Instruction::Norr(a, b) => (0xD, a, b),
            Instruction::Addir(a, b) => (0xE, a, b),
            Instruction::Norira(a, b) => (0xF, a, b),
        };

        r.push(
            ((op & 15) as u16) << 8 |
            ((a & 15) as u16) << 4 |
            ((b & 15) as u16)
        );
    }

    r
}
