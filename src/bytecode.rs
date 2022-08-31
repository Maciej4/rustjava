use crate::java_class::CodeAttribute;

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
    IStore,
    LStore,
    FStore,
    DStore,
    AStore,
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
pub enum StackFrame {
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

#[derive(Debug)]
pub enum LocalVariable {
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

#[derive(Debug)]
pub struct Bytecode {
    pub pc: usize,
    pub instructions: Vec<Instruction>,
    pub stack: Vec<StackFrame>,
    pub local_variables: Vec<LocalVariable>,
}

impl Bytecode {
    fn get_byte(code: &Vec<u8>, pc: &mut usize) -> i8 {
        let b = code[*pc + 1];
        *pc += 1;
        b as i8
    }

    fn get_short(code: &Vec<u8>, pc: &mut usize) -> i16 {
        let b1 = code[*pc + 1];
        let b2 = code[*pc + 2];
        *pc += 2;
        ((b1 as i16) << 8) | (b2 as i16)
    }

    fn get_int(code: &Vec<u8>, pc: &mut usize) -> i32 {
        let b1 = code[*pc + 1];
        let b2 = code[*pc + 2];
        let b3 = code[*pc + 3];
        let b4 = code[*pc + 4];
        *pc += 4;
        ((b1 as i32) << 24) | ((b2 as i32) << 16) | ((b3 as i32) << 8) | (b4 as i32)
    }

