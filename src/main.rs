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

    let mut pl_i = 10;

    for _i in 1..constant_pool_count {
        match bytes[pl_i] {
            1 => {
                let length: usize = b2i(bytes[pl_i + 1..pl_i + 3].to_vec()) as usize;
                let utf8 = str::from_utf8(&bytes[pl_i + 3..pl_i + 3 + length])
                    .unwrap()
                    .to_string();
                constant_pool.push(ConstantPoolEntry::Utf8(utf8));
                pl_i += 3 + length;
            }
            3 => {
                constant_pool.push(ConstantPoolEntry::Integer(b2i(
                    bytes[pl_i + 1..pl_i + 5].to_vec()
                )));
                pl_i += 5;
            }
            4 => {
                constant_pool.push(ConstantPoolEntry::Float(
                    b2i(bytes[pl_i + 1..pl_i + 5].to_vec()) as f32,
                ));
                pl_i += 5;
            }
            5 => {
                constant_pool.push(ConstantPoolEntry::Long(b2l(
                    bytes[pl_i + 1..pl_i + 9].to_vec()
                )));
                pl_i += 9;
            }
            6 => {
                constant_pool.push(ConstantPoolEntry::Double(
                    b2l(bytes[pl_i + 1..pl_i + 9].to_vec()) as f64,
                ));
                pl_i += 9;
            }
            7 => {
                constant_pool.push(ConstantPoolEntry::Class(b2us(
                    bytes[pl_i + 1..pl_i + 3].to_vec(),
                )));
                pl_i += 3;
            }
            8 => {
                constant_pool.push(ConstantPoolEntry::String(b2us(
                    bytes[pl_i + 1..pl_i + 3].to_vec(),
                )));
                pl_i += 3;
            }
            9 => {
                constant_pool.push(ConstantPoolEntry::FieldRef(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()),
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            10 => {
                constant_pool.push(ConstantPoolEntry::MethodRef(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()),
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            11 => {
                constant_pool.push(ConstantPoolEntry::InterfaceMethodRef(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()),
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            12 => {
                constant_pool.push(ConstantPoolEntry::NameAndType(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()),
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            15 => {
                constant_pool.push(ConstantPoolEntry::MethodHandle(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()) as u8,
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            16 => {
                constant_pool.push(ConstantPoolEntry::MethodType(b2us(
                    bytes[pl_i + 1..pl_i + 3].to_vec(),
                )));
                pl_i += 3;
            }
            18 => {
                constant_pool.push(ConstantPoolEntry::InvokeDynamic(
                    b2us(bytes[pl_i + 1..pl_i + 3].to_vec()),
                    b2us(bytes[pl_i + 3..pl_i + 5].to_vec()),
                ));
                pl_i += 5;
            }
            _ => panic!("not implemented"),
        }
        println!("  {} : {:?}", _i, constant_pool[constant_pool.len() - 1]);
    }

    pl_i
}

struct Method {
    name_index: u16,

}

fn parse_method(bytes: Vec<u8>) {
}

fn parse_methods(bytes: Vec<u8>) {

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
    println!("Constant pool count (2 bytes): {}", b2i(code[8..10].to_vec()));
    println!("Constant pool:");
    let mut pl_e = parse_constant_pool(code.clone());

    // Access flags
    println!("Access flags (2 bytes): {}", b2i(code[pl_e..pl_e + 2].to_vec()));
    println!("This class (2 bytes): {}", b2i(code[pl_e + 2..pl_e + 4].to_vec()));
    println!("Super class (2 bytes): {}", b2i(code[pl_e + 4..pl_e + 6].to_vec()));

    // Interfaces
    println!("Interfaces count (2 bytes): {}", b2i(code[pl_e + 6..pl_e + 8].to_vec()));
    // TODO: parse interfaces
    pl_e += 8;

    // Fields
    println!("Fields count (2 bytes): {}", b2i(code[pl_e..pl_e + 2].to_vec()));
    // TODO: parse fields
    pl_e += 2;

    // Methods
    println!("Methods count (2 bytes): {}", b2i(code[pl_e..pl_e + 2].to_vec()));


    println!("{:?}", code[pl_e..].to_vec());
}
