//! This module contains the code for the java class file parser.
use crate::bytecode::*;
use crate::java_class::*;
use crate::jvm::{Class, Method};
use crate::reader::Reader;
use std::collections::HashMap;

fn parse_constant_pool(r: &mut Reader, constant_pool_count: u16) -> Vec<ConstantPoolEntry> {
    let mut constant_pool = Vec::new();

    for _ in 1..constant_pool_count {
        constant_pool.push(match r.g1() {
            1 => {
                let length = r.g2u();
                ConstantPoolEntry::Utf8(String::from_utf8(r.g(length)).unwrap())
            }
            3 => ConstantPoolEntry::Integer(i32::from_be_bytes(r.g4_array())),
            4 => ConstantPoolEntry::Float(f32::from_be_bytes(r.g4_array())),
            5 => ConstantPoolEntry::Long(i64::from_be_bytes(r.g8_array())),
            6 => ConstantPoolEntry::Double(f64::from_be_bytes(r.g8_array())),
            7 => ConstantPoolEntry::Class(r.g2u()),
            8 => ConstantPoolEntry::String(r.g2u()),
            9 => ConstantPoolEntry::FieldRef(r.g2u(), r.g2u()),
            10 => ConstantPoolEntry::MethodRef(r.g2u(), r.g2u()),
            11 => ConstantPoolEntry::InterfaceMethodRef(r.g2u(), r.g2u()),
            12 => ConstantPoolEntry::NameAndType(r.g2u(), r.g2u()),
            15 => ConstantPoolEntry::MethodHandle(r.g1(), r.g2u()),
            16 => ConstantPoolEntry::MethodType(r.g2u()),
            18 => ConstantPoolEntry::InvokeDynamic(r.g2u(), r.g2u()),
            _ => panic!("unsupported constant pool entry"),
        });
    }

    constant_pool
}

fn parse_interfaces(r: &mut Reader, interfaces_count: u16) -> Vec<Interface> {
    let mut interfaces = Vec::new();

    for _ in 0..interfaces_count {
        interfaces.push(Interface { name: r.g2() });
    }

    interfaces
}

fn parse_fields(r: &mut Reader, ct: &[ConstantPoolEntry], fields_count: u16) -> Vec<Field> {
    let mut fields = Vec::new();

    for _ in 0..fields_count {
        let access_flags = r.g2();
        let name = r.g2();
        let descriptor = r.g2();
        let attributes_count = r.g2();
        let attributes = parse_attributes(r, ct, attributes_count);

        fields.push(Field {
            access_flags,
            name,
            descriptor,
            attributes_count,
            attributes,
        });
    }

    fields
}

fn parse_methods(
    r: &mut Reader,
    ct: &[ConstantPoolEntry],
    methods_count: u16,
) -> Vec<UnparsedMethod> {
    let mut methods = Vec::new();

    for _i in 0..methods_count {
        let access_flags = r.g2();
        let name_index = r.g2();
        let descriptor_index = r.g2();
        let attributes_count = r.g2();
        let attributes = parse_attributes(r, ct, attributes_count);

        methods.push(UnparsedMethod {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        });
    }

    methods
}

