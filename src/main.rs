extern crate core;

use crate::bytecode::*;

mod bytecode;
mod class_file_parser;
mod java_class;
mod javac;
mod jvm;
mod reader;
#[cfg(test)]
mod tests;

fn main() {
    let code = include_str!("java_tests/AdvancedIf.java");

    let classes = match javac::parse_to_class(code.to_string()) {
        Ok(classes) => classes,
        Err(e) => {
            println!("\x1b[31mError: {}\x1b[0m", e);
            return;
        }
    };

    println!("jvm has classes: {:?}", classes);
    let mut jvm = jvm::Jvm::new(classes);

    println!("\nRunning JVM:");
    match jvm.run() {
        Ok(_) => {}
        Err(e) => println!("\n\x1b[31m{}\x1b[0m", jvm.stack_trace(e)),
    };
}
