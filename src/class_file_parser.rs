//! This module contains the code for the java class file parser.
use crate::java_class::*;
use crate::reader::Reader;

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
            "InnerClasses" => {
                let number_of_classes = r.g2();
                let mut classes = Vec::new();

                for _ in 0..number_of_classes {
                    classes.push(InnerClassElement {
                        inner_class_info_index: r.g2(),
                        outer_class_info_index: r.g2(),
                        inner_name_index: r.g2(),
                        inner_class_access_flags: r.g2(),
                    });
                }

                Attribute::InnerClasses(InnerClassesAttribute {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                })
            }
            "EnclosingMethod" => Attribute::EnclosingMethod(EnclosingMethodAttribute {
                attribute_name_index,
                attribute_length,
                class_index: r.g2(),
                method_index: r.g2(),
            }),
            "Synthetic" => Attribute::Synthetic(SyntheticAttribute {
                attribute_name_index,
                attribute_length,
            }),
            "Signature" => Attribute::Signature(SignatureAttribute {
                attribute_name_index,
                attribute_length,
                signature_index: r.g2(),
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
            "LocalVariableTable" => {
                let local_variable_table_length = r.g2();
                let mut local_variable_table = Vec::new();

                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableTableElement {
                        start_pc: r.g2(),
                        length: r.g2(),
                        name_index: r.g2(),
                        descriptor_index: r.g2(),
                        index: r.g2(),
                    });
                }

                Attribute::LocalVariableTable(LocalVariableTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    local_variable_table_length,
                    local_variable_table,
                })
            }
            "LocalVariableTypeTable" => {
                let local_variable_type_table_length = r.g2();
                let mut local_variable_type_table = Vec::new();

                for _ in 0..local_variable_type_table_length {
                    local_variable_type_table.push(LocalVariableTypeTableElement {
                        start_pc: r.g2(),
                        length: r.g2(),
                        name_index: r.g2(),
                        signature_index: r.g2(),
                        index: r.g2(),
                    });
                }

                Attribute::LocalVariableTypeTable(LocalVariableTypeTableAttribute {
                    attribute_name_index,
                    attribute_length,
                    local_variable_type_table_length,
                    local_variable_type_table,
                })
            }
            "Deprecated" => Attribute::Deprecated(DeprecatedAttribute {
                attribute_name_index,
                attribute_length,
            }),
            _ => panic!("{} is an unsupported attribute type", attribute_str_name),
        });

        if r.pos() != attribute_start_position + attribute_length as usize {
            println!("{:?} was not parsed completely", ct[attribute_name_index as usize]);
            // panic!("attribute was not parsed completely");
        }

        r.set_pos(attribute_start_position + attribute_length as usize);
    }

    attributes
}

pub fn parse_file(filename: &str) -> ClassFile {
    let mut r = Reader::new(filename);

    let magic = r.g4();
    let minor_version = r.g2();
    let major_version = r.g2();

    let constant_pool_count = r.g2();
    let constant_pool = parse_constant_pool(&mut r, constant_pool_count);

    let access_flags = ClassFlags::parse(r.g2());
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

    cf
}
