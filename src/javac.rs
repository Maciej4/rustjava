use crate::java_class::ConstantPoolEntry;
use crate::jvm::{Class, Method};
use crate::{Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

/// Iterate over a tree's nodes and print them.
fn pretty_print_tree(root_node: &Node) {
    let mut stack = vec![*root_node];
    let mut indent_depth_list = vec![0];

    while let Some(node) = stack.pop() {
        let mut indent = indent_depth_list.pop().unwrap();
        println!(
            "{}{} [{}..{}]",
            "  ".repeat(indent),
            node.kind(),
            node.start_byte(),
            node.end_byte()
        );

        indent += 1;
        for i in (0..node.child_count()).rev() {
            stack.push(node.child(i).unwrap());
            indent_depth_list.push(indent);
        }
    }
}

fn pretty_print_node_full(node: &Node, source: &[u8]) {
    println!(
        "{} [{}..{}]:\n```\n{}\n```",
        node.kind(),
        node.start_byte(),
        node.end_byte(),
        node.utf8_text(source).unwrap()
    );
}

fn get_child_node_by_kind<'a>(node: &tree_sitter::Node<'a>, kind: &str) -> Node<'a> {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();

        if child.kind() == kind {
            return child;
        }
    }

    panic!("Could not find child node with name {}", kind);
}

fn get_child_node_by_kinds<'a>(node: &tree_sitter::Node<'a>, kinds: Vec<&str>) -> Node<'a> {
    if kinds.is_empty() {
        return *node;
    }

    return get_child_node_by_kinds(&get_child_node_by_kind(node, kinds[0]), kinds[1..].to_vec());
}

fn get_child_nodes_by_kind<'a>(node: &tree_sitter::Node<'a>, kind: &str) -> Vec<Node<'a>> {
    let mut nodes = vec![];

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();

        if child.kind() == kind {
            nodes.push(child);
        }
    }

    nodes
}

fn parse_expression(node: &Node, source: &[u8], super_locals: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = vec![];

    match node.kind() {
        "(" => {}
        "," => {}
        ")" => {}
        "decimal_integer_literal" => {
            let value = node.utf8_text(source).unwrap().parse::<i32>().unwrap();

            instructions.push(Instruction::Const(Primitive::Int(value)));
        }
        "identifier" => {
            let name = node.utf8_text(source).unwrap();

            if super_locals.contains(&name.to_string()) {
                instructions.push(Instruction::Load(
                    super_locals.iter().position(|r| r == name).unwrap(),
                    PrimitiveType::Int,
                ));
            } else {
                panic!("Could not find local variable {}", name);
            }
        }
        "binary_expression" => {
            let left = node.child(0).unwrap();
            let right = node.child(2).unwrap();

            instructions.append(&mut parse_expression(&left, source, super_locals));
            instructions.append(&mut parse_expression(&right, source, super_locals));

            match node.child(1).unwrap().kind() {
                "+" => instructions.push(Instruction::Add(PrimitiveType::Int)),
                "-" => instructions.push(Instruction::Sub(PrimitiveType::Int)),
                "*" => instructions.push(Instruction::Mul(PrimitiveType::Int)),
                "/" => instructions.push(Instruction::Div(PrimitiveType::Int)),
                "%" => instructions.push(Instruction::Rem(PrimitiveType::Int)),
                _ => panic!("Unknown operator {}", node.child(1).unwrap().kind()),
            }
        }
        "method_invocation" => {
            let method_name = get_child_node_by_kind(node, "identifier")
                .utf8_text(source)
                .unwrap();
            let arguments = get_child_node_by_kind(node, "argument_list");

            for i in 0..arguments.child_count() {
                let argument = arguments.child(i).unwrap();

                instructions.append(&mut parse_expression(&argument, source, super_locals));
            }

            // TODO: need to get the method from the class and add it to the constant pool to invoke it

            instructions.push(Instruction::InvokeVirtual(0));
        }
        "object_creation_expression" => {
            let class_name = get_child_node_by_kind(node, "type_identifier")
                .utf8_text(source)
                .unwrap();

            // TODO: get the class constant pool index

            instructions.push(Instruction::New(0));

            let arguments = get_child_node_by_kind(node, "argument_list");

            for i in 0..arguments.child_count() {
                let argument = arguments.child(i).unwrap();

                instructions.append(&mut parse_expression(&argument, source, super_locals));
            }

            // TODO: get the class init method constant pool index

            instructions.push(Instruction::InvokeSpecial(0));
        }
        "field_access" => {
            let class_or_object_name = node.child(0).unwrap().utf8_text(source).unwrap();
            let field_name = node.child(2).unwrap().utf8_text(source).unwrap();

            if super_locals.contains(&class_or_object_name.to_string()) {
                // The field is of a non-static type, as it's name is in the local variables

                // TODO: get the field constant pool index

                instructions.push(Instruction::GetField(0));
            } else {
                // The field is of a static type

                // TODO: get the static field constant pool index

                instructions.push(Instruction::GetStatic(0));
            }
        }
        _ => panic!("Unknown expression type {}", node.kind()),
    }

    instructions
}

