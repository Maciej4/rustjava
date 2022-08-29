//! This module contains the data structures used to represent java classes.
use std::fmt;

/// The overall structure for a java class.
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantPoolEntry>,
    pub access_flags: Vec<ClassFlags>,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<Interface>,
    pub fields_count: u16,
    pub fields: Vec<Field>,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:X} v{}.{}\n\
            Constant Pool ({} entries): {:?}\n\
            Access Flags: {:?}\n\
            This Class: {} Super class: {}\n\
            Interfaces ({} entries): {:?}\n\
            Fields ({} entries): {:?}\n\
            Methods ({} entries): {:?}\n\
            Attributes ({} entries): {:?}",
            self.magic,
            self.major_version,
            self.minor_version,
            self.constant_pool_count,
            self.constant_pool,
            self.access_flags,
            self.this_class,
            self.super_class,
            self.interfaces_count,
            self.interfaces,
            self.fields_count,
            self.fields,
            self.methods_count,
            self.methods,
            self.attributes_count,
            self.attributes
        )
    }
}

#[derive(Debug)]
pub enum ConstantPoolEntry {
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

#[derive(Debug)]
pub enum ClassFlags {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
    Module = 0x8000,
}

#[derive(Debug)]
pub struct Interface {
    pub name: u16,
}

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name: u16,
    pub descriptor: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

///
/// Attributes are used to store additional information about a class.
///

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
}

#[derive(Debug)]
pub struct ConstantValueAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub constant_value_index: u16,
}

#[derive(Debug)]
pub struct CodeAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<u8>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct StackMapTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_entries: u16,
    pub entries: Vec<u8>,
}

#[derive(Debug)]
pub struct ExceptionsAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_exceptions: u16,
    pub exception_index_table: Vec<u8>,
}

#[derive(Debug)]
pub struct InnerClassesAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub number_of_classes: u16,
    pub classes: Vec<u8>,
}

#[derive(Debug)]
pub struct SourceFileAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub sourcefile_index: u16,
}

#[derive(Debug)]
pub struct LineNumberTableElement {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableElement>,
}
