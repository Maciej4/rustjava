use crate::{class_file_parser, javac, jvm};

/// Javac Tests

#[test]
fn add_test() {
    compile_and_run_test("Add.java", "37");
}

#[test]
fn array_test() {
    compile_and_run_test("Array.java", "10");
}

#[test]
fn hello_world_test() {
    compile_and_run_test("HelloWorld.java", "1");
}

#[test]
fn if_test() {
    compile_and_run_test("If.java", "17");
}

#[test]
fn advanced_if_test() {
    compile_and_run_test("AdvancedIf.java", "17");
}

#[test]
fn main_test() {
    compile_and_run_test("Main.java", "17");
}

// TODO: Test multiple classes

/// JVM Tests

#[test]
fn add_class_file_test() {
    test_class("Add.class", "37");
}

#[test]
fn array_class_file_test() {
    test_class("Array.class", "10");
}

#[test]
fn hello_world_class_file_test() {
    test_class("HelloWorld.class", "1");
}

#[test]
fn if_class_file_test() {
    test_class("If.class", "17");
}

#[test]
fn advanced_if_class_file_test() {
    test_class("AdvancedIf.class", "17");
}

#[test]
fn main_class_file_test() {
    test_class("Main.class", "17");
}

#[test]
fn class_class_file_test() {
    test_class_set(vec!["ClassTest.class", "Point.class"], "90");
}

fn test_class(class_name: &str, expected: &str) {
    println!("Running {} | Expected {} and got: ", class_name, expected);

    let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);

    let classes = vec![class_file_parser::parse_file_to_class(class_name_and_path)];

    let mut jvm = jvm::Jvm::new(classes);

    match jvm.run() {
        Ok(_) => {}
        Err(e) => println!("\n\x1b[31m{}\x1b[0m", jvm.stack_trace(e)),
    };

    assert!(jvm.stdout.eq(expected));
}

fn test_class_set(class_names: Vec<&str>, expected: &str) {
    let mut classes = vec![];

    println!(
        "Running {} | Expected {} and got: ",
        class_names.first().unwrap(),
        expected
    );

    for class_name in class_names {
        let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);
        classes.push(class_file_parser::parse_file_to_class(class_name_and_path));
    }

    let mut jvm = jvm::Jvm::new(classes);

    match jvm.run() {
        Ok(_) => {}
        Err(e) => println!("\n\x1b[31m{}\x1b[0m", jvm.stack_trace(e)),
    };

    assert!(jvm.stdout.eq(expected));
}

// Compile and run the resulting class file with the JVM, and compare the output to the expected output.
fn compile_and_run_test(class_name: &str, expected: &str) {
    print!("Running {} | Expected {} and got: ", class_name, expected);

    let class_name_and_path = format!(".\\src\\java_tests\\{}", class_name);

    let class_code = std::fs::read_to_string(class_name_and_path).unwrap();

    let classes = match javac::parse_to_class(class_code) {
        Ok(classes) => classes,
        Err(e) => {
            panic!("\n\x1b[31m{}\x1b[0m", e);
        }
    };

    let mut jvm = jvm::Jvm::new(classes);

    match jvm.run() {
        Ok(_) => {}
        Err(e) => println!("\n\x1b[31m{}\x1b[0m", jvm.stack_trace(e)),
    };

    assert!(jvm.stdout.eq(expected));
}