    pub fn new(code: Vec<u8>) -> Bytecode {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut byte_pos: usize = 0;
        let mut past_byte_pos: usize = 0;

        while byte_pos < code.len() as usize {
            instructions.push(match code[byte_pos] {
                0 => Instruction::Nop,
                1 => Instruction::AConstNull,
                2 => Instruction::IConstM1,
                3 => Instruction::IConst0,
                4 => Instruction::IConst1,
                5 => Instruction::IConst2,
                6 => Instruction::IConst3,
                7 => Instruction::IConst4,
                8 => Instruction::IConst5,
                9 => Instruction::LConst0,
                10 => Instruction::LConst1,
                11 => Instruction::FConst0,
                12 => Instruction::FConst1,
                13 => Instruction::FConst2,
                14 => Instruction::DConst0,
                15 => Instruction::DConst1,
                16 => Instruction::BIPush(Bytecode::get_byte(&code, &mut byte_pos)),
                17 => Instruction::SIPush(Bytecode::get_short(&code, &mut byte_pos)),
                18 => Instruction::Ldc(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                19 => Instruction::LdcW(Bytecode::get_short(&code, &mut byte_pos) as usize),
                20 => Instruction::Ldc2W(Bytecode::get_short(&code, &mut byte_pos) as usize),
                21 => Instruction::ILoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                22 => Instruction::LLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                23 => Instruction::FLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                24 => Instruction::DLoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                25 => Instruction::ALoad(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                26 => Instruction::ILoad0,
                27 => Instruction::ILoad1,
                28 => Instruction::ILoad2,
                29 => Instruction::ILoad3,
                30 => Instruction::LLoad0,
                31 => Instruction::LLoad1,
                32 => Instruction::LLoad2,
                33 => Instruction::LLoad3,
                34 => Instruction::FLoad0,
                35 => Instruction::FLoad1,
                36 => Instruction::FLoad2,
                37 => Instruction::FLoad3,
                38 => Instruction::DLoad0,
                39 => Instruction::DLoad1,
                40 => Instruction::DLoad2,
                41 => Instruction::DLoad3,
                42 => Instruction::ALoad0,
                43 => Instruction::ALoad1,
                44 => Instruction::ALoad2,
                45 => Instruction::ALoad3,
                46 => Instruction::IALoad,
                47 => Instruction::LALoad,
                48 => Instruction::FALoad,
                49 => Instruction::DALoad,
                50 => Instruction::AALoad,
                51 => Instruction::BALoad,
                52 => Instruction::CALoad,
                53 => Instruction::SALoad,
                54 => Instruction::IStore,
                55 => Instruction::LStore,
                56 => Instruction::FStore,
                57 => Instruction::DStore,
                58 => Instruction::AStore,
                59 => Instruction::IStore0,
                60 => Instruction::IStore1,
                61 => Instruction::IStore2,
                62 => Instruction::IStore3,
                63 => Instruction::LStore0,
                64 => Instruction::LStore1,
                65 => Instruction::LStore2,
                66 => Instruction::LStore3,
                67 => Instruction::FStore0,
                68 => Instruction::FStore1,
                69 => Instruction::FStore2,
                70 => Instruction::FStore3,
                71 => Instruction::DStore0,
                72 => Instruction::DStore1,
                73 => Instruction::DStore2,
                74 => Instruction::DStore3,
                75 => Instruction::AStore0,
                76 => Instruction::AStore1,
                77 => Instruction::AStore2,
                78 => Instruction::AStore3,
                79 => Instruction::IAStore,
                80 => Instruction::LAStore,
                81 => Instruction::FAStore,
                82 => Instruction::DAStore,
                83 => Instruction::AAStore,
                84 => Instruction::BAStore,
                85 => Instruction::CAStore,
                86 => Instruction::SAStore,
                87 => Instruction::Pop,
                88 => Instruction::Pop2,
                89 => Instruction::Dup,
                90 => Instruction::DupX1,
                91 => Instruction::DupX2,
                92 => Instruction::Dup2,
                93 => Instruction::Dup2X1,
                94 => Instruction::Dup2X2,
                95 => Instruction::Swap,
                96 => Instruction::IAdd,
                97 => Instruction::LAdd,
                98 => Instruction::FAdd,
                99 => Instruction::DAdd,
                100 => Instruction::ISub,
                101 => Instruction::LSub,
                102 => Instruction::FSub,
                103 => Instruction::DSub,
                104 => Instruction::IMul,
                105 => Instruction::LMul,
                106 => Instruction::FMul,
                107 => Instruction::DMul,
                108 => Instruction::IDiv,
                109 => Instruction::LDiv,
                110 => Instruction::FDiv,
                111 => Instruction::DDiv,
                112 => Instruction::IRem,
                113 => Instruction::LRem,
                114 => Instruction::FRem,
                115 => Instruction::DRem,
                116 => Instruction::INeg,
                117 => Instruction::LNeg,
                118 => Instruction::FNeg,
                119 => Instruction::DNeg,
                120 => Instruction::IShl,
                121 => Instruction::LShl,
                122 => Instruction::IShr,
                123 => Instruction::LShr,
                124 => Instruction::IUShr,
                125 => Instruction::LUShr,
                126 => Instruction::IAnd,
                127 => Instruction::LAnd,
                128 => Instruction::IOr,
                129 => Instruction::LOr,
                130 => Instruction::IXor,
                131 => Instruction::LXor,
                132 => Instruction::IInc(
                    Bytecode::get_byte(&code, &mut byte_pos) as usize,
                    Bytecode::get_byte(&code, &mut byte_pos),
                ),
                133 => Instruction::I2L,
                134 => Instruction::I2F,
                135 => Instruction::I2D,
                136 => Instruction::L2I,
                137 => Instruction::L2F,
                138 => Instruction::L2D,
                139 => Instruction::F2I,
                140 => Instruction::F2L,
                141 => Instruction::F2D,
                142 => Instruction::D2I,
                143 => Instruction::D2L,
                144 => Instruction::D2F,
                145 => Instruction::I2B,
                146 => Instruction::I2C,
                147 => Instruction::I2S,
                148 => Instruction::LCmp,
                149 => Instruction::FCmpL,
                150 => Instruction::FCmpG,
                151 => Instruction::DCmpL,
                152 => Instruction::DCmpG,
                153 => Instruction::IfEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
                154 => Instruction::IfNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                155 => Instruction::IfLt(Bytecode::get_short(&code, &mut byte_pos) as usize),
                156 => Instruction::IfGe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                157 => Instruction::IfGt(Bytecode::get_short(&code, &mut byte_pos) as usize),
                158 => Instruction::IfLe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                159 => Instruction::IfICmpEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
                160 => Instruction::IfICmpNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                161 => Instruction::IfICmpLt(Bytecode::get_short(&code, &mut byte_pos) as usize),
                162 => Instruction::IfICmpGe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                163 => Instruction::IfICmpGt(Bytecode::get_short(&code, &mut byte_pos) as usize),
                164 => Instruction::IfICmpLe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                165 => Instruction::IfACmpEq(Bytecode::get_short(&code, &mut byte_pos) as usize),
                166 => Instruction::IfACmpNe(Bytecode::get_short(&code, &mut byte_pos) as usize),
                167 => Instruction::Goto(Bytecode::get_short(&code, &mut byte_pos) as usize),
                168 => Instruction::Jsr(Bytecode::get_short(&code, &mut byte_pos) as usize),
                169 => Instruction::Ret(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                170 => panic!("Unsupported instruction: {}", 170),
                171 => panic!("Unsupported instruction: {}", 171),
                172 => Instruction::IReturn,
                173 => Instruction::LReturn,
                174 => Instruction::FReturn,
                175 => Instruction::DReturn,
                176 => Instruction::AReturn,
                177 => Instruction::Return,
                178 => Instruction::GetStatic(Bytecode::get_short(&code, &mut byte_pos) as usize),
                179 => Instruction::PutStatic(Bytecode::get_short(&code, &mut byte_pos) as usize),
                180 => Instruction::GetField(Bytecode::get_short(&code, &mut byte_pos) as usize),
                181 => Instruction::PutField(Bytecode::get_short(&code, &mut byte_pos) as usize),
                182 => {
                    Instruction::InvokeVirtual(Bytecode::get_short(&code, &mut byte_pos) as usize)
                }
                183 => {
                    Instruction::InvokeSpecial(Bytecode::get_short(&code, &mut byte_pos) as usize)
                }
                184 => {
                    Instruction::InvokeStatic(Bytecode::get_short(&code, &mut byte_pos) as usize)
                }
                185 => {
                    Instruction::InvokeInterface(Bytecode::get_short(&code, &mut byte_pos) as usize)
                }
                186 => {
                    Instruction::InvokeDynamic(Bytecode::get_short(&code, &mut byte_pos) as usize)
                }
                187 => Instruction::New(Bytecode::get_short(&code, &mut byte_pos) as usize),
                188 => Instruction::NewArray(Bytecode::get_byte(&code, &mut byte_pos) as usize),
                189 => Instruction::ANewArray(Bytecode::get_short(&code, &mut byte_pos) as usize),
                190 => Instruction::ArrayLength,
                191 => Instruction::AThrow,
                192 => Instruction::CheckCast(Bytecode::get_short(&code, &mut byte_pos) as usize),
                193 => Instruction::InstanceOf(Bytecode::get_short(&code, &mut byte_pos) as usize),
                194 => Instruction::MonitorEnter,
                195 => Instruction::MonitorExit,
                196 => panic!("Unsupported instruction: {}", 196),
                197 => panic!("Unsupported instruction: {}", 197),
                198 => Instruction::IfNull(Bytecode::get_short(&code, &mut byte_pos) as usize),
                199 => Instruction::IfNonNull(Bytecode::get_short(&code, &mut byte_pos) as usize),
                200 => Instruction::GotoW(Bytecode::get_int(&code, &mut byte_pos) as usize),
                201 => Instruction::JsrW(Bytecode::get_int(&code, &mut byte_pos) as usize),
                202 => Instruction::Breakpoint,
                _ => panic!("unsupported instruction"),
            });

            for _ in past_byte_pos..byte_pos {
                instructions.push(Instruction::Skip);
            }

            byte_pos += 1;
            past_byte_pos = byte_pos;
        }

        Bytecode {
            pc: 0,
            instructions,
            stack: Vec::new(),
            local_variables: Vec::new(),
        }
    }

    pub fn step(&mut self) {
        // match &self.instructions[self.pc] {
        //     Nop => {
        //         self.pc += 1;
        //     }
        //     Panic => {
        //         panic!("reached unreachable program counter position");
        //     }
        //     _ => {}
        // }

        self.pc += 1;
    }
}
