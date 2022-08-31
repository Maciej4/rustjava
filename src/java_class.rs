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

impl ClassFile {
    pub fn pretty_print_constant(&self, n: usize) -> String {
        match self.constant_pool[n - 1] {
            ConstantPoolEntry::Utf8(ref s) => format!("[{}] Utf8 {}", n, s),
            ConstantPoolEntry::Integer(i) => format!("[{}] Integer {}", n, i),
            ConstantPoolEntry::Float(f) => format!("[{}] Float {}", n, f),
            ConstantPoolEntry::Long(l) => format!("[{}] Long {}", n, l),
            ConstantPoolEntry::Double(d) => format!("[{}] Double {}", n, d),
            ConstantPoolEntry::Class(i) => {
                let s = self.pretty_print_constant(i as usize);
                format!("[{}] Class {}", n, s)
            }
            ConstantPoolEntry::String(i) => {
                let s = self.pretty_print_constant(i as usize);
                format!("[{}] String {}", n, s)
            }
            ConstantPoolEntry::FieldRef(class_index, name_and_type_index) => {
                let class_name = self.pretty_print_constant(class_index as usize);
                let name_and_type = self.pretty_print_constant(name_and_type_index as usize);
                format!(
                    "[{}] FieldRef {} | {} (field ref)",
                    n, class_name, name_and_type
                )
            }
            ConstantPoolEntry::MethodRef(class_index, name_and_type_index) => {
                let class_name = self.pretty_print_constant(class_index as usize);
                let name_and_type = self.pretty_print_constant(name_and_type_index as usize);
                format!("[{}] MethodRef {} | {}", n, class_name, name_and_type)
            }
            ConstantPoolEntry::InterfaceMethodRef(class_index, name_and_type_index) => {
                let class_name = self.pretty_print_constant(class_index as usize);
                let name_and_type = self.pretty_print_constant(name_and_type_index as usize);
                format!(
                    "[{}] InterfaceMethodRef {} | {}",
                    n, class_name, name_and_type
                )
            }
            ConstantPoolEntry::NameAndType(name_index, descriptor_index) => {
                let name = self.pretty_print_constant(name_index as usize);
                let descriptor = self.pretty_print_constant(descriptor_index as usize);
                format!("[{}] NameAndType {} | {}", n, name, descriptor)
            }
            ConstantPoolEntry::MethodHandle(reference_kind, reference_index) => {
                let reference = self.pretty_print_constant(reference_index as usize);
                format!("[{}] MethodHandle {}", n, reference)
            }
            ConstantPoolEntry::MethodType(descriptor_index) => {
                let descriptor = self.pretty_print_constant(descriptor_index as usize);
                format!("[{}] MethodType {}", n, descriptor)
            }
            ConstantPoolEntry::InvokeDynamic(bootstrap_method_attr_index, name_and_type_index) => {
                let bootstrap_method_attr =
                    self.pretty_print_constant(bootstrap_method_attr_index as usize);
                let name_and_type = self.pretty_print_constant(name_and_type_index as usize);
                format!("[{}] InvokeDynamic {}", n, name_and_type)
            }
        }
    }

    pub fn pretty_print(&self) -> String {
        let mut constant_pool_pretty_vec = Vec::new();
        for i in 1..self.constant_pool_count as usize {
            constant_pool_pretty_vec.push(self.pretty_print_constant(i));
        }

        format!(
            "{:X} v{}.{} | Class: {} | Access flags: {:?}  | Superclass: {}\n\
            Constant Pool ({} entries): \n  {}\n\
            Interfaces ({} entries): {:?}\n\
            Fields ({} entries): {:?}\n\
            Methods ({} entries): {:?}\n\
            Attributes ({} entries): {:?}",
            self.magic,
            self.major_version,
            self.minor_version,
            self.pretty_print_constant(self.this_class as usize),
            self.access_flags,
            self.pretty_print_constant(self.super_class as usize),
            self.constant_pool_count,
            constant_pool_pretty_vec.join("\n  "),
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

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print().fmt(f)
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

impl ClassFlags {
    pub fn parse(flags: u16) -> Vec<ClassFlags> {
        let mut flags_vec = Vec::new();
        if flags & 0x0001 != 0 {
            flags_vec.push(ClassFlags::Public);
        }
        if flags & 0x0010 != 0 {
            flags_vec.push(ClassFlags::Final);
        }
        if flags & 0x0020 != 0 {
            flags_vec.push(ClassFlags::Super);
        }
        if flags & 0x0200 != 0 {
            flags_vec.push(ClassFlags::Interface);
        }
        if flags & 0x0400 != 0 {
            flags_vec.push(ClassFlags::Abstract);
        }
        if flags & 0x1000 != 0 {
            flags_vec.push(ClassFlags::Synthetic);
        }
        if flags & 0x2000 != 0 {
            flags_vec.push(ClassFlags::Annotation);
        }
        if flags & 0x4000 != 0 {
            flags_vec.push(ClassFlags::Enum);
        }
        if flags & 0x8000 != 0 {
            flags_vec.push(ClassFlags::Module);
        }
        flags_vec
    }
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

impl Method {
    pub fn get_code_attribute(&self) -> Vec<u8> {
        for attr in &self.attributes {
            if let Attribute::Code(code_attr) = attr {
                return code_attr.code.clone();
            }
        }
        panic!("No code attribute found")
    }
}

/// Attributes are used to store additional information about a class.
#[derive(Debug)]
pub enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    Signature(SignatureAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    Deprecated(DeprecatedAttribute),
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
    pub classes: Vec<InnerClassElement>,
}

#[derive(Debug)]
pub struct InnerClassElement {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug)]
pub struct SyntheticAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct SignatureAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub signature_index: u16,
}

#[derive(Debug)]
pub struct SourceFileAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub sourcefile_index: u16,
}

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub line_number_table_length: u16,
    pub line_number_table: Vec<LineNumberTableElement>,
}

#[derive(Debug)]
pub struct LineNumberTableElement {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct LocalVariableTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub local_variable_table_length: u16,
    pub local_variable_table: Vec<LocalVariableTableElement>,
}

#[derive(Debug)]
pub struct LocalVariableTableElement {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub local_variable_type_table_length: u16,
    pub local_variable_type_table: Vec<LocalVariableTypeTableElement>,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableElement {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug)]
pub struct DeprecatedAttribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}
