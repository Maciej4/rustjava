//! This module contains the data structures used to represent java classes.
use crate::Primitive;

// TODO: indexes in the constant pool should be usize rather than u16
#[derive(Debug, Clone)]
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
            ConstantPoolEntry::Class(r) => Primitive::Reference(*r as usize),
            ConstantPoolEntry::String(r) => Primitive::Reference(*r as usize), // TODO: this may be wrong
            ConstantPoolEntry::MethodHandle(_, r) => Primitive::Reference(*r as usize),
            ConstantPoolEntry::MethodType(r) => Primitive::Reference(*r as usize),
            _ => panic!("Unable to convert constant pool entry to loadable primitive"),
        }
    }

    // TODO: make this less awful

    pub fn find_class(constant_pool: &[ConstantPoolEntry], class_name: &str) -> u16 {
        for (i, entry) in constant_pool.iter().enumerate() {
            if let ConstantPoolEntry::Class(index) = entry {
                if let ConstantPoolEntry::Utf8(name) = &constant_pool[*index as usize - 1] {
                    if name == class_name {
                        return i as u16 + 1;
                    }
                }
            }
        }

        0
    }

    pub fn find_method_ref(
        constant_pool: &[ConstantPoolEntry],
        class_name: &str,
        method_name: &str,
        method_type: &str,
    ) -> u16 {
        for (i, entry) in constant_pool.iter().enumerate() {
            if let ConstantPoolEntry::MethodRef(class_index, name_and_type_index) = entry {
                if let ConstantPoolEntry::Utf8(name) =
                    &constant_pool[*name_and_type_index as usize - 1]
                {
                    if let ConstantPoolEntry::Utf8(descriptor) =
                        &constant_pool[*name_and_type_index as usize - 1]
                    {
                        if let ConstantPoolEntry::Utf8(class) =
                            &constant_pool[*class_index as usize - 1]
                        {
                            if name == method_name
                                && descriptor == method_type
                                && class == class_name
                            {
                                return i as u16 + 1;
                            }
                        }
                    }
                }
            }
        }

        0
    }

    pub fn find_field_ref(
        constant_pool: &[ConstantPoolEntry],
        class_name: &str,
        field_name: &str,
        field_type: &str,
    ) -> u16 {
        for (i, entry) in constant_pool.iter().enumerate() {
            if let ConstantPoolEntry::FieldRef(class_index, name_and_type_index) = entry {
                if let ConstantPoolEntry::Utf8(name) =
                    &constant_pool[*name_and_type_index as usize - 1]
                {
                    if let ConstantPoolEntry::Utf8(descriptor) =
                        &constant_pool[*name_and_type_index as usize - 1]
                    {
                        if let ConstantPoolEntry::Utf8(class) =
                            &constant_pool[*class_index as usize - 1]
                        {
                            if name == field_name && descriptor == field_type && class == class_name
                            {
                                return i as u16 + 1;
                            }
                        }
                    }
                }
            }
        }

        0
    }

    pub fn find_name_and_type(
        constant_pool: &[ConstantPoolEntry],
        name: &str,
        descriptor: &str,
    ) -> u16 {
        for (i, entry) in constant_pool.iter().enumerate() {
            if let ConstantPoolEntry::NameAndType(name_index, descriptor_index) = entry {
                if let ConstantPoolEntry::Utf8(name_string) =
                    &constant_pool[*name_index as usize - 1]
                {
                    if let ConstantPoolEntry::Utf8(descriptor_string) =
                        &constant_pool[*descriptor_index as usize - 1]
                    {
                        if name_string == name && descriptor_string == descriptor {
                            return i as u16 + 1;
                        }
                    }
                }
            }
        }

        0
    }

    pub fn find_utf8(constant_pool: &[ConstantPoolEntry], utf8: &str) -> u16 {
        for (i, entry) in constant_pool.iter().enumerate() {
            if let ConstantPoolEntry::Utf8(s) = entry {
                if s == utf8 {
                    return i as u16 + 1;
                }
            }
        }

        0
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
