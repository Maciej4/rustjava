use crate::bytecode::*;

mod bytecode;
mod class_file_parser;
mod java_class;
mod jvm;
mod reader;

fn test_class(class_name: &str, expected: &str) {
    let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);

    let classes = vec![class_file_parser::parse_file_to_class(class_name_and_path)];

    print!("Running {} | expected: {} and got: ", class_name, expected);

    let mut jvm = jvm::Jvm::new(classes);

    jvm.run();
}

fn main() {
    test_class("Add.class", "[Int(37)]");
    test_class("Array.class", "[Int(10)]");
    test_class("HelloWorld.class", "[Int(1)]");
    test_class("If.class", "[Int(17)]");
    test_class("Main.class", "[Int(17)]");
}