fn parse_code_block(node: &Node, source: &[u8], super_locals: Vec<String>) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut locals = super_locals;

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();

        match child.kind() {
            "local_variable_declaration" => {
                let variable_declarator = get_child_node_by_kind(&child, "variable_declarator");

                let variable_name = get_child_node_by_kind(&variable_declarator, "identifier")
                    .utf8_text(source)
                    .unwrap()
                    .to_string();

                if variable_declarator.child_count() == 3 {
                    instructions.append(&mut parse_expression(
                        &variable_declarator.child(2).unwrap(),
                        source,
                        &locals,
                    ));

                    instructions.push(Instruction::Store(locals.len(), PrimitiveType::Int));
                }

                locals.push(variable_name);
            }
            "expression_statement" => {
                let expression = child.child(0).unwrap();

                match expression.kind() {
                    "assignment_expression" => {
                        if expression.child_count() == 3 {
                            match expression.child(1).unwrap().kind() {
                                "=" => {
                                    instructions.append(&mut parse_expression(
                                        &expression.child(2).unwrap(),
                                        source,
                                        &locals,
                                    ));
                                    instructions.push(Instruction::Store(
                                        locals
                                            .iter()
                                            .position(|r| {
                                                r == expression
                                                    .child(0)
                                                    .unwrap()
                                                    .utf8_text(source)
                                                    .unwrap()
                                            })
                                            .unwrap(),
                                        PrimitiveType::Int,
                                    ));
                                }
                                "+=" => {
                                    instructions.append(&mut parse_expression(
                                        &expression.child(0).unwrap(),
                                        source,
                                        &locals,
                                    ));
                                    instructions.append(&mut parse_expression(
                                        &expression.child(2).unwrap(),
                                        source,
                                        &locals,
                                    ));
                                    instructions.push(Instruction::Add(PrimitiveType::Int));
                                    instructions.push(Instruction::Store(
                                        locals
                                            .iter()
                                            .position(|r| {
                                                r == expression
                                                    .child(0)
                                                    .unwrap()
                                                    .utf8_text(source)
                                                    .unwrap()
                                            })
                                            .unwrap(),
                                        PrimitiveType::Int,
                                    ));
                                }
                                _ => {
                                    panic!(
                                        "Unknown assignment operator {}",
                                        expression.child(1).unwrap().utf8_text(source).unwrap()
                                    );
                                }
                            }
                        }
                    }
                    "method_invocation" => {
                        instructions.append(&mut parse_expression(
                            &get_child_node_by_kinds(
                                &expression,
                                vec!["argument_list", "identifier"],
                            ),
                            source,
                            &locals,
                        ));
                    }
                    _ => {}
                }
            }
            "return_statement" => {
                instructions.append(&mut parse_expression(
                    &child.child(1).unwrap(),
                    source,
                    &locals,
                ));
                // This is handled by the caller
                // instructions.push(Instruction::Return(PrimitiveType::Int));
            }
            _ => {}
        }
    }

    instructions
}

