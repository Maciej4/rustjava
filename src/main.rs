use crate::bytecode::{Bytecode, Primitive};

mod bytecode;
mod class_file_parser;
mod java_class;
mod reader;

fn main() {
    let cf = class_file_parser::parse_file(
        "C:\\Users\\m\\CLionProjects\\rustjava\\src\\java_tests\\Main.class",
    );
    println!("{}", cf);

    let mut bytecodes = Vec::new();

    for method in cf.methods {
        bytecodes.push(Bytecode::new(method.get_code_attribute()));
    }

    // for bytecode in bytecodes {
    //     println!("{:?}", bytecode);
    // }

    println!("{:?}", bytecodes[1]);

    bytecodes[1].local_variables.push(Primitive::Int(1));
    bytecodes[1].local_variables.push(Primitive::Int(2));

    bytecodes[1].run();
}
