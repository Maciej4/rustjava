use crate::bytecode::*;

mod bytecode;
mod class_file_parser;
mod java_class;
mod jvm;
mod reader;

fn main() {
    let classes = vec![class_file_parser::parse_file_to_class(
        ".\\src\\java_tests\\Array.class",
    )];

    println!("{:?}", classes);

    let mut jvm = jvm::JVM::new(classes);

    jvm.run();
}
