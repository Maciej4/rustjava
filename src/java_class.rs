//! This module contains the data structures used to represent java classes.
use crate::Primitive;

#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(usize),                     // name_index
    String(usize),                    // string_index
    FieldRef(usize, usize),           // class_index, name_and_type_index
    MethodRef(usize, usize),          // class_index, name_and_type_index
    InterfaceMethodRef(usize, usize), // class_index, name_and_type_index
    NameAndType(usize, usize),        // name_index, descriptor_index
    MethodHandle(u8, usize),          // reference_kind, reference_index
    MethodType(usize),                // descriptor_index
    InvokeDynamic(usize, usize),      // bootstrap_method_attr_index, name_and_type_index
}

// TODO: Re-write parsers into ConstantPoolExt
impl ConstantPoolEntry {
    pub fn class_parser(index: usize, constant_pool: &[ConstantPoolEntry]) -> String {
        match &constant_pool[index - 1] {
            ConstantPoolEntry::Class(index) => match &constant_pool[*index as usize - 1] {
                ConstantPoolEntry::Utf8(s) => s.clone(),
                _ => panic!("Invalid constant pool entry"),
            },
            _ => panic!("Invalid constant pool entry"),
        }
    }

    pub fn name_and_type_parser(
        index: usize,
        constant_pool: &[ConstantPoolEntry],
    ) -> (String, String) {
        match &constant_pool[index - 1] {
            ConstantPoolEntry::NameAndType(name_index, descriptor_index) => {
                let name = match &constant_pool[*name_index as usize - 1] {
                    ConstantPoolEntry::Utf8(name) => name.clone(),
                    _ => panic!("Invalid constant pool entry"),
                };
                let descriptor = match &constant_pool[*descriptor_index as usize - 1] {
                    ConstantPoolEntry::Utf8(descriptor) => descriptor.clone(),
                    _ => panic!("Invalid constant pool entry"),
                };
                (name, descriptor)
            }
            _ => panic!("Invalid constant pool entry"),
        }
    }

    pub fn method_ref_parser(
        index: usize,
        constant_pool: &[ConstantPoolEntry],
    ) -> (String, String, String) {
        match &constant_pool[index - 1] {
            ConstantPoolEntry::MethodRef(class_index, name_and_type_index) => {
                let class_name =
                    ConstantPoolEntry::class_parser(*class_index as usize, constant_pool);

                let (method_name, method_type) = ConstantPoolEntry::name_and_type_parser(
                    *name_and_type_index as usize,
                    constant_pool,
                );

                (class_name, method_name, method_type)
            }
            _ => panic!("Invalid constant pool entry"),
        }
    }

    pub fn field_ref_parser(
        index: usize,
        constant_pool: &[ConstantPoolEntry],
    ) -> (String, String, String) {
        match &constant_pool[index - 1] {
            ConstantPoolEntry::FieldRef(class_index, name_and_type_index) => {
                let class_name =
                    ConstantPoolEntry::class_parser(*class_index as usize, constant_pool);

                let (method_name, method_type) = ConstantPoolEntry::name_and_type_parser(
                    *name_and_type_index as usize,
                    constant_pool,
                );

                (class_name, method_name, method_type)
            }
            _ => panic!("Invalid constant pool entry"),
        }
    }

    pub fn get_primitive(&self) -> Primitive {
        match self {
            ConstantPoolEntry::Integer(i) => Primitive::Int(*i),
            ConstantPoolEntry::Float(f) => Primitive::Float(*f),
            ConstantPoolEntry::Long(l) => Primitive::Long(*l),
            ConstantPoolEntry::Double(d) => Primitive::Double(*d),
            ConstantPoolEntry::Class(r) => Primitive::Reference(*r),
            ConstantPoolEntry::String(r) => Primitive::Reference(*r), // TODO: this may be wrong
            ConstantPoolEntry::MethodHandle(_, r) => Primitive::Reference(*r),
            ConstantPoolEntry::MethodType(r) => Primitive::Reference(*r),
            _ => panic!("Unable to convert constant pool entry to loadable primitive"),
        }
    }
}

