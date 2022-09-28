# RustJava

The goal of this project is to implement a small subset of the JVM in Rust to learn more about both the JVM and Rust.

# Current Features
* [x] Class file parsing
* [ ] Class file verification or any other kind of validation
* [x] Basic JVM capable of running multiple classes
  * [x] Local variable stores and loads
  * [x] Stack manipulation
  * [x] Arithmetic operations
  * [x] Array operations
  * [x] Object creation
  * [x] Method invocation
  * [x] Method return
  * [x] Field access
  * [x] Control flow
  * [ ] Exception handling
    * Currently, there is no support for exception handling. This means that any issue will cause the JVM to panic.
  * [ ] Synchronization (monitors)
  * [ ] Interface invocation
  * [ ] Dynamic invocation
  * [ ] Table and lookup switch
  * [ ] FCMPG, FCMPL, DCMPL, and DCMPG instructions
  * [ ] Type checked instructions and arrays
    * Some instructions do check that the item on the stack is of the correct type. However, this is not implemented for all instructions.
* [ ] Standard libraries
* [ ] Garbage collection
* [ ] Java to JVM bytecode compiler
  * [x] Code parsing into AST (using tree sitter)
  * [ ] Constant pool generation
  * [ ] Local variable stores and loads
  * [ ] Stack manipulation
  * [ ] Method calls
  * [ ] Handling improper code
  * [ ] Compiler errors
  * [ ] Linting

# Key files

* `src/main.rs` - The entry point of the program.
* `src/bytecode.rs` - Contains the bytecode instructions and some utility functions.
* `src/class_file_parser.rs` - Parses class files into a `Class` struct for use by the JVM.
* `src/java_class.rs` - The Class struct, which represents a Java class.
* `src/javac.rs` - Compiles Java source code into class files.
* `src/jvm.rs` - The JVM implementation.
* `src/reader.rs` - A utility for reading files byte by byte, which is used by the class file parser.