fn parse_method_args(node: &Node, source: &[u8]) -> (String, Vec<String>, Vec<PrimitiveType>) {
    let formal_parameters = get_child_node_by_kind(node, "formal_parameters");

    let mut parameters = String::new();
    let mut parameter_names = vec![];
    let mut parameter_types = vec![];

    parameters.push('(');

    let formal_params_to_parse = if formal_parameters.child_count() >= 3 {
        // This is a bit of a hack, but it works for now
        vec![
            get_child_nodes_by_kind(&formal_parameters, "formal_parameter")
                .into_iter()
                .map(|n| n.child(0).unwrap())
                .collect(),
            vec![node.child(1).unwrap()],
        ]
        .into_iter()
        .flatten()
        .collect()
    } else {
        vec![node.child(1).unwrap()]
    };

    for i in 0..formal_params_to_parse.len() {
        let parameter = formal_params_to_parse.get(i).unwrap();

        match parameter.kind() {
            "integral_type" => match parameter.child(0).unwrap().kind() {
                "byte" => {
                    parameters.push('B');
                    parameter_types.push(PrimitiveType::Byte);
                }
                "short" => {
                    parameters.push('S');
                    parameter_types.push(PrimitiveType::Short);
                }
                "int" => {
                    parameters.push('I');
                    parameter_types.push(PrimitiveType::Int);
                }
                "long" => {
                    parameters.push('J');
                    parameter_types.push(PrimitiveType::Long);
                }
                "char" => {
                    parameters.push('C');
                    parameter_types.push(PrimitiveType::Char);
                }
                _ => panic!(
                    "Unknown integral type {}",
                    parameter.child(0).unwrap().kind()
                ),
            },
            "floating_point_type" => match parameter.child(0).unwrap().kind() {
                "float" => {
                    parameters.push('F');
                    parameter_types.push(PrimitiveType::Float);
                }
                "double" => {
                    parameters.push('D');
                    parameter_types.push(PrimitiveType::Double);
                }
                _ => panic!(
                    "Unknown floating point type {}",
                    parameter.child(0).unwrap().kind()
                ),
            },
            "boolean_type" => {
                parameters.push('Z');
                // There is no boolean primitive type
                // parameter_types.push(PrimitiveType::Boolean);
            }
            "array_type" => {
                parameters.push('[');
                // TODO: actually parse the array type
                parameters.push_str("Ljava/lang/String;");
                parameter_types.push(PrimitiveType::Reference);
            }
            "void_type" => {
                parameters.push('V');
                parameter_types.push(PrimitiveType::Null);
            }
            _ => {
                panic!("Unknown parameter type {}", parameter.kind());
            }
        }

        if i < formal_params_to_parse.len() - 1 {
            parameter_names.push(
                parameter
                    .parent()
                    .unwrap()
                    .child(1)
                    .unwrap()
                    .utf8_text(source)
                    .unwrap()
                    .to_string(),
            );
        }
    }

    let c = parameters.pop().unwrap();

    parameters.push(')');
    parameters.push(c);

    (parameters, parameter_names, parameter_types)
}

