use crate::java_class::{CodeAttribute, ConstantPoolEntry};
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    AConstNull,
    IConstM1,
    IConst0,
    IConst1,
    IConst2,
    IConst3,
    IConst4,
    IConst5,
    LConst0,
    LConst1,
    FConst0,
    FConst1,
    FConst2,
    DConst0,
    DConst1,
    BIPush(i8),
    SIPush(i16),
    Ldc(usize),
    LdcW(usize),
    Ldc2W(usize),
    // Load constant from constant pool
    ILoad(usize),
    LLoad(usize),
    FLoad(usize),
    DLoad(usize),
    ALoad(usize),
    // Load operations
    ILoad0,
    ILoad1,
    ILoad2,
    ILoad3,
    LLoad0,
    LLoad1,
    LLoad2,
    LLoad3,
    FLoad0,
    FLoad1,
    FLoad2,
    FLoad3,
    DLoad0,
    DLoad1,
    DLoad2,
    DLoad3,
    ALoad0,
    ALoad1,
    ALoad2,
    ALoad3,
    IALoad,
    LALoad,
    FALoad,
    DALoad,
    AALoad,
    BALoad,
    CALoad,
    SALoad,
    // Array operations
    IStore(usize),
    LStore(usize),
    FStore(usize),
    DStore(usize),
    AStore(usize),
    IStore0,
    IStore1,
    IStore2,
    IStore3,
    LStore0,
    LStore1,
    LStore2,
    LStore3,
    FStore0,
    FStore1,
    FStore2,
    FStore3,
    DStore0,
    DStore1,
    DStore2,
    DStore3,
    AStore0,
    AStore1,
    AStore2,
    AStore3,
    IAStore,
    LAStore,
    FAStore,
    DAStore,
    AAStore,
    BAStore,
    CAStore,
    SAStore,
    // Stack operations
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    // Add
    IAdd,
    LAdd,
    FAdd,
    DAdd,
    // Subtract
    ISub,
    LSub,
    FSub,
    DSub,
    // Multiply
    IMul,
    LMul,
    FMul,
    DMul,
    // Divide
    IDiv,
    LDiv,
    FDiv,
    DDiv,
    // Remainder
    IRem,
    LRem,
    FRem,
    DRem,
    // Negate
    INeg,
    LNeg,
    FNeg,
    DNeg,
    // Shift left
    IShl,
    LShl,
    // Shift right
    IShr,
    LShr,
    // Logical shift right
    IUShr,
    LUShr,
    // And
    IAnd,
    LAnd,
    // Or
    IOr,
    LOr,
    // Xor
    IXor,
    LXor,
    // Increment
    IInc(usize, i8),
    // Convert
    I2L,
    I2F,
    I2D,
    L2I,
    L2F,
    L2D,
    F2I,
    F2L,
    F2D,
    D2I,
    D2L,
    D2F,
    I2B,
    I2C,
    I2S,
    // Compare
    LCmp,
    FCmpL,
    FCmpG,
    DCmpL,
    DCmpG,
    // If
    IfEq(usize),
    IfNe(usize),
    IfLt(usize),
    IfGe(usize),
    IfGt(usize),
    IfLe(usize),
    IfICmpEq(usize),
    IfICmpNe(usize),
    IfICmpLt(usize),
    IfICmpGe(usize),
    IfICmpGt(usize),
    IfICmpLe(usize),
    IfACmpEq(usize),
    IfACmpNe(usize),
    // Movement
    Goto(usize),
    Jsr(usize),
    // Return
    Ret(usize),
    TableSwitch(usize, usize, usize),  //Test
    LookupSwitch(usize, usize, usize), //Test
    // Return
    IReturn,
    LReturn,
    FReturn,
    DReturn,
    AReturn,
    Return,
    // Method invocation
    GetStatic(usize),
    PutStatic(usize),
    GetField(usize),
    PutField(usize),
    InvokeVirtual(usize),
    InvokeSpecial(usize),
    InvokeStatic(usize),
    InvokeInterface(usize),
    InvokeDynamic(usize),
    // Objects
    New(usize),
    NewArray(usize),
    ANewArray(usize),
    ArrayLength,
    AThrow,
    CheckCast(usize),
    InstanceOf(usize),
    MonitorEnter,
    MonitorExit,
    Wide(usize),
    MultiANewArray(usize, usize),
    IfNull(usize),
    IfNonNull(usize),
    GotoW(usize),
    JsrW(usize),
    Breakpoint,
    Skip,
}

#[derive(Debug)]
pub enum ReducedInstruction {
    Nop,
    AConstNull,
    Const(Primitive),
    LoadConst(usize),
    // Push local variable at index onto the stack
    Load(usize, PrimitiveType),
    // Array load operations
    ALoad(PrimitiveType),
    // Store top of stack in local variable at index
    Store(usize, PrimitiveType),
    // Array store operations
    AStore(PrimitiveType),
    // Stack operations
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    // Arithmetic
    Add(PrimitiveType),
    Sub(PrimitiveType),
    Mul(PrimitiveType),
    Div(PrimitiveType),
    Rem(PrimitiveType),
    Neg(PrimitiveType),
    // Shift
    Shl(PrimitiveType),
    Shr(PrimitiveType),
    UShr(PrimitiveType),
    // Logical
    And(PrimitiveType),
    Or(PrimitiveType),
    Xor(PrimitiveType),
    // Increment
    IInc(usize, i8),
    // Convert
    Convert(PrimitiveType, PrimitiveType),
    // Compare
    // TODO: Perhaps these could be reduced to a single instruction?
    LCmp,
    FCmpL,
    FCmpG,
    DCmpL,
    DCmpG,
    // If
    If(usize, Comparison),
    IfICmp(usize, Comparison),
    // Movement
    Goto(usize),
    Jsr(usize),
    Ret(usize),
    // Table switch
    // TODO: Properly implement this.
    TableSwitch(usize, usize, usize),
    LookupSwitch(usize, usize, usize),
    // Return
    Return(PrimitiveType),
    // Method invocation
    GetStatic(usize),
    PutStatic(usize),
    GetField(usize),
    PutField(usize),
    InvokeVirtual(usize),
    InvokeSpecial(usize),
    InvokeStatic(usize),
    InvokeInterface(usize),
    InvokeDynamic(usize),
    // Objects
    New(usize),
    NewArray(usize),
    ANewArray(usize),
    ArrayLength,
    AThrow,
    CheckCast(usize),
    InstanceOf(usize),
    MonitorEnter,
    MonitorExit,
    Wide(usize),
    MultiANewArray(usize, usize),
    IfNull(usize),
    IfNonNull(usize),
    GotoW(usize),
    JsrW(usize),
    Breakpoint,
}

#[derive(Debug)]
pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Shl,
    Shr,
    UShr,
    And,
    Or,
    Xor,
    Convert(PrimitiveType, PrimitiveType),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Null,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference,
}

#[derive(Debug)]
pub enum Primitive {
    Null,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Reference(usize),
    ReturnAddress(usize),
}

