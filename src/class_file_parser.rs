/// This module contains the code for the java class file parser.
use crate::class_file_parser::reader::Reader;

/// A utility for reading files byte by byte.
pub mod reader {
    use std::fs;
    use std::fs::File;
    use std::io::Read;

    /// Allows for easy reading of the raw bytes of a file.
    pub struct Reader {
        pub bytes: Vec<u8>,
        pub index: usize,
    }

    /// Creates a reader for a given file.
    pub fn new(filename: &str) -> Reader {
        let filename_string = filename.to_string();
        let mut f = File::open(&filename_string).expect("no file found");
        let metadata = fs::metadata(&filename_string).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");

        Reader {
            bytes: buffer,
            index: 0,
        }
    }

    impl Reader {
        /// Reads and advances a single byte.
        pub fn g1(&mut self) -> u8 {
            self.index += 1;
            self.bytes[self.index - 1]
        }

        /// Reads and advances two bytes.
        pub fn g2(&mut self) -> u16 {
            (self.g1() as u16) << 8 | (self.g1() as u16)
        }

        /// Reads and advances four bytes.
        pub fn g4(&mut self) -> u32 {
            (self.g1() as u32) << 24
                | (self.g1() as u32) << 16
                | (self.g1() as u32) << 8
                | (self.g1() as u32)
        }

        /// Reads and advances eight bytes.
        pub fn g8(&mut self) -> u64 {
            (self.g1() as u64) << 56
                | (self.g1() as u64) << 48
                | (self.g1() as u64) << 40
                | (self.g1() as u64) << 32
                | (self.g1() as u64) << 24
                | (self.g1() as u64) << 16
                | (self.g1() as u64) << 8
                | (self.g1() as u64)
        }

        /// Reads and advances a passed number of bytes.
        pub fn g(&mut self, size: usize) -> Vec<u8> {
            self.index += size;
            self.bytes[self.index - size..self.index].to_vec()
        }

        /// Read the current index.
        pub fn pos(&self) -> usize {
            self.index
        }

        /// Set the current index to a given value.
        pub fn set_pos(&mut self, pos: usize) {
            self.index = pos;
        }
    }
}

struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<ConstantPoolEntry>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<Interface>,
    fields_count: u16,
    fields: Vec<Field>,
    methods_count: u16,
    methods: Vec<Method>,
    attributes_count: u16,
    attributes: Vec<Attribute>,
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

struct Interface {
    name: u16,
}

struct Field {
    access_flags: u16,
    name: u16,
    descriptor: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
}

#[derive(Debug)]
struct ConstantValueAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    constant_value_index: u16,
}

#[derive(Debug)]
struct CodeAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: Vec<u8>,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct StackMapTableAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_entries: u16,
    entries: Vec<u8>,
}

#[derive(Debug)]
struct ExceptionsAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_exceptions: u16,
    exception_index_table: Vec<u8>,
}

#[derive(Debug)]
struct SourceFileAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    sourcefile_index: u16,
}

#[derive(Debug)]
struct LineNumberTableElement {
    start_pc: u16,
    line_number: u16,
}

#[derive(Debug)]
struct LineNumberTableAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    line_number_table_length: u16,
    line_number_table: Vec<LineNumberTableElement>,
}

fn parse_constant_pool(r: &mut Reader, constant_pool_count: u16) -> Vec<ConstantPoolEntry> {
    let mut constant_pool = Vec::new();

    for _ in 1..constant_pool_count {
        constant_pool.push(match r.g1() {
            1 => {
                let length = r.g2() as usize;
                ConstantPoolEntry::Utf8(String::from_utf8(r.g(length)).unwrap())
            }
            3 => ConstantPoolEntry::Integer(r.g4() as i32),
            4 => ConstantPoolEntry::Float(r.g4() as f32),
            5 => ConstantPoolEntry::Long(r.g8() as i64),
            6 => ConstantPoolEntry::Double(r.g8() as f64),
            7 => ConstantPoolEntry::Class(r.g2()),
            8 => ConstantPoolEntry::String(r.g2()),
            9 => ConstantPoolEntry::FieldRef(r.g2(), r.g2()),
            10 => ConstantPoolEntry::MethodRef(r.g2(), r.g2()),
            11 => ConstantPoolEntry::InterfaceMethodRef(r.g2(), r.g2()),
            12 => ConstantPoolEntry::NameAndType(r.g2(), r.g2()),
            15 => ConstantPoolEntry::MethodHandle(r.g1(), r.g2()),
            16 => ConstantPoolEntry::MethodType(r.g2()),
            18 => ConstantPoolEntry::InvokeDynamic(r.g2(), r.g2()),
            _ => panic!("unsupported constant pool entry"),
        });
    }

    constant_pool
}

fn parse_interfaces(r: &mut Reader, interfaces_count: u16) -> Vec<Interface> {
    let mut interfaces = Vec::new();

    for _ in 0..interfaces_count {
        interfaces.push(Interface { name: r.g2() });
    }

    interfaces
}

fn parse_fields(r: &mut Reader, ct: &[ConstantPoolEntry], fields_count: u16) -> Vec<Field> {
    let mut fields = Vec::new();

    for _ in 0..fields_count {
        let access_flags = r.g2();
        let name = r.g2();
        let descriptor = r.g2();
        let attributes_count = r.g2();
        let attributes = parse_attributes(r, ct, attributes_count);

        fields.push(Field {
            access_flags,
            name,
            descriptor,
            attributes_count,
            attributes,
        });
    }

    fields
}

