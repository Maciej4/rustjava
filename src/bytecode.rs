use crate::java_class::ConstantPoolEntry;

#[derive(Debug, Clone)]
pub enum Instruction {
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
    InvokeInterface(usize), // TODO: 4: indexbyte1, indexbyte2, count, 0
    InvokeDynamic(usize),   // TODO: 4: indexbyte1, indexbyte2, 0, 0
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

#[derive(Debug, Clone)]
pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum Operator {
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Primitive {
    Null,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    Reference(usize),
}

impl Primitive {
    pub fn eval(a: Primitive, o: Operator) -> Primitive {
        match o {
            Operator::Neg => match a {
                Primitive::Int(i) => Primitive::Int(-i),
                Primitive::Long(l) => Primitive::Long(-l),
                Primitive::Float(f) => Primitive::Float(-f),
                Primitive::Double(d) => Primitive::Double(-d),
                _ => panic!("Unsupported operation"),
            },
            Operator::Convert(source, destination) => match (a, source) {
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

    pub fn eval2(a: Primitive, b: Primitive, o: Operator) -> Primitive {
        match o {
            Operator::Add => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i + j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l + j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f + j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d + j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Sub => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i - j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l - j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f - j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d - j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Mul => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i * j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l * j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f * j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d * j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Div => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i / j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l / j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f / j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d / j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Rem => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i % j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l % j),
                (Primitive::Float(f), Primitive::Float(j)) => Primitive::Float(f % j),
                (Primitive::Double(d), Primitive::Double(j)) => Primitive::Double(d % j),
                _ => panic!("Unsupported operation"),
            },
            Operator::And => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i & j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l & j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Or => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i | j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l | j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Xor => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i ^ j),
                (Primitive::Long(l), Primitive::Long(j)) => Primitive::Long(l ^ j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Shl => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i << j),
                (Primitive::Long(l), Primitive::Int(j)) => Primitive::Long(l << j),
                _ => panic!("Unsupported operation"),
            },
            Operator::Shr => match (a, b) {
                (Primitive::Int(i), Primitive::Int(j)) => Primitive::Int(i >> j),
                (Primitive::Long(l), Primitive::Int(j)) => Primitive::Long(l >> j),
                _ => panic!("Unsupported operation"),
            },
            Operator::UShr => match (a, b) {
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
    pub instructions: Vec<Instruction>,
    pub stack: Vec<Primitive>,
    pub local_variables: Vec<Primitive>,
    pub constant_pool: Vec<ConstantPoolEntry>,
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

    pub fn new(code: Vec<u8>, constant_pool: Vec<ConstantPoolEntry>) -> Bytecode {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut pc: usize = 0;
        let mut past_byte_pos: usize = 0;

        while pc < code.len() as usize {
            instructions.push(match code[pc] {
                0 => Instruction::Nop,
                1 => Instruction::AConstNull,
                2 => Instruction::Const(Primitive::Int(-1)),
                3 => Instruction::Const(Primitive::Int(0)),
                4 => Instruction::Const(Primitive::Int(1)),
                5 => Instruction::Const(Primitive::Int(2)),
                6 => Instruction::Const(Primitive::Int(3)),
                7 => Instruction::Const(Primitive::Int(4)),
                8 => Instruction::Const(Primitive::Int(5)),
                9 => Instruction::Const(Primitive::Long(0)),
                10 => Instruction::Const(Primitive::Long(1)),
                11 => Instruction::Const(Primitive::Float(0.0)),
                12 => Instruction::Const(Primitive::Float(1.0)),
                13 => Instruction::Const(Primitive::Float(2.0)),
                14 => Instruction::Const(Primitive::Double(0.0)),
                15 => Instruction::Const(Primitive::Double(1.0)),
                16 => Instruction::Const(Primitive::Int(Bytecode::u1(&code, &mut pc) as i32)),
                17 => Instruction::Const(Primitive::Int(Bytecode::u2(&code, &mut pc) as i32)),
                18 => Instruction::LoadConst(Bytecode::u1(&code, &mut pc)),
                19 => Instruction::LoadConst(Bytecode::u2(&code, &mut pc)),
                20 => Instruction::LoadConst(Bytecode::u2(&code, &mut pc)),
                21 => Instruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Int),
                22 => Instruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Long),
                23 => Instruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Float),
                24 => Instruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Double),
                25 => Instruction::Load(Bytecode::u1(&code, &mut pc), PrimitiveType::Reference),
                26 => Instruction::Load(0, PrimitiveType::Int),
                27 => Instruction::Load(1, PrimitiveType::Int),
                28 => Instruction::Load(2, PrimitiveType::Int),
                29 => Instruction::Load(3, PrimitiveType::Int),
                30 => Instruction::Load(0, PrimitiveType::Long),
                31 => Instruction::Load(1, PrimitiveType::Long),
                32 => Instruction::Load(2, PrimitiveType::Long),
                33 => Instruction::Load(3, PrimitiveType::Long),
                34 => Instruction::Load(0, PrimitiveType::Float),
                35 => Instruction::Load(1, PrimitiveType::Float),
                36 => Instruction::Load(2, PrimitiveType::Float),
                37 => Instruction::Load(3, PrimitiveType::Float),
                38 => Instruction::Load(0, PrimitiveType::Double),
                39 => Instruction::Load(1, PrimitiveType::Double),
                40 => Instruction::Load(2, PrimitiveType::Double),
                41 => Instruction::Load(3, PrimitiveType::Double),
                42 => Instruction::Load(0, PrimitiveType::Reference),
                43 => Instruction::Load(1, PrimitiveType::Reference),
                44 => Instruction::Load(2, PrimitiveType::Reference),
                45 => Instruction::Load(3, PrimitiveType::Reference),
                46 => Instruction::ALoad(PrimitiveType::Int),
                47 => Instruction::ALoad(PrimitiveType::Long),
                48 => Instruction::ALoad(PrimitiveType::Float),
                49 => Instruction::ALoad(PrimitiveType::Double),
                50 => Instruction::ALoad(PrimitiveType::Reference),
                51 => Instruction::ALoad(PrimitiveType::Byte),
                52 => Instruction::ALoad(PrimitiveType::Char),
                53 => Instruction::ALoad(PrimitiveType::Short),
                54 => Instruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Int),
                55 => Instruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Long),
                56 => Instruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Float),
                57 => Instruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Double),
                58 => Instruction::Store(Bytecode::u1(&code, &mut pc), PrimitiveType::Reference),
                59 => Instruction::Store(0, PrimitiveType::Int),
                60 => Instruction::Store(1, PrimitiveType::Int),
                61 => Instruction::Store(2, PrimitiveType::Int),
                62 => Instruction::Store(3, PrimitiveType::Int),
                63 => Instruction::Store(0, PrimitiveType::Long),
                64 => Instruction::Store(1, PrimitiveType::Long),
                65 => Instruction::Store(2, PrimitiveType::Long),
                66 => Instruction::Store(3, PrimitiveType::Long),
                67 => Instruction::Store(0, PrimitiveType::Float),
                68 => Instruction::Store(1, PrimitiveType::Float),
                69 => Instruction::Store(2, PrimitiveType::Float),
                70 => Instruction::Store(3, PrimitiveType::Float),
                71 => Instruction::Store(0, PrimitiveType::Double),
                72 => Instruction::Store(1, PrimitiveType::Double),
                73 => Instruction::Store(2, PrimitiveType::Double),
                74 => Instruction::Store(3, PrimitiveType::Double),
                75 => Instruction::Store(0, PrimitiveType::Reference),
                76 => Instruction::Store(1, PrimitiveType::Reference),
                77 => Instruction::Store(2, PrimitiveType::Reference),
                78 => Instruction::Store(3, PrimitiveType::Reference),
                79 => Instruction::AStore(PrimitiveType::Int),
                80 => Instruction::AStore(PrimitiveType::Long),
                81 => Instruction::AStore(PrimitiveType::Float),
                82 => Instruction::AStore(PrimitiveType::Double),
                83 => Instruction::AStore(PrimitiveType::Reference),
                84 => Instruction::AStore(PrimitiveType::Byte),
                85 => Instruction::AStore(PrimitiveType::Char),
                86 => Instruction::AStore(PrimitiveType::Short),
                87 => Instruction::Pop,
                88 => Instruction::Pop2,
                89 => Instruction::Dup,
                90 => Instruction::DupX1,
                91 => Instruction::DupX2,
                92 => Instruction::Dup2,
                93 => Instruction::Dup2X1,
                94 => Instruction::Dup2X2,
                95 => Instruction::Swap,
                96 => Instruction::Add(PrimitiveType::Int),
                97 => Instruction::Add(PrimitiveType::Long),
                98 => Instruction::Add(PrimitiveType::Float),
                99 => Instruction::Add(PrimitiveType::Double),
                100 => Instruction::Sub(PrimitiveType::Int),
                101 => Instruction::Sub(PrimitiveType::Long),
                102 => Instruction::Sub(PrimitiveType::Float),
                103 => Instruction::Sub(PrimitiveType::Double),
                104 => Instruction::Mul(PrimitiveType::Int),
                105 => Instruction::Mul(PrimitiveType::Long),
                106 => Instruction::Mul(PrimitiveType::Float),
                107 => Instruction::Mul(PrimitiveType::Double),
                108 => Instruction::Div(PrimitiveType::Int),
                109 => Instruction::Div(PrimitiveType::Long),
                110 => Instruction::Div(PrimitiveType::Float),
                111 => Instruction::Div(PrimitiveType::Double),
                112 => Instruction::Rem(PrimitiveType::Int),
                113 => Instruction::Rem(PrimitiveType::Long),
                114 => Instruction::Rem(PrimitiveType::Float),
                115 => Instruction::Rem(PrimitiveType::Double),
                116 => Instruction::Neg(PrimitiveType::Int),
                117 => Instruction::Neg(PrimitiveType::Long),
                118 => Instruction::Neg(PrimitiveType::Float),
                119 => Instruction::Neg(PrimitiveType::Double),
                120 => Instruction::Shl(PrimitiveType::Int),
                121 => Instruction::Shl(PrimitiveType::Long),
                122 => Instruction::Shr(PrimitiveType::Int),
                123 => Instruction::Shr(PrimitiveType::Long),
                124 => Instruction::UShr(PrimitiveType::Int),
                125 => Instruction::UShr(PrimitiveType::Long),
                126 => Instruction::And(PrimitiveType::Int),
                127 => Instruction::And(PrimitiveType::Long),
                128 => Instruction::Or(PrimitiveType::Int),
                129 => Instruction::Or(PrimitiveType::Long),
                130 => Instruction::Xor(PrimitiveType::Int),
                131 => Instruction::Xor(PrimitiveType::Long),
                132 => Instruction::IInc(
                    Bytecode::u1(&code, &mut pc),
                    Bytecode::u1(&code, &mut pc) as i8,
                ),
                133 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Long),
                134 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Float),
                135 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Double),
                136 => Instruction::Convert(PrimitiveType::Long, PrimitiveType::Int),
                137 => Instruction::Convert(PrimitiveType::Long, PrimitiveType::Float),
                138 => Instruction::Convert(PrimitiveType::Long, PrimitiveType::Double),
                139 => Instruction::Convert(PrimitiveType::Float, PrimitiveType::Int),
                140 => Instruction::Convert(PrimitiveType::Float, PrimitiveType::Long),
                141 => Instruction::Convert(PrimitiveType::Float, PrimitiveType::Double),
                142 => Instruction::Convert(PrimitiveType::Double, PrimitiveType::Int),
                143 => Instruction::Convert(PrimitiveType::Double, PrimitiveType::Long),
                144 => Instruction::Convert(PrimitiveType::Double, PrimitiveType::Float),
                145 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Byte),
                146 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Char),
                147 => Instruction::Convert(PrimitiveType::Int, PrimitiveType::Short),
                148 => Instruction::LCmp,
                149 => Instruction::FCmpL,
                150 => Instruction::FCmpG,
                151 => Instruction::DCmpL,
                152 => Instruction::DCmpG,
                153 => Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                154 => Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::NotEqual),
                155 => Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::LessThan),
                156 => {
                    Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::GreaterThanOrEqual)
                }
                157 => Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::GreaterThan),
                158 => Instruction::If(Bytecode::u2(&code, &mut pc), Comparison::LessThanOrEqual),
                159 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                160 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::NotEqual),
                161 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::LessThan),
                162 => Instruction::IfICmp(
                    Bytecode::u2(&code, &mut pc),
                    Comparison::GreaterThanOrEqual,
                ),
                163 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::GreaterThan),
                164 => {
                    Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::LessThanOrEqual)
                }
                165 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::Equal),
                166 => Instruction::IfICmp(Bytecode::u2(&code, &mut pc), Comparison::NotEqual),
                167 => Instruction::Goto(Bytecode::u2(&code, &mut pc)),
                168 => Instruction::Jsr(Bytecode::u2(&code, &mut pc)),
                169 => Instruction::Ret(Bytecode::u1(&code, &mut pc)),
                170 => panic!("Unsupported instruction: {}", 170),
                171 => panic!("Unsupported instruction: {}", 171),
                172 => Instruction::Return(PrimitiveType::Int),
                173 => Instruction::Return(PrimitiveType::Long),
                174 => Instruction::Return(PrimitiveType::Float),
                175 => Instruction::Return(PrimitiveType::Double),
                176 => Instruction::Return(PrimitiveType::Reference),
                177 => Instruction::Return(PrimitiveType::Null),
                178 => Instruction::GetStatic(Bytecode::u2(&code, &mut pc) as usize),
                179 => Instruction::PutStatic(Bytecode::u2(&code, &mut pc) as usize),
                180 => Instruction::GetField(Bytecode::u2(&code, &mut pc) as usize),
                181 => Instruction::PutField(Bytecode::u2(&code, &mut pc) as usize),
                182 => Instruction::InvokeVirtual(Bytecode::u2(&code, &mut pc) as usize),
                183 => Instruction::InvokeSpecial(Bytecode::u2(&code, &mut pc) as usize),
                184 => Instruction::InvokeStatic(Bytecode::u2(&code, &mut pc) as usize),
                185 => Instruction::InvokeInterface(Bytecode::u2(&code, &mut pc) as usize),
                186 => Instruction::InvokeDynamic(Bytecode::u2(&code, &mut pc) as usize),
                187 => Instruction::New(Bytecode::u2(&code, &mut pc) as usize),
                188 => Instruction::NewArray(Bytecode::u1(&code, &mut pc) as usize),
                189 => Instruction::ANewArray(Bytecode::u2(&code, &mut pc) as usize),
                190 => Instruction::ArrayLength,
                191 => Instruction::AThrow,
                192 => Instruction::CheckCast(Bytecode::u2(&code, &mut pc) as usize),
                193 => Instruction::InstanceOf(Bytecode::u2(&code, &mut pc) as usize),
                194 => Instruction::MonitorEnter,
                195 => Instruction::MonitorExit,
                196 => panic!("Unsupported instruction: {}", 196),
                197 => panic!("Unsupported instruction: {}", 197),
                198 => Instruction::IfNull(Bytecode::u2(&code, &mut pc) as usize),
                199 => Instruction::IfNonNull(Bytecode::u2(&code, &mut pc) as usize),
                200 => Instruction::GotoW(Bytecode::u4(&code, &mut pc) as usize),
                201 => Instruction::JsrW(Bytecode::u4(&code, &mut pc) as usize),
                202 => Instruction::Breakpoint,
                _ => panic!("unsupported instruction"),
            });

            for _ in past_byte_pos..pc {
                instructions.push(Instruction::Nop);
            }

            pc += 1;
            past_byte_pos = pc;
        }

        Bytecode {
            pc: 0,
            instructions,
            stack: Vec::with_capacity(5),
            local_variables: Vec::with_capacity(5),
            constant_pool,
        }
    }

    pub fn math(&mut self, _op_type: PrimitiveType, o: Operator) {
        let a = self.stack.pop().expect("empty stack");
        let b = self.stack.pop().expect("empty stack");

        // TODO: Check that a is the same type as op_type

        self.stack.push(Primitive::eval2(b, a, o));
    }

    pub fn comp(&mut self, comparator: Comparison) -> bool {
        let a = self.stack.pop().expect("empty stack");

        match a {
            Primitive::Int(x) => match comparator {
                Comparison::Equal => x == 0,
                Comparison::NotEqual => x != 0,
                Comparison::LessThan => x < 0,
                Comparison::GreaterThanOrEqual => x >= 0,
                Comparison::GreaterThan => x > 0,
                Comparison::LessThanOrEqual => x <= 0,
            }
            Primitive::Long(x) => match comparator {
                Comparison::Equal => x == 0,
                Comparison::NotEqual => x != 0,
                Comparison::LessThan => x < 0,
                Comparison::GreaterThanOrEqual => x >= 0,
                Comparison::GreaterThan => x > 0,
                Comparison::LessThanOrEqual => x <= 0,
            }
            Primitive::Float(x) => match comparator {
                Comparison::Equal => x == 0.0,
                Comparison::NotEqual => x != 0.0,
                Comparison::LessThan => x < 0.0,
                Comparison::GreaterThanOrEqual => x >= 0.0,
                Comparison::GreaterThan => x > 0.0,
                Comparison::LessThanOrEqual => x <= 0.0,
            }
            Primitive::Double(x) => match comparator {
                Comparison::Equal => x == 0.0,
                Comparison::NotEqual => x != 0.0,
                Comparison::LessThan => x < 0.0,
                Comparison::GreaterThanOrEqual => x >= 0.0,
                Comparison::GreaterThan => x > 0.0,
                Comparison::LessThanOrEqual => x <= 0.0,
            }
            _ => panic!("unsupported type for comparison"),
        }
    }

    pub fn i_comp(&mut self, comparator: Comparison) -> bool {
        let a = self.stack.pop().expect("empty stack");
        let b = self.stack.pop().expect("empty stack");

        match (b, a) {
            (Primitive::Int(x), Primitive::Int(y)) => match comparator {
                Comparison::Equal => x == y,
                Comparison::NotEqual => x != y,
                Comparison::LessThan => x < y,
                Comparison::GreaterThanOrEqual => x >= y,
                Comparison::GreaterThan => x > y,
                Comparison::LessThanOrEqual => x <= y,
            },
            _ => panic!("comparing non-int types"),
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.instructions.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let instruction = self.instructions[self.pc].clone();

        let mut no_step = false;

        // TODO: Implement array operations

        match instruction {
            Instruction::Nop => {}
            Instruction::AConstNull => self.stack.push(Primitive::Null),
            Instruction::Const(value) => self.stack.push(value.clone()),
            // Instruction::LoadConst(index) => {}
            // TODO: Check that the stored or loaded type matches the expected type
            Instruction::Load(index, _type_to_load) => {
                self.stack.push(self.local_variables[index].clone())
            }
            // Instruction::ALoad(stored_type) => {}
            Instruction::Store(index, _type_to_store) => {
                if self.local_variables.len() <= index {
                    self.local_variables.resize(index + 1, Primitive::Null)
                };
                self.local_variables[index] = self.stack.pop().expect("empty stack")
            }
            // Instruction::AStore(stored_type) => {}
            Instruction::Pop => {
                self.stack.pop();
            }
            Instruction::Pop2 => {
                if !self.stack.pop().expect("empty stack").is_wide() {
                    self.stack.pop();
                }
            }
            Instruction::Dup => {}
            Instruction::DupX1 => {}
            Instruction::DupX2 => {}
            Instruction::Dup2 => {}
            Instruction::Dup2X1 => {}
            Instruction::Dup2X2 => {}
            Instruction::Swap => {}
            Instruction::Add(operand_type) => self.math(operand_type, Operator::Add),
            Instruction::Sub(operand_type) => self.math(operand_type, Operator::Sub),
            Instruction::Mul(operand_type) => self.math(operand_type, Operator::Mul),
            Instruction::Div(operand_type) => self.math(operand_type, Operator::Div),
            Instruction::Rem(operand_type) => self.math(operand_type, Operator::Rem),
            Instruction::Neg(operand_type) => self.math(operand_type, Operator::Neg),
            Instruction::Shl(operand_type) => self.math(operand_type, Operator::Shl),
            Instruction::Shr(operand_type) => self.math(operand_type, Operator::Shr),
            Instruction::UShr(operand_type) => self.math(operand_type, Operator::UShr),
            Instruction::And(operand_type) => self.math(operand_type, Operator::And),
            Instruction::Or(operand_type) => self.math(operand_type, Operator::Or),
            Instruction::Xor(operand_type) => self.math(operand_type, Operator::Xor),
            Instruction::IInc(index, constant) => {
                self.local_variables[index] = Primitive::eval2(
                    self.local_variables[index].clone(),
                    Primitive::Int(constant as i32),
                    Operator::Add,
                );
            }
            Instruction::Convert(start_type, end_type) => {
                let converted = Primitive::eval(
                    self.stack.pop().expect("empty stack"),
                    Operator::Convert(start_type, end_type),
                );
                self.stack.push(converted);
            }
            // TODO: Implement branching
            Instruction::LCmp => {}
            Instruction::FCmpL => {}
            Instruction::FCmpG => {}
            Instruction::DCmpL => {}
            Instruction::DCmpG => {}
            Instruction::If(branch_offset, comparator) => {if self.comp(comparator) {
                self.pc += branch_offset;
                no_step = true;
            }}
            Instruction::IfICmp(branch_offset, comparator) => {if self.i_comp(comparator) {
                self.pc += branch_offset;
                no_step = true;
            }}
            Instruction::Goto(branch_offset) => {
                self.pc += branch_offset;
                no_step = true;
            }
            Instruction::Jsr(branch_offset) => {
                self.stack.push(Primitive::Reference(self.pc + 1));
                self.pc += branch_offset;
                no_step = true;
            }
            Instruction::Ret(index) => {
                self.pc = match self.local_variables[index] {
                    Primitive::Reference(x) => x,
                    _ => panic!("invalid return address"),
                };
                no_step = true;
            }
            // Instruction::TableSwitch(usize, usize, usize) => {},
            // Instruction::LookupSwitch(usize, usize, usize) => {},
            Instruction::Return(return_type) => {}
            Instruction::GetStatic(index) => {}
            Instruction::PutStatic(index) => {}
            Instruction::GetField(index) => {}
            Instruction::PutField(index) => {}
            Instruction::InvokeVirtual(index) => {
                println!("{:?}", self.stack);
            }
            Instruction::InvokeSpecial(index) => {}
            Instruction::InvokeStatic(index) => {}
            Instruction::InvokeInterface(index) => {}
            Instruction::InvokeDynamic(index) => {}
            Instruction::New(index) => {}
            Instruction::NewArray(a_type) => {}
            Instruction::ANewArray(index) => {}
            Instruction::ArrayLength => {}
            Instruction::AThrow => {}
            Instruction::CheckCast(usize) => {}
            Instruction::InstanceOf(usize) => {}
            Instruction::MonitorEnter => {}
            Instruction::MonitorExit => {}
            Instruction::Wide(usize) => {}
            Instruction::MultiANewArray(usize, usize2) => {}
            Instruction::IfNull(usize) => {}
            Instruction::IfNonNull(usize) => {}
            Instruction::GotoW(usize) => {}
            Instruction::JsrW(usize) => {}
            Instruction::Breakpoint => {}
            _ => panic!("unsupported instruction"),
        }

        if !no_step {
            self.pc += 1;
        }
    }
}