pub trait ConstantPoolExt {
    fn find_utf8(&self, utf8: &str) -> Option<usize>;
    fn find_class(&self, class_name: &str) -> Option<usize>;
    fn find_name_and_type(&self, name: &str, type_: &str) -> Option<usize>;
    fn find_field_ref(&self, class_name: &str, name: &str, type_: &str) -> Option<usize>;
    fn find_method_ref(&self, class_name: &str, name: &str, type_: &str) -> Option<usize>;
    fn find_or_add_utf8(&mut self, value: &str) -> usize;
    fn find_or_add_class(&mut self, name: &str) -> usize;
    fn find_or_add_name_and_type(&mut self, name: &str, descriptor: &str) -> usize;
    fn find_or_add_method_ref(&mut self, class_name: &str, name: &str, descriptor: &str) -> usize;
    fn find_or_add_field_ref(&mut self, class_name: &str, name: &str, descriptor: &str) -> usize;
}

impl ConstantPoolExt for Vec<ConstantPoolEntry> {
    fn find_utf8(&self, utf8: &str) -> Option<usize> {
        for (i, entry) in self.iter().enumerate() {
            if let ConstantPoolEntry::Utf8(value) = entry {
                if value == utf8 {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn find_class(&self, class_name: &str) -> Option<usize> {
        let class_name_index = self.find_utf8(class_name)?;
        for (i, entry) in self.iter().enumerate() {
            if let ConstantPoolEntry::Class(name_index) = entry {
                if *name_index == class_name_index {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn find_name_and_type(&self, name: &str, descriptor: &str) -> Option<usize> {
        let name_index = self.find_utf8(name)?;
        let type_index = self.find_utf8(descriptor)?;
        for (i, entry) in self.iter().enumerate() {
            if let ConstantPoolEntry::NameAndType(n, t) = entry {
                if *n == name_index && *t == type_index {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn find_field_ref(&self, class_name: &str, name: &str, descriptor: &str) -> Option<usize> {
        let class_index = self.find_class(class_name)?;
        let name_and_type_index = self.find_name_and_type(name, descriptor)?;
        for (i, entry) in self.iter().enumerate() {
            if let ConstantPoolEntry::FieldRef(c, n) = entry {
                if *c == class_index && *n == name_and_type_index {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn find_method_ref(&self, class_name: &str, name: &str, descriptor: &str) -> Option<usize> {
        let class_index = self.find_class(class_name)?;
        let name_and_type_index = self.find_name_and_type(name, descriptor)?;
        for (i, entry) in self.iter().enumerate() {
            if let ConstantPoolEntry::MethodRef(c, n) = entry {
                if *c == class_index && *n == name_and_type_index {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn find_or_add_utf8(&mut self, value: &str) -> usize {
        match self.find_utf8(value) {
            Some(index) => index,
            None => {
                self.push(ConstantPoolEntry::Utf8(value.to_string()));
                self.len()
            }
        }
    }

    fn find_or_add_class(&mut self, name: &str) -> usize {
        match self.find_class(name) {
            Some(index) => index,
            None => {
                let name_index = self.find_or_add_utf8(name);
                self.push(ConstantPoolEntry::Class(name_index));
                self.len()
            }
        }
    }

    fn find_or_add_name_and_type(&mut self, name: &str, descriptor: &str) -> usize {
        match self.find_name_and_type(name, descriptor) {
            Some(index) => index,
            None => {
                let name_index = self.find_or_add_utf8(name);
                let descriptor_index = self.find_or_add_utf8(descriptor);
                self.push(ConstantPoolEntry::NameAndType(name_index, descriptor_index));
                self.len()
            }
        }
    }

    fn find_or_add_method_ref(&mut self, class_name: &str, name: &str, descriptor: &str) -> usize {
        match self.find_method_ref(class_name, name, descriptor) {
            Some(index) => index,
            None => {
                let class_index = self.find_or_add_class(class_name);
                let name_and_type_index = self.find_or_add_name_and_type(name, descriptor);
                self.push(ConstantPoolEntry::MethodRef(
                    class_index,
                    name_and_type_index,
                ));
                self.len()
            }
        }
    }

    fn find_or_add_field_ref(&mut self, class_name: &str, name: &str, descriptor: &str) -> usize {
        match self.find_field_ref(class_name, name, descriptor) {
            Some(index) => index,
            None => {
                let class_index = self.find_or_add_class(class_name);
                let name_and_type_index = self.find_or_add_name_and_type(name, descriptor);
                self.push(ConstantPoolEntry::FieldRef(
                    class_index,
                    name_and_type_index,
                ));
                self.len()
            }
        }
    }
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
pub struct UnparsedMethod {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
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
