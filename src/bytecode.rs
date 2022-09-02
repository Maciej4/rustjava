use crate::java_class::ConstantPoolEntry;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    AConstNull,
    Const(Primitive),
    LoadConst(usize),
    Load(usize, PrimitiveType),
    ALoad(PrimitiveType),
    Store(usize, PrimitiveType),
    AStore(PrimitiveType),
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    Add(PrimitiveType), // TODO: Squish these into one math instruction
    Sub(PrimitiveType),
    Mul(PrimitiveType),
    Div(PrimitiveType),
    Rem(PrimitiveType),
    Neg(PrimitiveType),
    Shl(PrimitiveType),
    Shr(PrimitiveType),
    UShr(PrimitiveType),
    And(PrimitiveType),
    Or(PrimitiveType),
    Xor(PrimitiveType),
    IInc(usize, i8),
    Convert(PrimitiveType, PrimitiveType),
    LCmp, // TODO: Perhaps these could be reduced to a single instruction?
    FCmpL,
    FCmpG,
    DCmpL,
    DCmpG,
    If(usize, Comparison),
    IfICmp(usize, Comparison),
    Goto(usize),
    Jsr(usize),
    Ret(usize),
    TableSwitch(usize, usize, usize), // TODO: Properly implement this.
    LookupSwitch(usize, usize, usize),
    Return(PrimitiveType),
    GetStatic(usize),
    PutStatic(usize),
    GetField(usize),
    PutField(usize),
    InvokeVirtual(usize),
    InvokeSpecial(usize),
    InvokeStatic(usize),
    InvokeInterface(usize), // TODO: 4: indexbyte1, indexbyte2, count, 0
    InvokeDynamic(usize),   // TODO: 4: indexbyte1, indexbyte2, 0, 0
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