impl Primitive {
    pub fn make_copy(&self) -> Primitive {
        match self {
            Primitive::Null => Primitive::Null,
            Primitive::Byte(b) => Primitive::Byte(*b),
            Primitive::Short(s) => Primitive::Short(*s),
            Primitive::Int(i) => Primitive::Int(*i),
            Primitive::Long(l) => Primitive::Long(*l),
            Primitive::Char(c) => Primitive::Char(*c),
            Primitive::Float(f) => Primitive::Float(*f),
            Primitive::Double(d) => Primitive::Double(*d),
            Primitive::Boolean(b) => Primitive::Boolean(*b),
            Primitive::Reference(r) => Primitive::Reference(*r),
            Primitive::ReturnAddress(ra) => Primitive::ReturnAddress(*ra),
        }
    }

    pub fn eval(a: Primitive, o: Operation) -> Primitive {
        match o {
            Operation::Neg => match a {
                Primitive::Int(i) => Primitive::Int(-i),
                Primitive::Long(l) => Primitive::Long(-l),
                Primitive::Float(f) => Primitive::Float(-f),
                Primitive::Double(d) => Primitive::Double(-d),
                _ => panic!("Unsupported operation"),
            },
            Operation::Convert(source, destination) => match (a, source) {
                (Primitive::Int(i), PrimitiveType::Int) => match destination {
                    PrimitiveType::Byte => Primitive::Byte(i as i8),
                    PrimitiveType::Short => Primitive::Short(i as i16),
                    PrimitiveType::Char => Primitive::Char(i as u16),
                    PrimitiveType::Long => Primitive::Long(i as i64),
                    PrimitiveType::Float => Primitive::Float(i as f32),
                    PrimitiveType::Double => Primitive::Double(i as f64),
                    _ => panic!("cannot convert int to passed type"),
                },
                (Primitive::Long(l), PrimitiveType::Long) => match destination {
                    PrimitiveType::Int => Primitive::Int(l as i32),
                    PrimitiveType::Float => Primitive::Float(l as f32),
                    PrimitiveType::Double => Primitive::Double(l as f64),
                    _ => panic!("cannot convert long to passed type"),
                },
                (Primitive::Float(f), PrimitiveType::Float) => match destination {
                    PrimitiveType::Int => Primitive::Int(f as i32),
                    PrimitiveType::Long => Primitive::Long(f as i64),
                    PrimitiveType::Double => Primitive::Double(f as f64),
                    _ => panic!("cannot convert float to passed type"),
                },
                (Primitive::Double(d), PrimitiveType::Double) => match destination {
                    PrimitiveType::Int => Primitive::Int(d as i32),
                    PrimitiveType::Long => Primitive::Long(d as i64),
                    PrimitiveType::Float => Primitive::Float(d as f32),
                    _ => panic!("cannot convert double to passed type"),
                },
                _ => panic!("unsupported conversion or incorrect type passed"),
            },

            _ => panic!("Unsupported operation"),
        }
    }

    pub fn eval2(a: Primitive, b: Primitive, o: Operation) -> Primitive {
        match o {
            Operation::Add => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i + j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l + j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f + j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d + j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Sub => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i - j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l - j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f - j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d - j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Mul => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i * j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l * j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f * j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d * j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Div => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i / j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l / j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f / j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d / j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Rem => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i % j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l % j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f % j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d % j),
                _ => panic!("Unsupported operation"),
            },
            Operation::And => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i & j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l & j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Or => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i | j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l | j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Xor => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i ^ j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l ^ j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Shl => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i << j),
                (Primitive::Long(l), Primitive::Int(j)) => Primitive::Long(l << j),
                _ => panic!("Unsupported operation"),
            },
            Operation::Shr => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i >> j),
                (Primitive::Long(l), Primitive::Int(j)) => Primitive::Long(l >> j),
                _ => panic!("Unsupported operation"),
            },
            Operation::UShr => match (a, b) {
                // TODO: implement unsigned (or logical?) shift correctly
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i >> j),
                (Primitive::Long(l), Primitive::Int(j)) => Primitive::Long(l >> j),
                _ => panic!("Unsupported operation"),
            },
            _ => panic!("unsupported operation"),
        }
    }

    pub fn is_wide(&self) -> bool {
        matches!(self, Primitive::Long(_) | Primitive::Double(_))
    }
}

#[derive(Debug)]
pub struct Bytecode {
    pub pc: usize,
    pub instructions: Vec<ReducedInstruction>,
    pub stack: Vec<Primitive>,
    pub local_variables: Vec<Primitive>,
}

impl Bytecode {
    fn u1(code: &[u8], pc: &mut usize) -> usize {
        let b = code[*pc + 1];
        *pc += 1;
        b as usize
    }

    fn u2(code: &[u8], pc: &mut usize) -> usize {
        let b1 = code[*pc + 1];
        let b2 = code[*pc + 2];
        *pc += 2;
        (((b1 as i16) << 8) | (b2 as i16)) as usize
    }

    fn u4(code: &[u8], pc: &mut usize) -> usize {
        let b1 = code[*pc + 1];
        let b2 = code[*pc + 2];
        let b3 = code[*pc + 3];
        let b4 = code[*pc + 4];
        *pc += 4;
        (((b1 as i32) << 24) | ((b2 as i32) << 16) | ((b3 as i32) << 8) | (b4 as i32)) as usize
    }