fn parse_methods(r: &mut Reader, ct: &[ConstantPoolEntry], methods_count: u16) -> Vec<Method> {
    let mut methods = Vec::new();

    for _i in 0..methods_count {
        let access_flags = r.g2();
        let name_index = r.g2();
        let descriptor_index = r.g2();
        let attributes_count = r.g2();
        let attributes = parse_attributes(r, ct, attributes_count);

        methods.push(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        });
    }

    methods
}

fn parse_attributes(
    r: &mut Reader,
    ct: &[ConstantPoolEntry],
    attributes_count: u16,
) -> Vec<Attribute> {
    let mut attributes = Vec::new();

    for _i in 0..attributes_count {
        let attribute_name_index = r.g2() - 1;
        let attribute_length = r.g4();
        let attribute_start_position = r.pos();
        let attribute_str_name = match ct[attribute_name_index as usize] {
            ConstantPoolEntry::Utf8(ref s) => s,
            _ => {
                println!("{:?}", ct[attribute_name_index as usize]);
                panic!("attribute name is not a utf8 string")
            }
        };

        attributes.push(match &attribute_str_name[..] {
            "ConstantValue" => Attribute::ConstantValue(ConstantValueAttribute {
                attribute_name_index,
                attribute_length,
                constant_value_index: r.g2(),
            }),
            "Code" => {
                let max_stack = r.g2();
                let max_locals = r.g2();
                let code_length = r.g4();
                let code = r.g(code_length as usize);
                let exception_table_length = r.g2();
                let exception_table = r.g(exception_table_length as usize);
                let attributes_count = r.g2();
                let attributes = parse_attributes(r, ct, attributes_count);

                Attribute::Code(CodeAttribute {
                    attribute_name_index,
                    attribute_length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    attributes_count,
                    attributes,
                })
            }
            "StackMapTable" => Attribute::StackMapTable(StackMapTableAttribute {
                attribute_name_index,
                attribute_length,
                number_of_entries: r.g2(),
                entries: r.g(attribute_length as usize),
            }),
            "Exceptions" => Attribute::Exceptions(ExceptionsAttribute {
                attribute_name_index,
                attribute_length,
                number_of_exceptions: r.g2(),
                exception_index_table: r.g(attribute_length as usize),
            }),
            "SourceFile" => Attribute::SourceFile(SourceFileAttribute {
                attribute_name_index,
                attribute_length,
                sourcefile_index: r.g2(),
            }),
            "LineNumberTable" => {
                let line_number_table_length = r.g2();
                let mut line_number_table = Vec::new();

                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberTableElement {
                        start_pc: r.g2(),
                        line_number: r.g2(),
                    });
                }

                Attribute::LineNumberTable(LineNumberTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                })
            }
            _ => panic!("{} is an unsupported attribute type", attribute_str_name),
        });

        if r.pos() != attribute_start_position + attribute_length as usize {
            panic!("attribute was not parsed completely");
        }

        r.set_pos(attribute_start_position + attribute_length as usize);
    }

    attributes
}

fn pretty_print_class(cf: &ClassFile) {
    println!("magic: {:X}", cf.magic);
    println!("minor_version: {}", cf.minor_version);
    println!("major_version: {}", cf.major_version);
    println!("CONSTANT POOL ({} items):", cf.constant_pool_count);
    for (i, c) in cf.constant_pool.iter().enumerate() {
        println!("  {:0width$} | {:?}", i, c, width = 2);
    }
    println!("access_flags: {}", cf.access_flags);
    println!("this_class: {}", cf.this_class);
    println!("super_class: {}", cf.super_class);
    println!("INTERFACES ({} items):", cf.interfaces_count);
    for i in &cf.interfaces {
        println!("  {}", i.name);
    }
    println!("FIELDS ({} items):", cf.fields_count);
    for f in &cf.fields {
        println!("  access_flags: {}", f.access_flags);
        println!("  name: {}", f.name);
        println!("  descriptor: {}", f.descriptor);
        println!("  attributes_count: {}", f.attributes_count);
        for a in &f.attributes {
            println!("    {:?}", a);
        }
    }
    println!("METHODS ({} items):", cf.methods_count);
    for (i, m) in cf.methods.iter().enumerate() {
        println!("  method {:#?}:", i);
        println!("      access_flags: {}", m.access_flags);
        println!("      name: {}", m.name_index);
        println!("      descriptor: {}", m.descriptor_index);
        println!("      attributes_count: {}", m.attributes_count);
        println!("      attributes:");
        for a in &m.attributes {
            println!("          {:?}", a);
        }
    }

    println!("ATTRIBUTES ({} items):", cf.attributes_count);
    for a in &cf.attributes {
        println!("  {:?}", a);
    }
}

pub fn parse_file_test() {
    let mut r =
        reader::new("C:\\Users\\m\\CLionProjects\\rustjava\\src\\java_tests\\HelloWorld.class");

    let magic = r.g4();
    let minor_version = r.g2();
    let major_version = r.g2();

    let constant_pool_count = r.g2();
    let constant_pool = parse_constant_pool(&mut r, constant_pool_count);

    let access_flags = r.g2();
    let this_class = r.g2();
    let super_class = r.g2();

    let interfaces_count = r.g2();
    let interfaces = parse_interfaces(&mut r, interfaces_count);

    let fields_count = r.g2();
    let fields = parse_fields(&mut r, &constant_pool, fields_count);

    let methods_count = r.g2();
    let methods = parse_methods(&mut r, &constant_pool, methods_count);

    let attributes_count = r.g2();
    let attributes = parse_attributes(&mut r, &constant_pool, attributes_count);

    let cf = ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool_count,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes,
    };

    // println!("{:?}", cf);
    pretty_print_class(&cf);
}