fn parse_method(node: &Node, source: &[u8]) -> (String, Method) {
    let mut method_name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let (parameters, parameter_names, _parameter_types) = parse_method_args(node, source);

    method_name.push_str(parameters.as_str());

    let method_code_block = get_child_node_by_kind(node, "block");

    let mut instructions = parse_code_block(&method_code_block, source, parameter_names);

    let c = method_name.chars().last().unwrap();

    instructions.push(Instruction::Return(match c {
        'V' => PrimitiveType::Null,
        'B' => PrimitiveType::Byte,
        'S' => PrimitiveType::Short,
        'I' => PrimitiveType::Int,
        'J' => PrimitiveType::Long,
        'C' => PrimitiveType::Char,
        'F' => PrimitiveType::Float,
        'D' => PrimitiveType::Double,
        // Missing boolean ('Z') support
        _ => {
            panic!("unsupported return type character")
        }
    }));

    let method = Method { instructions };

    (method_name, method)
}

fn find_or_add_utf8(constant_pool: &mut Vec<ConstantPoolEntry>, value: &str) -> u16 {
    let index = ConstantPoolEntry::find_utf8(&constant_pool[..], value);

    if index == 0 {
        constant_pool.push(ConstantPoolEntry::Utf8(value.to_string()));
        return constant_pool.len() as u16 + 1;
    }

    index
}

fn find_or_add_class(constant_pool: &mut Vec<ConstantPoolEntry>, class_name: &str) -> u16 {
    let index = ConstantPoolEntry::find_class(&constant_pool[..], class_name);

    if index == 0 {
        let class_name_index = find_or_add_utf8(constant_pool, class_name);

        constant_pool.push(ConstantPoolEntry::Class(class_name_index))
    }

    index
}

fn find_or_add_name_and_type(
    constant_pool: &mut Vec<ConstantPoolEntry>,
    name: &str,
    descriptor: &str,
) -> u16 {
    let index = ConstantPoolEntry::find_name_and_type(&constant_pool[..], name, descriptor);

    if index == 0 {
        let name_index = find_or_add_utf8(constant_pool, name);
        let descriptor_index = find_or_add_utf8(constant_pool, descriptor);

        constant_pool.push(ConstantPoolEntry::NameAndType(name_index, descriptor_index));
    }

    index
}

fn find_or_add_method_ref(
    constant_pool: &mut Vec<ConstantPoolEntry>,
    class_name: &str,
    method_name: &str,
    method_type: &str,
) -> u16 {
    let index = ConstantPoolEntry::find_method_ref(
        &constant_pool[..],
        class_name,
        method_name,
        method_type,
    );

    if index == 0 {
        let class_index = find_or_add_class(constant_pool, class_name);
        let name_and_type_index =
            find_or_add_name_and_type(constant_pool, method_name, method_type);

        constant_pool.push(ConstantPoolEntry::MethodRef(
            class_index,
            name_and_type_index,
        ))
    }

    index
}

fn find_or_add_field_ref(
    constant_pool: &mut Vec<ConstantPoolEntry>,
    class_name: &str,
    field_name: &str,
    field_type: &str,
) -> u16 {
    let index =
        ConstantPoolEntry::find_field_ref(&constant_pool[..], class_name, field_name, field_type);

    if index == 0 {
        let class_index = find_or_add_class(constant_pool, class_name);
        let name_and_type_index = find_or_add_name_and_type(constant_pool, field_name, field_type);

        constant_pool.push(ConstantPoolEntry::FieldRef(
            class_index,
            name_and_type_index,
        ))
    }

    index
}