fn parse_attributes(
    r: &mut Reader,
    ct: &[ConstantPoolEntry],
    attributes_count: u16,
) -> Vec<Attribute> {
    let mut attributes = Vec::new();

    for _i in 0..attributes_count {
        let attribute_name_index = r.g2() - 1;
        let attribute_length = r.g4();
        let attribute_start_position = r.pos();
        let attribute_str_name = match ct[attribute_name_index as usize] {
            ConstantPoolEntry::Utf8(ref s) => s,
            _ => {
                println!("{:?}", ct[attribute_name_index as usize]);
                panic!("attribute name is not a utf8 string")
            }
        };

        attributes.push(match &attribute_str_name[..] {
            "ConstantValue" => Attribute::ConstantValue(ConstantValueAttribute {
                attribute_name_index,
                attribute_length,
                constant_value_index: r.g2(),
            }),
            "Code" => {
                let max_stack = r.g2();
                let max_locals = r.g2();
                let code_length = r.g4();
                let code = r.g(code_length as usize);
                let exception_table_length = r.g2();
                let exception_table = r.g(exception_table_length as usize);
                let attributes_count = r.g2();
                let attributes = parse_attributes(r, ct, attributes_count);

                Attribute::Code(CodeAttribute {
                    attribute_name_index,
                    attribute_length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    attributes_count,
                    attributes,
                })
            }
            "StackMapTable" => Attribute::StackMapTable(StackMapTableAttribute {
                attribute_name_index,
                attribute_length,
                number_of_entries: r.g2(),
                entries: r.g(attribute_length as usize),
            }),
            "Exceptions" => Attribute::Exceptions(ExceptionsAttribute {
                attribute_name_index,
                attribute_length,
                number_of_exceptions: r.g2(),
                exception_index_table: r.g(attribute_length as usize),
            }),
            "InnerClasses" => {
                let number_of_classes = r.g2();
                let mut classes = Vec::new();

                for _ in 0..number_of_classes {
                    classes.push(InnerClassElement {
                        inner_class_info_index: r.g2(),
                        outer_class_info_index: r.g2(),
                        inner_name_index: r.g2(),
                        inner_class_access_flags: r.g2(),
                    });
                }

                Attribute::InnerClasses(InnerClassesAttribute {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "EnclosingMethod" => Attribute::EnclosingMethod(EnclosingMethodAttribute {
                attribute_name_index,
                attribute_length,
                class_index: r.g2(),
                method_index: r.g2(),
            }),
            "Synthetic" => Attribute::Synthetic(SyntheticAttribute {
                attribute_name_index,
                attribute_length,
            }),
            "Signature" => Attribute::Signature(SignatureAttribute {
                attribute_name_index,
                attribute_length,
                signature_index: r.g2(),
            }),
            "SourceFile" => Attribute::SourceFile(SourceFileAttribute {
                attribute_name_index,
                attribute_length,
                sourcefile_index: r.g2(),
            }),
            "LineNumberTable" => {
                let line_number_table_length = r.g2();
                let mut line_number_table = Vec::new();

                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberTableElement {
                        start_pc: r.g2(),
                        line_number: r.g2(),
                    });
                }

                Attribute::LineNumberTable(LineNumberTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                })
            }
            "LocalVariableTable" => {
                let local_variable_table_length = r.g2();
                let mut local_variable_table = Vec::new();

                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableTableElement {
                        start_pc: r.g2(),
                        length: r.g2(),
                        name_index: r.g2(),
                        descriptor_index: r.g2(),
                        index: r.g2(),
                    });
                }

                Attribute::LocalVariableTable(LocalVariableTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    local_variable_table_length,
                    local_variable_table,
                })
            }
            "LocalVariableTypeTable" => {
                let local_variable_type_table_length = r.g2();
                let mut local_variable_type_table = Vec::new();

                for _ in 0..local_variable_type_table_length {
                    local_variable_type_table.push(LocalVariableTypeTableElement {
                        start_pc: r.g2(),
                        length: r.g2(),
                        name_index: r.g2(),
                        signature_index: r.g2(),
                        index: r.g2(),
                    });
                }

                Attribute::LocalVariableTypeTable(LocalVariableTypeTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    local_variable_type_table_length,
                    local_variable_type_table,
                })
            }
            "Deprecated" => Attribute::Deprecated(DeprecatedAttribute {
                attribute_name_index,
                attribute_length,
            }),
            _ => panic!("{} is an unsupported attribute type", attribute_str_name),
        });

        // if r.pos() != attribute_start_position + attribute_length as usize {
        //     println!(
        //         "{:?} was not parsed completely",
        //         ct[attribute_name_index as usize]
        //     );
        // }

        r.set_pos(attribute_start_position + attribute_length as usize);
    }

    attributes
}

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

