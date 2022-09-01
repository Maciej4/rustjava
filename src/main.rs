use crate::bytecode::*;

mod bytecode;
mod class_file_parser;
mod java_class;
mod reader;

fn main() {
    let cf = class_file_parser::parse_file(".\\src\\java_tests\\If.class");
    println!("{}", cf);

    let mut bytecodes = Vec::new();

    for method in cf.methods {
        bytecodes.push(Bytecode::new(method.get_code_attribute(), cf.constant_pool.clone()));
    }

    // for bytecode in bytecodes {
    //     println!("{:?}", bytecode);
    // }

    println!("{:?}", bytecodes[1]);

    bytecodes[1].run();
}
