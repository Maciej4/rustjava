use crate::bytecode::*;

mod bytecode;
mod class_file_parser;
mod java_class;
mod jvm;
mod reader;

fn test_class(class_name: &str, expected: &str) {
    let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);

    print!("Running {} | expected: {} and got: ", class_name, expected);

    let classes = vec![class_file_parser::parse_file_to_class(class_name_and_path)];

    let mut jvm = jvm::Jvm::new(classes);

    jvm.run();
}

fn test_class_set(class_names: Vec<&str>, expected: &str) {
    let mut classes = vec![];

    print!(
        "Running {:?} | expected: {} and got: ",
        class_names[0], expected
    );

    for class_name in class_names {
        let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);

        classes.push(class_file_parser::parse_file_to_class(class_name_and_path));
    }

    let mut jvm = jvm::Jvm::new(classes);

    // println!("{:?}", jvm);

    jvm.run();
}

fn main() {
    println!();
    test_class("Add.class", "37");
    test_class("Array.class", "10");
    test_class("HelloWorld.class", "1");
    test_class("If.class", "17");
    test_class("Main.class", "17");
    test_class_set(vec!["ClassTest.class", "Point.class"], "90");
}
