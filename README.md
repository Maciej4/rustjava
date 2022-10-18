# RustJava

The goal of this project is to implement a small subset of the JVM in Rust to learn more about both the JVM and Rust.

# Key files

* `src/main.rs` - The entry point of the program.
* `src/bytecode.rs` - Contains the bytecode instructions and some utility functions.
* `src/class_file_parser.rs` - Parses class files into a `Class` struct for use by the JVM.
* `src/java_class.rs` - The Class struct, which represents a Java class.
* `src/javac.rs` - Compiles Java source code into class files.
* `src/jvm.rs` - The JVM implementation.
* `src/reader.rs` - A utility for reading files byte by byte, which is used by the class file parser.