fn find_invocations(root_node: &Node, source: &[u8], class: &mut Class) {
    let mut stack = vec![*root_node];

    let mut invocations = vec![];

    let mut param_names_and_types = HashMap::new();

    while let Some(node) = stack.pop() {
        match node.kind() {
            "method_invocation" | "field_access" => {
                invocations.push(node);
            }
            "type_identifier" => {
                invocations.push(node);

                let mut name_node = node.parent().unwrap();

                while name_node.kind() == "array_type" {
                    name_node = name_node.parent().unwrap();
                }

                name_node = get_child_node_by_kind(&name_node, "identifier");

                let name = name_node.utf8_text(source).unwrap().to_string();

                let var_type = node.utf8_text(source).unwrap().to_string();

                param_names_and_types.insert(name, var_type);
            }
            _ => {}
        }

        for i in 0..node.child_count() {
            stack.push(node.child(i).unwrap());
        }
    }

    invocations.reverse();

    println!(
        "Found {} invocations: {:?}",
        invocations.len(),
        invocations
            .iter()
            .map(|n| n.utf8_text(source).unwrap())
            .collect::<Vec<&str>>()
    );

    // for invoke in &invocations {
    //     println!();
    //     pretty_print_node_full(invoke, source);
    // }

    println!(
        "Found {} param names and types: {:?}",
        param_names_and_types.len(),
        param_names_and_types
    );

    let mut constant_pool = vec![];

    // TODO: reserve the first n slots for the classes, methods, and fields

    for access_node in invocations {
        match access_node.kind() {
            "type_identifier" => {
                if access_node.parent().unwrap().kind() == "object_creation_expression" {
                    let class_name = access_node.utf8_text(source).unwrap().to_string();

                    let class_index = ConstantPoolEntry::find_class(&constant_pool, &class_name);

                    // TODO: this should add the class initialization method
                } else {
                    let class_name = access_node.utf8_text(source).unwrap().to_string();

                    let class_index = constant_pool.len() as u16 + 2;

                    constant_pool.push(ConstantPoolEntry::Class(class_index));

                    constant_pool.push(ConstantPoolEntry::Utf8(class_name));
                }
            }
            "field_access" => {
                let class_or_object_name = access_node
                    .child(0)
                    .unwrap()
                    .utf8_text(source)
                    .unwrap()
                    .to_string();

                let field_name = access_node
                    .child(2)
                    .unwrap()
                    .utf8_text(source)
                    .unwrap()
                    .to_string();

                let class_index =
                    ConstantPoolEntry::find_class(&constant_pool, &class_or_object_name);

                if class_index == 0 {
                    let object_type = param_names_and_types.get(&class_or_object_name).unwrap();

                    // let name_index = find_or_add_utf8(&mut constant_pool, &field_name);

                    println!(
                        "Could not find class {}, it is likely an instance of an object",
                        class_or_object_name
                    );
                    continue;
                }

                let name_and_type_index = ConstantPoolEntry::find_name_and_type(
                    &constant_pool,
                    &field_name,
                    param_names_and_types.get(&class_or_object_name).unwrap(),
                );

                if name_and_type_index == 0 {
                    let name_index = find_or_add_utf8(&mut constant_pool, &field_name);

                    let type_index = find_or_add_utf8(&mut constant_pool, &class_or_object_name);

                    constant_pool.push(ConstantPoolEntry::NameAndType(name_index, type_index));

                    continue;
                }
            }
            "method_invocation" => {
                // Implement this
            }
            _ => {}
        }
    }

    class.constant_pool = constant_pool;
}

fn parse_class(node: &Node, source: &[u8]) -> Class {
    let name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let mut class = Class {
        name,
        constant_pool: vec![],
        static_fields: HashMap::new(),
        methods: HashMap::new(),
    };

    find_invocations(node, source, &mut class);

    let class_body = get_child_node_by_kind(node, "class_body");

    let unparsed_methods = get_child_nodes_by_kind(&class_body, "method_declaration");

    let mut methods = HashMap::new();

    for method in unparsed_methods {
        let (method_name, method) = parse_method(&method, source);

        methods.insert(method_name, method);
    }

    class.methods = methods;

    class
}

pub fn parse_java_code_to_classes(code: String) -> Vec<Class> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(&code, None).unwrap();

    pretty_print_tree(&tree.root_node());
    println!();

    let mut classes = Vec::new();

    for i in 0..tree.root_node().child_count() {
        let node = tree.root_node().child(i).unwrap();

        if node.kind() == "class_declaration" {
            classes.push(parse_class(&node, code.as_bytes()));
        }
    }

    println!("classes: {:?}", classes);

    classes
}