pub fn bytes_to_bytecode(code: Vec<u8>) -> Vec<Instruction> {
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
            16 => Instruction::Const(Primitive::Int(u1(&code, &mut pc) as i32)),
            17 => Instruction::Const(Primitive::Int(u2(&code, &mut pc) as i32)),
            18 => Instruction::LoadConst(u1(&code, &mut pc)),
            19 => Instruction::LoadConst(u2(&code, &mut pc)),
            20 => Instruction::LoadConst(u2(&code, &mut pc)),
            21 => Instruction::Load(u1(&code, &mut pc), PrimitiveType::Int),
            22 => Instruction::Load(u1(&code, &mut pc), PrimitiveType::Long),
            23 => Instruction::Load(u1(&code, &mut pc), PrimitiveType::Float),
            24 => Instruction::Load(u1(&code, &mut pc), PrimitiveType::Double),
            25 => Instruction::Load(u1(&code, &mut pc), PrimitiveType::Reference),
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
            54 => Instruction::Store(u1(&code, &mut pc), PrimitiveType::Int),
            55 => Instruction::Store(u1(&code, &mut pc), PrimitiveType::Long),
            56 => Instruction::Store(u1(&code, &mut pc), PrimitiveType::Float),
            57 => Instruction::Store(u1(&code, &mut pc), PrimitiveType::Double),
            58 => Instruction::Store(u1(&code, &mut pc), PrimitiveType::Reference),
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
            132 => Instruction::IInc(u1(&code, &mut pc), u1(&code, &mut pc) as i8),
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
            153 => Instruction::If(u2(&code, &mut pc), Comparison::Equal),
            154 => Instruction::If(u2(&code, &mut pc), Comparison::NotEqual),
            155 => Instruction::If(u2(&code, &mut pc), Comparison::LessThan),
            156 => Instruction::If(u2(&code, &mut pc), Comparison::GreaterThanOrEqual),
            157 => Instruction::If(u2(&code, &mut pc), Comparison::GreaterThan),
            158 => Instruction::If(u2(&code, &mut pc), Comparison::LessThanOrEqual),
            159 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::Equal),
            160 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::NotEqual),
            161 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::LessThan),
            162 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::GreaterThanOrEqual),
            163 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::GreaterThan),
            164 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::LessThanOrEqual),
            165 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::Equal),
            166 => Instruction::IfICmp(u2(&code, &mut pc), Comparison::NotEqual),
            167 => Instruction::Goto(u2(&code, &mut pc)),
            168 => Instruction::Jsr(u2(&code, &mut pc)),
            169 => Instruction::Ret(u1(&code, &mut pc)),
            170 => panic!("Unsupported instruction: {}", 170),
            171 => panic!("Unsupported instruction: {}", 171),
            172 => Instruction::Return(PrimitiveType::Int),
            173 => Instruction::Return(PrimitiveType::Long),
            174 => Instruction::Return(PrimitiveType::Float),
            175 => Instruction::Return(PrimitiveType::Double),
            176 => Instruction::Return(PrimitiveType::Reference),
            177 => Instruction::Return(PrimitiveType::Null),
            178 => Instruction::GetStatic(u2(&code, &mut pc) as usize),
            179 => Instruction::PutStatic(u2(&code, &mut pc) as usize),
            180 => Instruction::GetField(u2(&code, &mut pc) as usize),
            181 => Instruction::PutField(u2(&code, &mut pc) as usize),
            182 => Instruction::InvokeVirtual(u2(&code, &mut pc) as usize),
            183 => Instruction::InvokeSpecial(u2(&code, &mut pc) as usize),
            184 => Instruction::InvokeStatic(u2(&code, &mut pc) as usize),
            185 => Instruction::InvokeInterface(u2(&code, &mut pc) as usize),
            186 => Instruction::InvokeDynamic(u2(&code, &mut pc) as usize),
            187 => Instruction::New(u2(&code, &mut pc) as usize),
            188 => Instruction::NewArray(PrimitiveType::from_type_id(u1(&code, &mut pc)).unwrap()),
            189 => Instruction::ANewArray(PrimitiveType::from_type_id(u2(&code, &mut pc)).unwrap()),
            190 => Instruction::ArrayLength,
            191 => Instruction::AThrow,
            192 => Instruction::CheckCast(u2(&code, &mut pc) as usize),
            193 => Instruction::InstanceOf(u2(&code, &mut pc) as usize),
            194 => Instruction::MonitorEnter,
            195 => Instruction::MonitorExit,
            196 => panic!("Unsupported instruction: {}", 196),
            197 => panic!("Unsupported instruction: {}", 197),
            198 => Instruction::IfNull(u2(&code, &mut pc) as usize),
            199 => Instruction::IfNonNull(u2(&code, &mut pc) as usize),
            200 => Instruction::Goto(u4(&code, &mut pc) as usize),
            201 => Instruction::Jsr(u4(&code, &mut pc) as usize),
            202 => Instruction::Breakpoint,
            _ => panic!("unsupported instruction"),
        });

        for _ in past_byte_pos..pc {
            instructions.push(Instruction::Nop);
        }

        pc += 1;
        past_byte_pos = pc;
    }

    instructions
}

