enum Instruction {
    DAT(Modifier, Operand, Operand),
    MOV(Modifier, Operand, Operand),
    ADD(Modifier, Operand, Operand),
    SUB(Modifier, Operand, Operand),
    MUL(Modifier, Operand, Operand),
    DIV(Modifier, Operand, Operand),
    MOD(Modifier, Operand, Operand),
    JMP(Modifier, Operand, Operand),
    JMZ(Modifier, Operand, Operand),
    JMN(Modifier, Operand, Operand),
    DJN(Modifier, Operand, Operand),
    CMP(Modifier, Operand, Operand),
    SLT(Modifier, Operand, Operand),
    SPL(Modifier, Operand, Operand)
}

enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I
}

enum Operand {
    Immediate(i32),
    Direct(i32),
    Indirect(i32),
    Decrement(i32),
    Increment(i32)
}