    pub fn new(code: Vec<u8>) -> Bytecode {
        let mut instructions: Vec<ReducedInstruction> = Vec::new();
        let mut pc: usize = 0;
        let mut past_byte_pos: usize = 0;

        // while byte_pos < code.len() as usize {
        //     instructions.push(match code[byte_pos] {
        //         0 => Instruction::Nop,
        //         1 => Instruction::AConstNull,
        //         2 => Instruction::IConstM1,
        //         3 => Instruction::IConst0,
        //         4 => Instruction::IConst1,
        //         5 => Instruction::IConst2,
        //         6 => Instruction::IConst3,
        //         7 => Instruction::IConst4,
        //         8 => Instruction::IConst5,
        //         9 => Instruction::LConst0,
        //         10 => Instruction::LConst1,
        //         11 => Instruction::FConst0,
        //         12 => Instruction::FConst1,
        //         13 => Instruction::FConst2,
        //         14 => Instruction::DConst0,
        //         15 => Instruction::DConst1,
        //         16 => Instruction::BIPush(Bytecode::get_byte(&code, &mut byte_pos)),
        //         17 => Instruction::SIPush(Bytecode::get_short(&code, &mut byte_pos)),
        //         18 => Instruction::Ldc(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         19 => Instruction::LdcW(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         20 => Instruction::Ldc2W(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         21 => Instruction::ILoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         22 => Instruction::LLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         23 => Instruction::FLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         24 => Instruction::DLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         25 => Instruction::ALoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         26 => Instruction::ILoad0,
        //         27 => Instruction::ILoad1,
        //         28 => Instruction::ILoad2,
        //         29 => Instruction::ILoad3,
        //         30 => Instruction::LLoad0,
        //         31 => Instruction::LLoad1,
        //         32 => Instruction::LLoad2,
        //         33 => Instruction::LLoad3,
        //         34 => Instruction::FLoad0,
        //         35 => Instruction::FLoad1,
        //         36 => Instruction::FLoad2,
        //         37 => Instruction::FLoad3,
        //         38 => Instruction::DLoad0,
        //         39 => Instruction::DLoad1,
        //         40 => Instruction::DLoad2,
        //         41 => Instruction::DLoad3,
        //         42 => Instruction::ALoad0,
        //         43 => Instruction::ALoad1,
        //         44 => Instruction::ALoad2,
        //         45 => Instruction::ALoad3,
        //         46 => Instruction::IALoad,
        //         47 => Instruction::LALoad,
        //         48 => Instruction::FALoad,
        //         49 => Instruction::DALoad,
        //         50 => Instruction::AALoad,
        //         51 => Instruction::BALoad,
        //         52 => Instruction::CALoad,
        //         53 => Instruction::SALoad,
        //         54 => Instruction::IStore(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         55 => Instruction::LStore(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         56 => Instruction::FStore(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         57 => Instruction::DStore(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         58 => Instruction::AStore(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         59 => Instruction::IStore0,
        //         60 => Instruction::IStore1,
        //         61 => Instruction::IStore2,
        //         62 => Instruction::IStore3,
        //         63 => Instruction::LStore0,
        //         64 => Instruction::LStore1,
        //         65 => Instruction::LStore2,
        //         66 => Instruction::LStore3,
        //         67 => Instruction::FStore0,
        //         68 => Instruction::FStore1,
        //         69 => Instruction::FStore2,
        //         70 => Instruction::FStore3,
        //         71 => Instruction::DStore0,
        //         72 => Instruction::DStore1,
        //         73 => Instruction::DStore2,
        //         74 => Instruction::DStore3,
        //         75 => Instruction::AStore0,
        //         76 => Instruction::AStore1,
        //         77 => Instruction::AStore2,
        //         78 => Instruction::AStore3,
        //         79 => Instruction::IAStore,
        //         80 => Instruction::LAStore,
        //         81 => Instruction::FAStore,
        //         82 => Instruction::DAStore,
        //         83 => Instruction::AAStore,
        //         84 => Instruction::BAStore,
        //         85 => Instruction::CAStore,
        //         86 => Instruction::SAStore,
        //         87 => Instruction::Pop,
        //         88 => Instruction::Pop2,
        //         89 => Instruction::Dup,
        //         90 => Instruction::DupX1,
        //         91 => Instruction::DupX2,
        //         92 => Instruction::Dup2,
        //         93 => Instruction::Dup2X1,
        //         94 => Instruction::Dup2X2,
        //         95 => Instruction::Swap,
        //         96 => Instruction::IAdd,
        //         97 => Instruction::LAdd,
        //         98 => Instruction::FAdd,
        //         99 => Instruction::DAdd,
        //         100 => Instruction::ISub,
        //         101 => Instruction::LSub,
        //         102 => Instruction::FSub,
        //         103 => Instruction::DSub,
        //         104 => Instruction::IMul,
        //         105 => Instruction::LMul,
        //         106 => Instruction::FMul,
        //         107 => Instruction::DMul,
        //         108 => Instruction::IDiv,
        //         109 => Instruction::LDiv,
        //         110 => Instruction::FDiv,
        //         111 => Instruction::DDiv,
        //         112 => Instruction::IRem,
        //         113 => Instruction::LRem,
        //         114 => Instruction::FRem,
        //         115 => Instruction::DRem,
        //         116 => Instruction::INeg,
        //         117 => Instruction::LNeg,
        //         118 => Instruction::FNeg,
        //         119 => Instruction::DNeg,
        //         120 => Instruction::IShl,
        //         121 => Instruction::LShl,
        //         122 => Instruction::IShr,
        //         123 => Instruction::LShr,
        //         124 => Instruction::IUShr,
        //         125 => Instruction::LUShr,
        //         126 => Instruction::IAnd,
        //         127 => Instruction::LAnd,
        //         128 => Instruction::IOr,
        //         129 => Instruction::LOr,
        //         130 => Instruction::IXor,
        //         131 => Instruction::LXor,
        //         132 => Instruction::IInc(
        //             Bytecode::get_byte(&code, &mut byte_pos) as usize,
        //             Bytecode::get_byte(&code, &mut byte_pos),
        //         ),
        //         133 => Instruction::I2L,
        //         134 => Instruction::I2F,
        //         135 => Instruction::I2D,
        //         136 => Instruction::L2I,
        //         137 => Instruction::L2F,
        //         138 => Instruction::L2D,
        //         139 => Instruction::F2I,
        //         140 => Instruction::F2L,
        //         141 => Instruction::F2D,
        //         142 => Instruction::D2I,
        //         143 => Instruction::D2L,
        //         144 => Instruction::D2F,
        //         145 => Instruction::I2B,
        //         146 => Instruction::I2C,
        //         147 => Instruction::I2S,
        //         148 => Instruction::LCmp,
        //         149 => Instruction::FCmpL,
        //         150 => Instruction::FCmpG,
        //         151 => Instruction::DCmpL,
        //         152 => Instruction::DCmpG,
        //         153 => Instruction::IfEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         154 => Instruction::IfNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         155 => Instruction::IfLt(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         156 => Instruction::IfGe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         157 => Instruction::IfGt(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         158 => Instruction::IfLe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         159 => Instruction::IfICmpEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         160 => Instruction::IfICmpNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         161 => Instruction::IfICmpLt(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         162 => Instruction::IfICmpGe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         163 => Instruction::IfICmpGt(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         164 => Instruction::IfICmpLe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         165 => Instruction::IfACmpEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         166 => Instruction::IfACmpNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         167 => Instruction::Goto(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         168 => Instruction::Jsr(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         169 => Instruction::Ret(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         170 => panic!("Unsupported instruction: {}", 170),
        //         171 => panic!("Unsupported instruction: {}", 171),
        //         172 => Instruction::IReturn,
        //         173 => Instruction::LReturn,
        //         174 => Instruction::FReturn,
        //         175 => Instruction::DReturn,
        //         176 => Instruction::AReturn,
        //         177 => Instruction::Return,
        //         178 => Instruction::GetStatic(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         179 => Instruction::PutStatic(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         180 => Instruction::GetField(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         181 => Instruction::PutField(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         182 => {
        //             Instruction::InvokeVirtual(Bytecode::get_short(&code, &mut byte_pos) as usize)
        //         }
        //         183 => {
        //             Instruction::InvokeSpecial(Bytecode::get_short(&code, &mut byte_pos) as usize)
        //         }
        //         184 => {
        //             Instruction::InvokeStatic(Bytecode::get_short(&code, &mut byte_pos) as usize)
        //         }
        //         185 => {
        //             Instruction::InvokeInterface(Bytecode::get_short(&code, &mut byte_pos) as usize)
        //         }
        //         186 => {
        //             Instruction::InvokeDynamic(Bytecode::get_short(&code, &mut byte_pos) as usize)
        //         }
        //         187 => Instruction::New(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         188 => Instruction::NewArray(Bytecode::get_byte(&code, &mut byte_pos) as usize),
        //         189 => Instruction::ANewArray(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         190 => Instruction::ArrayLength,
        //         191 => Instruction::AThrow,
        //         192 => Instruction::CheckCast(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         193 => Instruction::InstanceOf(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         194 => Instruction::MonitorEnter,
        //         195 => Instruction::MonitorExit,
        //         196 => panic!("Unsupported instruction: {}", 196),
        //         197 => panic!("Unsupported instruction: {}", 197),
        //         198 => Instruction::IfNull(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         199 => Instruction::IfNonNull(Bytecode::get_short(&code, &mut byte_pos) as usize),
        //         200 => Instruction::GotoW(Bytecode::get_int(&code, &mut byte_pos) as usize),
        //         201 => Instruction::JsrW(Bytecode::get_int(&code, &mut byte_pos) as usize),
        //         202 => Instruction::Breakpoint,
        //         _ => panic!("unsupported instruction"),
        //     });
        //
        //     for _ in past_byte_pos..byte_pos {
        //         instructions.push(Instruction::Nop);
        //     }
        //
        //     byte_pos += 1;
        //     past_byte_pos = byte_pos;
        // }

        while pc < code.len() as usize {
            instructions.push(match code[pc] {
                0 => ReducedInstruction::Nop,
                1 => ReducedInstruction::AConstNull,
                2 => ReducedInstruction::Const(Primitive::Int(-1)),
                3 => ReducedInstruction::Const(Primitive::Int(0)),
                4 => ReducedInstruction::Const(Primitive::Int(1)),
                5 => ReducedInstruction::Const(Primitive::Int(2)),
                6 => ReducedInstruction::Const(Primitive::Int(3)),
                7 => ReducedInstruction::Const(Primitive::Int(4)),
                8 => ReducedInstruction::Const(Primitive::Int(5)),
                9 => ReducedInstruction::Const(Primitive::Long(0)),
                10 => ReducedInstruction::Const(Primitive::Long(1)),
                11 => ReducedInstruction::Const(Primitive::Float(0.0)),
                12 => ReducedInstruction::Const(Primitive::Float(1.0)),
                13 => ReducedInstruction::Const(Primitive::Float(2.0)),
                14 => ReducedInstruction::Const(Primitive::Double(0.0)),
                15 => ReducedInstruction::Const(Primitive::Double(1.0)),
                16 => {
                    ReducedInstruction::Const(Primitive::Int(Bytecode::u1(&code, &mut pc) as i32))
                }
                17 => {
                    ReducedInstruction::Const(Primitive::Int(Bytecode::u2(&code, &mut pc) as i32))
                }
                18 => ReducedInstruction::LoadConst(Bytecode::u1(&code, &mut pc)),
                19 => ReducedInstruction::LoadConst(Bytecode::u2(&code, &mut pc)),
                20 => ReducedInstruction::LoadConst(Bytecode::u2(&code, &mut pc)),
                21 => ReducedInstruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Int),
                22 => ReducedInstruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Long),
                23 => ReducedInstruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Float),
                24 => ReducedInstruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Double),
                25 => {
                    ReducedInstruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Reference)
                }
                26 => ReducedInstruction::Load(0, PrimitiveType::Int),
                27 => ReducedInstruction::Load(1, PrimitiveType::Int),
                28 => ReducedInstruction::Load(2, PrimitiveType::Int),
                29 => ReducedInstruction::Load(3, PrimitiveType::Int),
                30 => ReducedInstruction::Load(0, PrimitiveType::Long),
                31 => ReducedInstruction::Load(1, PrimitiveType::Long),
                32 => ReducedInstruction::Load(2, PrimitiveType::Long),
                33 => ReducedInstruction::Load(3, PrimitiveType::Long),
                34 => ReducedInstruction::Load(0, PrimitiveType::Float),
                35 => ReducedInstruction::Load(1, PrimitiveType::Float),
                36 => ReducedInstruction::Load(2, PrimitiveType::Float),
                37 => ReducedInstruction::Load(3, PrimitiveType::Float),
                38 => ReducedInstruction::Load(0, PrimitiveType::Double),
                39 => ReducedInstruction::Load(1, PrimitiveType::Double),
                40 => ReducedInstruction::Load(2, PrimitiveType::Double),
                41 => ReducedInstruction::Load(3, PrimitiveType::Double),
                42 => ReducedInstruction::Load(0, PrimitiveType::Reference),
                43 => ReducedInstruction::Load(1, PrimitiveType::Reference),
                44 => ReducedInstruction::Load(2, PrimitiveType::Reference),
                45 => ReducedInstruction::Load(3, PrimitiveType::Reference),
                46 => ReducedInstruction::ALoad(PrimitiveType::Int),
                47 => ReducedInstruction::ALoad(PrimitiveType::Long),
                48 => ReducedInstruction::ALoad(PrimitiveType::Float),
                49 => ReducedInstruction::ALoad(PrimitiveType::Double),
                50 => ReducedInstruction::ALoad(PrimitiveType::Reference),
                51 => ReducedInstruction::ALoad(PrimitiveType::Byte),
                52 => ReducedInstruction::ALoad(PrimitiveType::Char),
                53 => ReducedInstruction::ALoad(PrimitiveType::Short),
                54 => ReducedInstruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Int),
                55 => ReducedInstruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Long),
                56 => ReducedInstruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Float),
                57 => {
                    ReducedInstruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Double)
                }
                58 => ReducedInstruction::Store(
                    Bytecode::u1(&code, &mut pc),
                    PrimitiveType::Reference,
                ),
                59 => ReducedInstruction::Store(0, PrimitiveType::Int),
                60 => ReducedInstruction::Store(1, PrimitiveType::Int),
                61 => ReducedInstruction::Store(2, PrimitiveType::Int),
                62 => ReducedInstruction::Store(3, PrimitiveType::Int),
                63 => ReducedInstruction::Store(0, PrimitiveType::Long),
                64 => ReducedInstruction::Store(1, PrimitiveType::Long),
                65 => ReducedInstruction::Store(2, PrimitiveType::Long),
                66 => ReducedInstruction::Store(3, PrimitiveType::Long),
                67 => ReducedInstruction::Store(0, PrimitiveType::Float),
                68 => ReducedInstruction::Store(1, PrimitiveType::Float),
                69 => ReducedInstruction::Store(2, PrimitiveType::Float),
                70 => ReducedInstruction::Store(3, PrimitiveType::Float),
                71 => ReducedInstruction::Store(0, PrimitiveType::Double),
                72 => ReducedInstruction::Store(1, PrimitiveType::Double),
                73 => ReducedInstruction::Store(2, PrimitiveType::Double),
                74 => ReducedInstruction::Store(3, PrimitiveType::Double),
                75 => ReducedInstruction::Store(0, PrimitiveType::Reference),
                76 => ReducedInstruction::Store(1, PrimitiveType::Reference),
                77 => ReducedInstruction::Store(2, PrimitiveType::Reference),
                78 => ReducedInstruction::Store(3, PrimitiveType::Reference),
                79 => ReducedInstruction::AStore(PrimitiveType::Int),
                80 => ReducedInstruction::AStore(PrimitiveType::Long),
                81 => ReducedInstruction::AStore(PrimitiveType::Float),
                82 => ReducedInstruction::AStore(PrimitiveType::Double),
                83 => ReducedInstruction::AStore(PrimitiveType::Reference),
                84 => ReducedInstruction::AStore(PrimitiveType::Byte),
                85 => ReducedInstruction::AStore(PrimitiveType::Char),
                86 => ReducedInstruction::AStore(PrimitiveType::Short),
                87 => ReducedInstruction::Pop,
                88 => ReducedInstruction::Pop2,
                89 => ReducedInstruction::Dup,
                90 => ReducedInstruction::DupX1,
                91 => ReducedInstruction::DupX2,
                92 => ReducedInstruction::Dup2,
                93 => ReducedInstruction::Dup2X1,
                94 => ReducedInstruction::Dup2X2,
                95 => ReducedInstruction::Swap,
                96 => ReducedInstruction::Add(PrimitiveType::Int),
                97 => ReducedInstruction::Add(PrimitiveType::Long),
                98 => ReducedInstruction::Add(PrimitiveType::Float),
                99 => ReducedInstruction::Add(PrimitiveType::Double),
                100 => ReducedInstruction::Sub(PrimitiveType::Int),
                101 => ReducedInstruction::Sub(PrimitiveType::Long),
                102 => ReducedInstruction::Sub(PrimitiveType::Float),
                103 => ReducedInstruction::Sub(PrimitiveType::Double),
                104 => ReducedInstruction::Mul(PrimitiveType::Int),
                105 => ReducedInstruction::Mul(PrimitiveType::Long),
                106 => ReducedInstruction::Mul(PrimitiveType::Float),
                107 => ReducedInstruction::Mul(PrimitiveType::Double),
                108 => ReducedInstruction::Div(PrimitiveType::Int),
                109 => ReducedInstruction::Div(PrimitiveType::Long),
                110 => ReducedInstruction::Div(PrimitiveType::Float),
                111 => ReducedInstruction::Div(PrimitiveType::Double),
                112 => ReducedInstruction::Rem(PrimitiveType::Int),
                113 => ReducedInstruction::Rem(PrimitiveType::Long),
                114 => ReducedInstruction::Rem(PrimitiveType::Float),
                115 => ReducedInstruction::Rem(PrimitiveType::Double),
                116 => ReducedInstruction::Neg(PrimitiveType::Int),
                117 => ReducedInstruction::Neg(PrimitiveType::Long),
                118 => ReducedInstruction::Neg(PrimitiveType::Float),
                119 => ReducedInstruction::Neg(PrimitiveType::Double),
                120 => ReducedInstruction::Shl(PrimitiveType::Int),
                121 => ReducedInstruction::Shl(PrimitiveType::Long),
                122 => ReducedInstruction::Shr(PrimitiveType::Int),
                123 => ReducedInstruction::Shr(PrimitiveType::Long),
                124 => ReducedInstruction::UShr(PrimitiveType::Int),
                125 => ReducedInstruction::UShr(PrimitiveType::Long),
                126 => ReducedInstruction::And(PrimitiveType::Int),
                127 => ReducedInstruction::And(PrimitiveType::Long),
                128 => ReducedInstruction::Or(PrimitiveType::Int),
                129 => ReducedInstruction::Or(PrimitiveType::Long),
                130 => ReducedInstruction::Xor(PrimitiveType::Int),
                131 => ReducedInstruction::Xor(PrimitiveType::Long),
                132 => ReducedInstruction::IInc(
                    Bytecode::u1(&code, &mut pc),
                    Bytecode::u1(&code, &mut pc) as i8,
                ),
                133 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Long),
                134 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Float),
                135 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Double),
                136 => ReducedInstruction::Convert(PrimitiveType::Long, PrimitiveType::Int),
                137 => ReducedInstruction::Convert(PrimitiveType::Long, PrimitiveType::Float),
                138 => ReducedInstruction::Convert(PrimitiveType::Long, PrimitiveType::Double),
                139 => ReducedInstruction::Convert(PrimitiveType::Float, PrimitiveType::Int),
                140 => ReducedInstruction::Convert(PrimitiveType::Float, PrimitiveType::Long),
                141 => ReducedInstruction::Convert(PrimitiveType::Float, PrimitiveType::Double),
                142 => ReducedInstruction::Convert(PrimitiveType::Double, PrimitiveType::Int),
                143 => ReducedInstruction::Convert(PrimitiveType::Double, PrimitiveType::Long),
                144 => ReducedInstruction::Convert(PrimitiveType::Double, PrimitiveType::Float),
                145 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Byte),
                146 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Char),
                147 => ReducedInstruction::Convert(PrimitiveType::Int, PrimitiveType::Short),
                148 => ReducedInstruction::LCmp,
                149 => ReducedInstruction::FCmpL,
                150 => ReducedInstruction::FCmpG,
                151 => ReducedInstruction::DCmpL,
                152 => ReducedInstruction::DCmpG,
                153 => ReducedInstruction::If(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                154 => ReducedInstruction::If(Bytecode::u2(&code, &mut pc), Comparison::NotEqual),
                155 => ReducedInstruction::If(Bytecode::u2(&code, &mut pc), Comparison::LessThan),
                156 => ReducedInstruction::If(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::GreaterThanOrEqual,
                ),
                157 => {
                    ReducedInstruction::If(Bytecode::u2(&code, &mut pc), Comparison::GreaterThan)
                }
                158 => ReducedInstruction::If(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::LessThanOrEqual,
                ),
                159 => ReducedInstruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                160 => {
                    ReducedInstruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::NotEqual)
                }
                161 => {
                    ReducedInstruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::LessThan)
                }
                162 => ReducedInstruction::IfICmp(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::GreaterThanOrEqual,
                ),
                163 => ReducedInstruction::IfICmp(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::GreaterThan,
                ),
                164 => ReducedInstruction::IfICmp(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::LessThanOrEqual,
                ),
                165 => ReducedInstruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                166 => {
                    ReducedInstruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::NotEqual)
                }
                167 => ReducedInstruction::Goto(Bytecode::u2(&code, &mut pc)),
                168 => ReducedInstruction::Jsr(Bytecode::u2(&code, &mut pc)),
                169 => ReducedInstruction::Ret(Bytecode::u1(&code, &mut pc)),
                170 => panic!("Unsupported instruction: {}", 170),
                171 => panic!("Unsupported instruction: {}", 171),
                172 => ReducedInstruction::Return(PrimitiveType::Int),
                173 => ReducedInstruction::Return(PrimitiveType::Long),
                174 => ReducedInstruction::Return(PrimitiveType::Float),
                175 => ReducedInstruction::Return(PrimitiveType::Double),
                176 => ReducedInstruction::Return(PrimitiveType::Reference),
                177 => ReducedInstruction::Return(PrimitiveType::Null),
                178 => ReducedInstruction::GetStatic(Bytecode::u2(&code, &mut pc) as usize),
                179 => ReducedInstruction::PutStatic(Bytecode::u2(&code, &mut pc) as usize),
                180 => ReducedInstruction::GetField(Bytecode::u2(&code, &mut pc) as usize),
                181 => ReducedInstruction::PutField(Bytecode::u2(&code, &mut pc) as usize),
                182 => ReducedInstruction::InvokeVirtual(Bytecode::u2(&code, &mut pc) as usize),
                183 => ReducedInstruction::InvokeSpecial(Bytecode::u2(&code, &mut pc) as usize),
                184 => ReducedInstruction::InvokeStatic(Bytecode::u2(&code, &mut pc) as usize),
                185 => ReducedInstruction::InvokeInterface(Bytecode::u2(&code, &mut pc) as usize),
                186 => ReducedInstruction::InvokeDynamic(Bytecode::u2(&code, &mut pc) as usize),
                187 => ReducedInstruction::New(Bytecode::u2(&code, &mut pc) as usize),
                188 => ReducedInstruction::NewArray(Bytecode::u1(&code, &mut pc) as usize),
                189 => ReducedInstruction::ANewArray(Bytecode::u2(&code, &mut pc) as usize),
                190 => ReducedInstruction::ArrayLength,
                191 => ReducedInstruction::AThrow,
                192 => ReducedInstruction::CheckCast(Bytecode::u2(&code, &mut pc) as usize),
                193 => ReducedInstruction::InstanceOf(Bytecode::u2(&code, &mut pc) as usize),
                194 => ReducedInstruction::MonitorEnter,
                195 => ReducedInstruction::MonitorExit,
                196 => panic!("Unsupported instruction: {}", 196),
                197 => panic!("Unsupported instruction: {}", 197),
                198 => ReducedInstruction::IfNull(Bytecode::u2(&code, &mut pc) as usize),
                199 => ReducedInstruction::IfNonNull(Bytecode::u2(&code, &mut pc) as usize),
                200 => ReducedInstruction::GotoW(Bytecode::u4(&code, &mut pc) as usize),
                201 => ReducedInstruction::JsrW(Bytecode::u4(&code, &mut pc) as usize),
                202 => ReducedInstruction::Breakpoint,
                _ => panic!("unsupported instruction"),
            });

            for _ in past_byte_pos..pc {
                instructions.push(ReducedInstruction::Nop);
            }

            pc += 1;
            past_byte_pos = pc;
        }

        Bytecode {
            pc: 0,
            instructions,
            stack: Vec::with_capacity(5),
            local_variables: Vec::with_capacity(5),
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.instructions.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let instruction = &self.instructions[self.pc];

        // TODO: Implement array operations

        // match instruction {
        //     Instruction::Nop => {}
        //     Instruction::AConstNull => self.stack.push(Primitive::Reference(0)),
        //     Instruction::IConstM1 => self.stack.push(Primitive::Int(-1)),
        //     Instruction::IConst0 => self.stack.push(Primitive::Int(0)),
        //     Instruction::IConst1 => self.stack.push(Primitive::Int(1)),
        //     Instruction::IConst2 => self.stack.push(Primitive::Int(2)),
        //     Instruction::IConst3 => self.stack.push(Primitive::Int(3)),
        //     Instruction::IConst4 => self.stack.push(Primitive::Int(4)),
        //     Instruction::IConst5 => self.stack.push(Primitive::Int(5)),
        //     Instruction::LConst0 => self.stack.push(Primitive::Long(0)),
        //     Instruction::LConst1 => self.stack.push(Primitive::Long(1)),
        //     Instruction::FConst0 => self.stack.push(Primitive::Float(0.0)),
        //     Instruction::FConst1 => self.stack.push(Primitive::Float(1.0)),
        //     Instruction::FConst2 => self.stack.push(Primitive::Float(2.0)),
        //     Instruction::DConst0 => self.stack.push(Primitive::Double(0.0)),
        //     Instruction::DConst1 => self.stack.push(Primitive::Double(1.0)),
        //     Instruction::BIPush(byte) => {
        //         self.stack.push(Primitive::Int(*byte as i32));
        //         self.pc += 1;
        //     }
        //     Instruction::SIPush(short) => {
        //         self.stack.push(Primitive::Int(*short as i32));
        //         self.pc += 2;
        //     }
        //     Instruction::Ldc(index) => {
        //         panic!("Ldc not implemented")
        //     }
        //     Instruction::LdcW(index) => {
        //         panic!("LdcW not implemented")
        //     }
        //     Instruction::Ldc2W(index) => {
        //         panic!("Ldc2W not implemented")
        //     }
        //     Instruction::ILoad(index) => {
        //         self.stack
        //             .push(self.local_variables[*index as usize].make_copy());
        //         self.pc += 1;
        //     }
        //     Instruction::LLoad(index) => {
        //         self.stack
        //             .push(self.local_variables[*index as usize].make_copy());
        //         self.pc += 1;
        //     }
        //     Instruction::FLoad(index) => {
        //         self.stack
        //             .push(self.local_variables[*index as usize].make_copy());
        //         self.pc += 1;
        //     }
        //     Instruction::DLoad(index) => {
        //         self.stack
        //             .push(self.local_variables[*index as usize].make_copy());
        //         self.pc += 1;
        //     }
        //     Instruction::ALoad(index) => {
        //         self.stack
        //             .push(self.local_variables[*index as usize].make_copy());
        //         self.pc += 1;
        //     }
        //     Instruction::ILoad0 => {
        //         self.stack.push(self.local_variables[0].make_copy());
        //     }
        //     Instruction::ILoad1 => {
        //         self.stack.push(self.local_variables[1].make_copy());
        //     }
        //     Instruction::ILoad2 => {
        //         self.stack.push(self.local_variables[2].make_copy());
        //     }
        //     Instruction::ILoad3 => {
        //         self.stack.push(self.local_variables[3].make_copy());
        //     }
        //     Instruction::LLoad0 => {
        //         self.stack.push(self.local_variables[0].make_copy());
        //     }
        //     Instruction::LLoad1 => {
        //         self.stack.push(self.local_variables[1].make_copy());
        //     }
        //     Instruction::LLoad2 => {
        //         self.stack.push(self.local_variables[2].make_copy());
        //     }
        //     Instruction::LLoad3 => {
        //         self.stack.push(self.local_variables[3].make_copy());
        //     }
        //     Instruction::FLoad0 => {
        //         self.stack.push(self.local_variables[0].make_copy());
        //     }
        //     Instruction::FLoad1 => {
        //         self.stack.push(self.local_variables[1].make_copy());
        //     }
        //     Instruction::FLoad2 => {
        //         self.stack.push(self.local_variables[2].make_copy());
        //     }
        //     Instruction::FLoad3 => {
        //         self.stack.push(self.local_variables[3].make_copy());
        //     }
        //     Instruction::DLoad0 => {
        //         self.stack.push(self.local_variables[0].make_copy());
        //     }
        //     Instruction::DLoad1 => {
        //         self.stack.push(self.local_variables[1].make_copy());
        //     }
        //     Instruction::DLoad2 => {
        //         self.stack.push(self.local_variables[2].make_copy());
        //     }
        //     Instruction::DLoad3 => {
        //         self.stack.push(self.local_variables[3].make_copy());
        //     }
        //     Instruction::ALoad0 => {
        //         self.stack.push(self.local_variables[0].make_copy());
        //     }
        //     Instruction::ALoad1 => {
        //         self.stack.push(self.local_variables[1].make_copy());
        //     }
        //     Instruction::ALoad2 => {
        //         self.stack.push(self.local_variables[2].make_copy());
        //     }
        //     Instruction::ALoad3 => {
        //         self.stack.push(self.local_variables[3].make_copy());
        //     }
        //     Instruction::IALoad => {
        //         panic!("IALoad not implemented")
        //     }
        //     Instruction::LALoad => {
        //         panic!("LALoad not implemented")
        //     }
        //     Instruction::FALoad => {
        //         panic!("FALoad not implemented")
        //     }
        //     Instruction::DALoad => {
        //         panic!("DALoad not implemented")
        //     }
        //     Instruction::AALoad => {
        //         panic!("AALoad not implemented")
        //     }
        //     Instruction::BALoad => {
        //         panic!("BALoad not implemented")
        //     }
        //     Instruction::CALoad => {
        //         panic!("CALoad not implemented")
        //     }
        //     Instruction::SALoad => {
        //         panic!("SALoad not implemented")
        //     }
        //     Instruction::IStore(index) => {
        //         self.local_variables[*index] =
        //             self.stack.pop().expect("Stack is empty").make_copy();
        //         self.pc += 1;
        //     }
        //     Instruction::LStore(index) => {
        //         self.local_variables[*index] =
        //             self.stack.pop().expect("Stack is empty").make_copy();
        //         self.pc += 1;
        //     }
        //     Instruction::FStore(index) => {
        //         self.local_variables[*index] =
        //             self.stack.pop().expect("Stack is empty").make_copy();
        //         self.pc += 1;
        //     }
        //     Instruction::DStore(index) => {
        //         self.local_variables[*index] =
        //             self.stack.pop().expect("Stack is empty").make_copy();
        //         self.pc += 1;
        //     }
        //     Instruction::AStore(index) => {
        //         self.local_variables[*index] =
        //             self.stack.pop().expect("Stack is empty").make_copy();
        //         self.pc += 1;
        //     }
        //     Instruction::IStore0 => {
        //         self.local_variables[0] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::IStore1 => {
        //         self.local_variables[1] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::IStore2 => {
        //         self.local_variables[2] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::IStore3 => {
        //         self.local_variables[3] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::LStore0 => {
        //         self.local_variables[0] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::LStore1 => {
        //         self.local_variables[1] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::LStore2 => {
        //         self.local_variables[2] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::LStore3 => {
        //         self.local_variables[3] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::FStore0 => {
        //         self.local_variables[0] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::FStore1 => {
        //         self.local_variables[1] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::FStore2 => {
        //         self.local_variables[2] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::FStore3 => {
        //         self.local_variables[3] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::DStore0 => {
        //         self.local_variables[0] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::DStore1 => {
        //         self.local_variables[1] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::DStore2 => {
        //         self.local_variables[2] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::DStore3 => {
        //         self.local_variables[3] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::AStore0 => {
        //         self.local_variables[0] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::AStore1 => {
        //         self.local_variables[1] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::AStore2 => {
        //         self.local_variables[2] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::AStore3 => {
        //         self.local_variables[3] = self.stack.pop().expect("Stack is empty").make_copy();
        //     }
        //     Instruction::IAStore => {
        //         panic!("IAStore not implemented")
        //     }
        //     Instruction::LAStore => {
        //         panic!("LAStore not implemented")
        //     }
        //     Instruction::FAStore => {
        //         panic!("FAStore not implemented")
        //     }
        //     Instruction::DAStore => {
        //         panic!("DAStore not implemented")
        //     }
        //     Instruction::AAStore => {
        //         panic!("AAStore not implemented")
        //     }
        //     Instruction::BAStore => {
        //         panic!("BAStore not implemented")
        //     }
        //     Instruction::CAStore => {
        //         panic!("CAStore not implemented")
        //     }
        //     Instruction::SAStore => {
        //         panic!("SAStore not implemented")
        //     }
        //     Instruction::Pop => {
        //         self.stack.pop();
        //     }
        //     Instruction::Pop2 => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         if value.is_wide() {
        //             self.stack.pop();
        //         }
        //     }
        //     Instruction::Dup => {
        //         // TODO: Properly implement duplication and check if it is correct
        //         let value = self.stack.pop().expect("Stack is empty");
        //         self.stack.push(value.make_copy());
        //         self.stack.push(value.make_copy());
        //     }
        //     Instruction::DupX1 => {
        //         panic!("DupX1 not implemented")
        //     }
        //     Instruction::DupX2 => {
        //         panic!("DupX2 not implemented")
        //     }
        //     Instruction::Dup2 => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         if value.is_wide() {
        //             self.stack.push(value.make_copy());
        //             self.stack.push(value);
        //         } else {
        //             let value2 = self.stack.pop().expect("Stack is empty");
        //             self.stack.push(value2.make_copy());
        //             self.stack.push(value.make_copy());
        //             self.stack.push(value2);
        //             self.stack.push(value);
        //         }
        //     }
        //     Instruction::Dup2X1 => {
        //         panic!("Dup2X1 not implemented")
        //     }
        //     Instruction::Dup2X2 => {
        //         panic!("Dup2X2 not implemented")
        //     }
        //     Instruction::Swap => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         self.stack.push(value);
        //         self.stack.push(value2);
        //     }
        //     Instruction::IAdd => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => match value2 {
        //                 Primitive::Int(v2) => {
        //                     self.stack.push(Primitive::Int(v + v2));
        //                 }
        //                 _ => panic!("IAdd not implemented for non integers"),
        //             },
        //             _ => panic!("IAdd not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LAdd => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => match value2 {
        //                 Primitive::Long(v2) => {
        //                     self.stack.push(Primitive::Long(v + v2));
        //                 }
        //                 _ => panic!("LAdd not implemented for non longs"),
        //             },
        //             _ => panic!("LAdd not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FAdd => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => match value2 {
        //                 Primitive::Float(v2) => {
        //                     self.stack.push(Primitive::Float(v + v2));
        //                 }
        //                 _ => panic!("FAdd not implemented for non floats"),
        //             },
        //             _ => panic!("FAdd not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DAdd => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => match value2 {
        //                 Primitive::Double(v2) => {
        //                     self.stack.push(Primitive::Double(v + v2));
        //                 }
        //                 _ => panic!("DAdd not implemented for non doubles"),
        //             },
        //             _ => panic!("DAdd not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::ISub => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => match value2 {
        //                 Primitive::Int(v2) => {
        //                     self.stack.push(Primitive::Int(v2 - v));
        //                 }
        //                 _ => panic!("ISub not implemented for non integers"),
        //             },
        //             _ => panic!("ISub not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LSub => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => match value2 {
        //                 Primitive::Long(v2) => {
        //                     self.stack.push(Primitive::Long(v2 - v));
        //                 }
        //                 _ => panic!("LSub not implemented for non longs"),
        //             },
        //             _ => panic!("LSub not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FSub => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => match value2 {
        //                 Primitive::Float(v2) => {
        //                     self.stack.push(Primitive::Float(v2 - v));
        //                 }
        //                 _ => panic!("FSub not implemented for non floats"),
        //             },
        //             _ => panic!("FSub not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DSub => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => match value2 {
        //                 Primitive::Double(v2) => {
        //                     self.stack.push(Primitive::Double(v2 - v));
        //                 }
        //                 _ => panic!("DSub not implemented for non doubles"),
        //             },
        //             _ => panic!("DSub not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::IMul => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => match value2 {
        //                 Primitive::Int(v2) => {
        //                     self.stack.push(Primitive::Int(v * v2));
        //                 }
        //                 _ => panic!("IMul not implemented for non integers"),
        //             },
        //             _ => panic!("IMul not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LMul => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => match value2 {
        //                 Primitive::Long(v2) => {
        //                     self.stack.push(Primitive::Long(v * v2));
        //                 }
        //                 _ => panic!("LMul not implemented for non longs"),
        //             },
        //             _ => panic!("LMul not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FMul => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => match value2 {
        //                 Primitive::Float(v2) => {
        //                     self.stack.push(Primitive::Float(v * v2));
        //                 }
        //                 _ => panic!("FMul not implemented for non floats"),
        //             },
        //             _ => panic!("FMul not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DMul => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => match value2 {
        //                 Primitive::Double(v2) => {
        //                     self.stack.push(Primitive::Double(v * v2));
        //                 }
        //                 _ => panic!("DMul not implemented for non doubles"),
        //             },
        //             _ => panic!("DMul not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::IDiv => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => match value2 {
        //                 Primitive::Int(v2) => {
        //                     self.stack.push(Primitive::Int(v2 / v));
        //                 }
        //                 _ => panic!("IDiv not implemented for non integers"),
        //             },
        //             _ => panic!("IDiv not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LDiv => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => match value2 {
        //                 Primitive::Long(v2) => {
        //                     self.stack.push(Primitive::Long(v2 / v));
        //                 }
        //                 _ => panic!("LDiv not implemented for non longs"),
        //             },
        //             _ => panic!("LDiv not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FDiv => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => match value2 {
        //                 Primitive::Float(v2) => {
        //                     self.stack.push(Primitive::Float(v2 / v));
        //                 }
        //                 _ => panic!("FDiv not implemented for non floats"),
        //             },
        //             _ => panic!("FDiv not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DDiv => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => match value2 {
        //                 Primitive::Double(v2) => {
        //                     self.stack.push(Primitive::Double(v2 / v));
        //                 }
        //                 _ => panic!("DDiv not implemented for non doubles"),
        //             },
        //             _ => panic!("DDiv not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::IRem => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => match value2 {
        //                 Primitive::Int(v2) => {
        //                     self.stack.push(Primitive::Int(v2 % v));
        //                 }
        //                 _ => panic!("IRem not implemented for non integers"),
        //             },
        //             _ => panic!("IRem not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LRem => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => match value2 {
        //                 Primitive::Long(v2) => {
        //                     self.stack.push(Primitive::Long(v2 % v));
        //                 }
        //                 _ => panic!("LRem not implemented for non longs"),
        //             },
        //             _ => panic!("LRem not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FRem => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => match value2 {
        //                 Primitive::Float(v2) => {
        //                     self.stack.push(Primitive::Float(v2 % v));
        //                 }
        //                 _ => panic!("FRem not implemented for non floats"),
        //             },
        //             _ => panic!("FRem not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DRem => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => match value2 {
        //                 Primitive::Double(v2) => {
        //                     self.stack.push(Primitive::Double(v2 % v));
        //                 }
        //                 _ => panic!("DRem not implemented for non doubles"),
        //             },
        //             _ => panic!("DRem not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::INeg => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Int(-v));
        //             }
        //             _ => panic!("INeg not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LNeg => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => {
        //                 self.stack.push(Primitive::Long(-v));
        //             }
        //             _ => panic!("LNeg not implemented for non longs"),
        //         }
        //     }
        //     Instruction::FNeg => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => {
        //                 self.stack.push(Primitive::Float(-v));
        //             }
        //             _ => panic!("FNeg not implemented for non floats"),
        //         }
        //     }
        //     Instruction::DNeg => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => {
        //                 self.stack.push(Primitive::Double(-v));
        //             }
        //             _ => panic!("DNeg not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::IInc(index, constant) => {
        //         let local_var = self
        //             .local_variables
        //             .get_mut(*index)
        //             .expect("Local variable not found");
        //         match local_var {
        //             Primitive::Int(v) => {
        //                 *v += *constant as i32;
        //             }
        //             _ => panic!("IInc not implemented for non integers"),
        //         }
        //     }
        //     Instruction::I2L => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Long(v as i64));
        //             }
        //             _ => panic!("I2L not implemented for non integers"),
        //         }
        //     }
        //     Instruction::I2F => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Float(v as f32));
        //             }
        //             _ => panic!("I2F not implemented for non integers"),
        //         }
        //     }
        //     Instruction::I2D => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Double(v as f64));
        //             }
        //             _ => panic!("I2D not implemented for non integers"),
        //         }
        //     }
        //     Instruction::L2I => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => {
        //                 self.stack.push(Primitive::Int(v as i32));
        //             }
        //             _ => panic!("L2I not implemented for non longs"),
        //         }
        //     }
        //     Instruction::L2F => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => {
        //                 self.stack.push(Primitive::Float(v as f32));
        //             }
        //             _ => panic!("L2F not implemented for non longs"),
        //         }
        //     }
        //     Instruction::L2D => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Long(v) => {
        //                 self.stack.push(Primitive::Double(v as f64));
        //             }
        //             _ => panic!("L2D not implemented for non longs"),
        //         }
        //     }
        //     Instruction::F2I => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => {
        //                 self.stack.push(Primitive::Int(v as i32));
        //             }
        //             _ => panic!("F2I not implemented for non floats"),
        //         }
        //     }
        //     Instruction::F2L => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => {
        //                 self.stack.push(Primitive::Long(v as i64));
        //             }
        //             _ => panic!("F2L not implemented for non floats"),
        //         }
        //     }
        //     Instruction::F2D => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Float(v) => {
        //                 self.stack.push(Primitive::Double(v as f64));
        //             }
        //             _ => panic!("F2D not implemented for non floats"),
        //         }
        //     }
        //     Instruction::D2I => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => {
        //                 self.stack.push(Primitive::Int(v as i32));
        //             }
        //             _ => panic!("D2I not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::D2L => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => {
        //                 self.stack.push(Primitive::Long(v as i64));
        //             }
        //             _ => panic!("D2L not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::D2F => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Double(v) => {
        //                 self.stack.push(Primitive::Float(v as f32));
        //             }
        //             _ => panic!("D2F not implemented for non doubles"),
        //         }
        //     }
        //     Instruction::I2B => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Byte(v as i8));
        //             }
        //             _ => panic!("I2B not implemented for non integers"),
        //         }
        //     }
        //     Instruction::I2C => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Char(v as u16));
        //             }
        //             _ => panic!("I2C not implemented for non integers"),
        //         }
        //     }
        //     Instruction::I2S => {
        //         let value = self.stack.pop().expect("Stack is empty");
        //         match value {
        //             Primitive::Int(v) => {
        //                 self.stack.push(Primitive::Short(v as i16));
        //             }
        //             _ => panic!("I2S not implemented for non integers"),
        //         }
        //     }
        //     Instruction::LCmp => {
        //         let value2 = self.stack.pop().expect("Stack is empty");
        //         let value1 = self.stack.pop().expect("Stack is empty");
        //         match (value1, value2) {
        //             (Primitive::Long(v1), Primitive::Long(v2)) => {
        //                 self.stack.push(Primitive::Int(match v1.cmp(&v2) {
        //                     Ordering::Less => -1,
        //                     Ordering::Equal => 0,
        //                     Ordering::Greater => 1,
        //                 }));
        //             }
        //             _ => panic!("LCmp not implemented for non longs"),
        //         }
        //     } // TODO: Implement remaining instructions
        //     Instruction::GetStatic(index) => {
        //         println!("GetStatic: {}", index);
        //         self.pc += 2;
        //     }
        //     Instruction::InvokeVirtual(index) => {
        //         println!("InvokeVirtual: {}", index);
        //         println!("{:?}", self.stack);
        //         self.pc += 2;
        //     }
        //     Instruction::InvokeStatic(index) => {
        //         println!("InvokeStatic: {}", index);
        //         self.pc += 2;
        //     }
        //     Instruction::Return => {
        //         println!("Return");
        //     }
        //
        //     _ => panic!("unable to execute unsupported instruction"),
        // }

        self.pc += 1;
    }
}
