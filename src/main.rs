// use tree_sitter::{Parser};
//
// fn print_ast(start: tree_sitter::Node) {
//     let mut tree_cursor = start.walk();
//     let mut stack = vec![start];
//     while let Some(node) = stack.pop() {
//         println!("{:?}", node);
//         for child in node.children(&mut tree_cursor) {
//             stack.push(child);
//         }
//     }
// }

// let code = include_str!("java_tests/Test.java");
// println!("{}", code);
//
// let mut parser = Parser::new();
// parser.set_language(tree_sitter_java::language()).expect("Error loading Java grammar");
// let tree = parser.parse(code, None).unwrap();
// let root_node = tree.root_node();
//
// println!("{:?}", root_node);
// println!("{:?}", root_node.child_count());
// println!("{:?}", root_node.child(0));
// println!();
//
// print_ast(root_node);

mod classfile_parser;

extern crate core;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    buffer
}

// fn b2us(bytes: Vec<u8>, s: usize, e: usize) -> u16 {
//     let mut result = 0;
//     for i in s..e {
//         result = result << 8;
//         result = result | bytes[i] as u16;
//     }
//     result
// }

fn b2us(bytes: Vec<u8>) -> u16 {
    (bytes[0] as u16) << 8 | (bytes[1] as u16)
}

fn b2i(bytes: Vec<u8>) -> i32 {
    let mut integer = 0;
    for byte in bytes {
        integer += byte as i32;
    }
    integer
}

fn b2l(bytes: Vec<u8>) -> i64 {
    let mut integer = 0;
    for byte in bytes {
        integer += byte as i64;
    }
    integer
}

#[derive(Debug)]
enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    InvokeDynamic(u16, u16),
}