pub fn parse_file_to_class(filename: String) -> Class {
    let mut r = Reader::new(filename);

    let magic = r.g4();

    if magic != 0xCAFEBABE {
        panic!("invalid magic number");
    }

    let _minor_version = r.g2();
    let _major_version = r.g2();

    let constant_pool_count = r.g2();
    let constant_pool = parse_constant_pool(&mut r, constant_pool_count);

    let _access_flags = ClassFlags::parse(r.g2());
    let this_class = r.g2();
    let _super_class = r.g2();

    let interfaces_count = r.g2();
    let _interfaces = parse_interfaces(&mut r, interfaces_count);

    let fields_count = r.g2();
    let _fields = parse_fields(&mut r, &constant_pool, fields_count);

    let methods_count = r.g2();
    let unparsed_methods = parse_methods(&mut r, &constant_pool, methods_count);

    let attributes_count = r.g2();
    let _attributes = parse_attributes(&mut r, &constant_pool, attributes_count);

    let name_as_cpe = &constant_pool[this_class as usize - 1];
    let name = match name_as_cpe {
        ConstantPoolEntry::Class(name_index) => match &constant_pool[*name_index as usize - 1] {
            ConstantPoolEntry::Utf8(name_as_utf8) => name_as_utf8.clone(),
            _ => panic!("this_class is not a Utf8Info"),
        },
        _ => panic!("this_class is not a ClassInfo"),
    };

    let mut methods: HashMap<String, Method> = HashMap::new();

    for up_method in unparsed_methods {
        let name_as_cpe = &constant_pool[up_method.name_index as usize - 1];

        let name = match name_as_cpe {
            ConstantPoolEntry::Utf8(name_as_utf8) => name_as_utf8.clone(),
            _ => panic!("method name is not a Utf8Info"),
        };

        let signature = match &constant_pool[up_method.descriptor_index as usize - 1] {
            ConstantPoolEntry::Utf8(signature_as_utf8) => signature_as_utf8.clone(),
            _ => panic!("method signature is not a Utf8Info"),
        };

        let name_and_signature = format!("{}{}", name, signature);

        let unparsed_attribute = &up_method.attributes[0];

        let code_attribute = match unparsed_attribute {
            Attribute::Code(code_attribute) => code_attribute,
            _ => panic!("method attribute is not a CodeAttribute"),
        };

        let parsed_bytecode = bytes_to_bytecode(code_attribute.code.clone());

        let parsed_method = Method {
            instructions: parsed_bytecode,
        };

        methods.insert(name_and_signature, parsed_method);
    }

    Class {
        name,
        constant_pool,
        static_fields: HashMap::new(),
        methods,
    }
}