fn parse_constant_pool(bytes: Vec<u8>) -> usize {
    let constant_pool_count = b2i(bytes[8..10].to_vec());

    let mut constant_pool: Vec<ConstantPoolEntry> = Vec::new();

    let mut byte_index = 10;

    for _i in 1..constant_pool_count {
        match bytes[byte_index] {
            1 => {
                let length: usize = b2i(bytes[byte_index + 1..byte_index + 3].to_vec()) as usize;
                let utf8 = str::from_utf8(&bytes[byte_index + 3..byte_index + 3 + length])
                    .unwrap()
                    .to_string();
                constant_pool.push(ConstantPoolEntry::Utf8(utf8));
                byte_index += 3 + length;
            }
            3 => {
                constant_pool.push(ConstantPoolEntry::Integer(b2i(bytes
                    [byte_index + 1..byte_index + 5]
                    .to_vec())));
                byte_index += 5;
            }
            4 => {
                constant_pool
                    .push(ConstantPoolEntry::Float(b2i(
                        bytes[byte_index + 1..byte_index + 5].to_vec()
                    ) as f32));
                byte_index += 5;
            }
            5 => {
                constant_pool.push(ConstantPoolEntry::Long(b2l(bytes
                    [byte_index + 1..byte_index + 9]
                    .to_vec())));
                byte_index += 9;
            }
            6 => {
                constant_pool
                    .push(ConstantPoolEntry::Double(b2l(
                        bytes[byte_index + 1..byte_index + 9].to_vec()
                    ) as f64));
                byte_index += 9;
            }
            7 => {
                constant_pool.push(ConstantPoolEntry::Class(b2us(
                    bytes[byte_index + 1..byte_index + 3].to_vec(),
                )));
                byte_index += 3;
            }
            8 => {
                constant_pool.push(ConstantPoolEntry::String(b2us(
                    bytes[byte_index + 1..byte_index + 3].to_vec(),
                )));
                byte_index += 3;
            }
            9 => {
                constant_pool.push(ConstantPoolEntry::FieldRef(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()),
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            10 => {
                constant_pool.push(ConstantPoolEntry::MethodRef(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()),
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            11 => {
                constant_pool.push(ConstantPoolEntry::InterfaceMethodRef(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()),
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            12 => {
                constant_pool.push(ConstantPoolEntry::NameAndType(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()),
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            15 => {
                constant_pool.push(ConstantPoolEntry::MethodHandle(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()) as u8,
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            16 => {
                constant_pool.push(ConstantPoolEntry::MethodType(b2us(
                    bytes[byte_index + 1..byte_index + 3].to_vec(),
                )));
                byte_index += 3;
            }
            18 => {
                constant_pool.push(ConstantPoolEntry::InvokeDynamic(
                    b2us(bytes[byte_index + 1..byte_index + 3].to_vec()),
                    b2us(bytes[byte_index + 3..byte_index + 5].to_vec()),
                ));
                byte_index += 5;
            }
            _ => panic!("not implemented"),
        }
        println!("  {} : {:?}", _i, constant_pool[constant_pool.len() - 1]);
    }

    byte_index
}

#[derive(Debug)]
struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Attribute {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u32,
    exception_table: Vec<u8>,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

// fn parse_attribute(bytes: Vec<u8>) -> Attribute {
//     Attribute {
//         attribute_name_index: b2us(bytes[0..2].to_vec()),
//         attribute_length: b2i(bytes[2..6].to_vec()) as u32,
//         max_stack: b2us(bytes[6..8].to_vec()),
//         max_locals: b2us(bytes[8..10].to_vec()),
//         code_length: b2i(bytes[10..14].to_vec()) as u32,
//         code: bytes[14..14 + b2i(bytes[10..14].to_vec()) as usize].to_vec(),
//         exception_table_length: b2i(bytes[14 + b2i(bytes[10..14].to_vec()) as usize..18].to_vec())
//             as u32,
//         exception_table: bytes
//             [18..18 + b2i(bytes[14 + b2i(bytes[10..14].to_vec()) as usize..18].to_vec()) as usize]
//             .to_vec(),
//         attributes_count: b2us(
//             bytes[18 + b2i(bytes[14 + b2i(bytes[10..14].to_vec()) as usize..18].to_vec()) as usize
//                 ..20]
//                 .to_vec(),
//         ),
//         attributes: bytes[20..20
//             + b2i(bytes[18
//                 + b2i(bytes[14 + b2i(bytes[10..14].to_vec()) as usize..18].to_vec()) as usize
//                 ..20]
//                 .to_vec()) as usize]
//             .to_vec(),
//     }
// }

fn parse_code_attribute(bytes: Vec<u8>) -> (Attribute, usize) {
    let code_length_bytes = b2i(bytes[10..14].to_vec()) as usize;
    let exception_table_length_bytes = (b2i(
        bytes[(14 + code_length_bytes)..(14 + code_length_bytes + 2)]
            .to_vec()
    ) * 8) as usize;

    (Attribute {
        attribute_name_index: b2us(bytes[0..2].to_vec()),
        attribute_length: b2i(bytes[2..6].to_vec()) as u32,
        max_stack: b2us(bytes[6..8].to_vec()),
        max_locals: b2us(bytes[8..10].to_vec()),
        code_length: code_length_bytes as u32,
        code: bytes[14..(14 + code_length_bytes as usize)].to_vec(),
        exception_table_length: b2i(
            bytes[(14 + code_length_bytes)..(14 + code_length_bytes + 2)]
                .to_vec()
        ) as u32,
        exception_table: Vec::new(),
        attributes_count: b2us(
            bytes[(14 + 2 + code_length_bytes + exception_table_length_bytes)..(14 + 2 + code_length_bytes + exception_table_length_bytes + 2)]
                .to_vec()
        ),
        attributes: Vec::new(),
    }, b2i(bytes[2..6].to_vec()) as usize + 6)
}

fn parse_method(bytes: Vec<u8>) -> (Method, usize) {
    let attributes_count = b2us(bytes[6..8].to_vec());
    let mut attributes = Vec::new();
    let mut byte_index: usize = 8;

    for _i in 0..attributes_count {
        let (attribute, attribute_size) = parse_code_attribute(bytes[byte_index..].to_vec());
        attributes.push(attribute);
        byte_index += attribute_size;
    }

    (Method {
        access_flags: b2us(bytes[0..2].to_vec()),
        name_index: b2us(bytes[2..4].to_vec()),
        descriptor_index: b2us(bytes[4..6].to_vec()),
        attributes_count: attributes_count,
        attributes: attributes,
    }, byte_index)
}

fn parse_methods(bytes: Vec<u8>, methods_count: u16) -> (Vec<Method>, usize) {
    let mut methods: Vec<Method> = Vec::new();
    let mut byte_index = 0;
    for _i in 0..methods_count {
        let (method, method_size) = parse_method(bytes[byte_index..].to_vec());
        byte_index += method_size;
        println!("{:?}", method);
        methods.push(method);
    }

    (methods, byte_index)
}

fn main() {
    let target_file =
        String::from("C:\\Users\\m\\CLionProjects\\rustjava\\src\\java_tests\\HelloWorld.class");

    let code = get_file_as_byte_vec(&target_file);

    // Magic
    println!("Magic (4 bytes): {:X?}", code[0..4].to_vec());

    // Version
    println!("Minor version (2 bytes): {}", b2i(code[4..6].to_vec()));
    println!("Major version (2 bytes): {}", b2i(code[6..8].to_vec()));

    // Constant Pool
    println!(
        "Constant pool count (2 bytes): {}",
        b2i(code[8..10].to_vec())
    );
    println!("Constant pool:");
    let mut pl_e = parse_constant_pool(code.clone());

    // Access flags
    println!(
        "Access flags (2 bytes): {}",
        b2i(code[pl_e..pl_e + 2].to_vec())
    );
    println!(
        "This class (2 bytes): {}",
        b2i(code[pl_e + 2..pl_e + 4].to_vec())
    );
    println!(
        "Super class (2 bytes): {}",
        b2i(code[pl_e + 4..pl_e + 6].to_vec())
    );

    // Interfaces
    println!(
        "Interfaces count (2 bytes): {}",
        b2us(code[pl_e + 6..pl_e + 8].to_vec())
    );
    // TODO: parse interfaces
    pl_e += 8;

    // Fields
    println!(
        "Fields count (2 bytes): {}",
        b2us(code[pl_e..pl_e + 2].to_vec())
    );
    // TODO: parse fields
    pl_e += 2;

    // Methods
    let method_count = b2us(code[pl_e..pl_e + 2].to_vec());
    println!("Methods count (2 bytes): {}", method_count);
    pl_e += 2;
    let (methods, methods_length_bytes) = parse_methods(code[pl_e..].to_vec(), method_count);
    println!("{:?}", methods);
    pl_e += methods_length_bytes;

    println!("{:?}", code[pl_e..].to_vec());
}
