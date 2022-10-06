use crate::java_class::ConstantPoolEntry;
use crate::jvm::{Class, Method};
use crate::{Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

// This takes a kind of weird approach to parsing the java code. First, it goes through the class
// file and finds all the invocations, static fields, object fields, and creates the constant pool
// entries for them. Then it generates the bytecode for the methods. This is a flawed approach as
// it doesn't take into account the types of the variables. It also isn't able to determine the
// signature of the methods. Combining these two steps into one could be a better approach as the
// constant pool generation could be done alongside the bytecode generation. Given that the
// constant pool generator keeps track of the variables and their types, it could be used to
// allow for better type checking. Method signatures could also be guessed. One issue with this
// is the parsing of multiple classes which use methods from each other. Perhaps an array of
// method (names and signatures) and field (names and types) could be generated first for each
// class and then passed to the bytecode generator?

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

fn parse_expression(
    node: &Node,
    source: &[u8],
    super_locals: &Vec<String>,
    constant_pool: &[ConstantPoolEntry],
) -> Vec<Instruction> {
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

            instructions.append(&mut parse_expression(
                &left,
                source,
                super_locals,
                constant_pool,
            ));
            instructions.append(&mut parse_expression(
                &right,
                source,
                super_locals,
                constant_pool,
            ));

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
            if node.child_count() < 3 {
                // Do stuff
                panic!("Method invocation not implemented for methods inside the same class");
            }

            // TODO: Implement method invocation for methods inside the same class

            // TODO: Handle methods called using field access (e.g. System.out.println)
            // For instance, a class which contains a static field populated by an object with methods

            let class_or_object_name = node.child(0).unwrap().utf8_text(source).unwrap();
            let method_name = node.child(2).unwrap().utf8_text(source).unwrap();
            let arguments = get_child_node_by_kind(node, "argument_list");

            // TODO: remove this or make it more generic
            if node.child(0).unwrap().kind() == "field_access" {
                instructions.append(&mut parse_expression(
                    &get_child_node_by_kinds(node, vec!["field_access"]),
                    source,
                    super_locals,
                    constant_pool,
                ));
            }

            for i in 0..arguments.child_count() {
                let argument = arguments.child(i).unwrap();

                instructions.append(&mut parse_expression(
                    &argument,
                    source,
                    super_locals,
                    constant_pool,
                ));
            }

            let arguments_count =
                arguments.child_count() - 2 - get_child_nodes_by_kind(&arguments, ",").len();

            // TODO: Handle methods with non-integer parameters and return values
            let method_type = format!("({})I", "I".repeat(arguments_count));

            // TODO: remove this or make it more generic
            if class_or_object_name == "System.out" && method_name == "println" {
                let index = ConstantPoolEntry::find_method_ref(
                    constant_pool,
                    "java/io/PrintStream",
                    "println",
                    "(I)V",
                );

                instructions.push(Instruction::InvokeVirtual(index as usize));

                return instructions;
            }

            if super_locals.contains(&class_or_object_name.to_string()) {
                let index = super_locals
                    .iter()
                    .position(|r| r == class_or_object_name)
                    .unwrap();

                instructions.push(Instruction::Load(index, PrimitiveType::Reference));

                let method_index = ConstantPoolEntry::find_method_ref(
                    constant_pool,
                    "Point", // TODO: Get class name
                    method_name,
                    "()I", // TODO: Get method type signature
                );

                instructions.push(Instruction::InvokeVirtual(method_index as usize));
            } else {
                let method_index = ConstantPoolEntry::find_method_ref(
                    constant_pool,
                    class_or_object_name,
                    method_name,
                    method_type.as_str(),
                );

                instructions.push(Instruction::InvokeVirtual(method_index as usize));
            }
        }
        "object_creation_expression" => {
            let class_name = get_child_node_by_kind(node, "type_identifier")
                .utf8_text(source)
                .unwrap();

            let class_index = ConstantPoolEntry::find_class(constant_pool, class_name);

            instructions.push(Instruction::New(class_index as usize));

            instructions.push(Instruction::Dup);

            let arguments = get_child_node_by_kind(node, "argument_list");

            for i in 0..arguments.child_count() {
                let argument = arguments.child(i).unwrap();

                instructions.append(&mut parse_expression(
                    &argument,
                    source,
                    super_locals,
                    constant_pool,
                ));
            }

            let arguments_count =
                arguments.child_count() - get_child_nodes_by_kind(&arguments, ",").len() - 2;

            let method_type = format!("({})V", "I".repeat(arguments_count));

            let method_index = ConstantPoolEntry::find_method_ref(
                constant_pool,
                class_name,
                "<init>",
                method_type.as_str(),
            );

            instructions.push(Instruction::InvokeSpecial(method_index as usize));
        }
        "field_access" => {
            let class_or_object_name = node.child(0).unwrap().utf8_text(source).unwrap();
            let field_name = node.child(2).unwrap().utf8_text(source).unwrap();

            if super_locals.contains(&class_or_object_name.to_string()) {
                // The field is of a non-static type, as it's name is in the local variables

                instructions.push(Instruction::Load(
                    super_locals
                        .iter()
                        .position(|r| r == class_or_object_name)
                        .unwrap(),
                    PrimitiveType::Reference,
                ));

                let field_index = ConstantPoolEntry::find_field_ref(
                    constant_pool,
                    "Point", // TODO: get the class name
                    field_name,
                    "I", // TODO: get the field type
                );

                instructions.push(Instruction::GetField(field_index as usize));
            } else {
                // The field is of a static type

                let field_index = ConstantPoolEntry::find_field_ref(
                    constant_pool,
                    class_or_object_name,
                    field_name,
                    "I", // TODO: get the field type
                );

                instructions.push(Instruction::GetStatic(field_index as usize));
            }
        }
        _ => panic!("Unknown expression type {}", node.kind()),
    }

    instructions
}

fn parse_code_block(
    node: &Node,
    source: &[u8],
    super_locals: Vec<String>,
    constant_pool: &[ConstantPoolEntry],
) -> Vec<Instruction> {
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
                        constant_pool,
                    ));

                    // instructions.push(Instruction::Store(locals.len(), PrimitiveType::Int));

                    // TODO: Figure out what the type of the variable is

                    instructions.push(Instruction::Store(locals.len(), PrimitiveType::Reference));
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
                                        constant_pool,
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
                                        constant_pool,
                                    ));
                                    instructions.append(&mut parse_expression(
                                        &expression.child(2).unwrap(),
                                        source,
                                        &locals,
                                        constant_pool,
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
                            &expression,
                            source,
                            &locals,
                            constant_pool,
                        ));

                        // instructions.append(&mut parse_expression(
                        //     &get_child_node_by_kinds(
                        //         &expression,
                        //         vec!["argument_list", "identifier"],
                        //     ),
                        //     source,
                        //     &locals,
                        //     constant_pool,
                        // ));
                    }
                    _ => {}
                }
            }
            "return_statement" => {
                instructions.append(&mut parse_expression(
                    &child.child(1).unwrap(),
                    source,
                    &locals,
                    constant_pool,
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

fn parse_method(
    node: &Node,
    source: &[u8],
    constant_pool: &[ConstantPoolEntry],
) -> (String, Method) {
    let mut method_name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let (parameters, parameter_names, _parameter_types) = parse_method_args(node, source);

    method_name.push_str(parameters.as_str());

    let method_code_block = get_child_node_by_kind(node, "block");

    let mut instructions =
        parse_code_block(&method_code_block, source, parameter_names, constant_pool);

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

        return constant_pool.len() as u16;
    }

    index
}

fn find_or_add_class(constant_pool: &mut Vec<ConstantPoolEntry>, class_name: &str) -> u16 {
    let index = ConstantPoolEntry::find_class(&constant_pool[..], class_name);

    if index == 0 {
        let class_name_index = find_or_add_utf8(constant_pool, class_name);

        constant_pool.push(ConstantPoolEntry::Class(class_name_index));

        return constant_pool.len() as u16;
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

        return constant_pool.len() as u16;
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
        ));

        return constant_pool.len() as u16;
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
        ));

        return constant_pool.len() as u16;
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

                if name_node.kind() == "object_creation_expression" {
                    continue;
                }

                // pretty_print_node_full(&name_node, source);

                if name_node.kind() == "local_variable_declaration" {
                    name_node = get_child_node_by_kinds(
                        &name_node,
                        ["variable_declarator", "identifier"].to_vec(),
                    );
                } else {
                    name_node = get_child_node_by_kind(&name_node, "identifier");
                }

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

    // println!(
    //     "Found {} invocations: {:?}",
    //     invocations.len(),
    //     invocations
    //         .iter()
    //         .map(|n| n.utf8_text(source).unwrap())
    //         .collect::<Vec<&str>>()
    // );

    let mut constant_pool = vec![];

    // TODO: reserve the first n slots for the classes, methods, and fields

    for access_node in invocations {
        // println!(
        //     "Name: {:width$} | Kind: {:width$} | Constant pool: {:?}",
        //     access_node.utf8_text(source).unwrap(),
        //     access_node.kind(),
        //     constant_pool,
        //     width = 25
        // );

        match access_node.kind() {
            "type_identifier" => {
                let class_name = access_node.utf8_text(source).unwrap();
                let _class_index = find_or_add_class(&mut constant_pool, class_name);

                if access_node.parent().unwrap().kind() == "object_creation_expression" {
                    let method_name = "<init>";

                    let argument_list_node =
                        get_child_node_by_kind(&access_node.parent().unwrap(), "argument_list");

                    let argument_count = argument_list_node.child_count()
                        - get_child_nodes_by_kind(&argument_list_node, ",").len()
                        - 2;

                    let method_type = format!("({})V", "I".repeat(argument_count as usize));

                    let _method_index = find_or_add_method_ref(
                        &mut constant_pool,
                        class_name,
                        method_name,
                        method_type.as_str(),
                    );
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

                if param_names_and_types.contains_key(&class_or_object_name) {
                    let field_type = param_names_and_types.get(&class_or_object_name).unwrap();

                    let _field_index = find_or_add_field_ref(
                        &mut constant_pool,
                        field_type,
                        &field_name,
                        "I", // TODO: get the type from the field
                    );
                } else {
                    let _field_index = find_or_add_field_ref(
                        &mut constant_pool,
                        &class_or_object_name,
                        &field_name,
                        "I",
                    );
                }
            }
            "method_invocation" => {
                // TODO: actually find the method signature

                if access_node.child_count() < 3 {
                    let method_name = access_node
                        .child(0)
                        .unwrap()
                        .utf8_text(source)
                        .unwrap()
                        .to_string();

                    let method_args = get_child_node_by_kind(&access_node, "argument_list");
                    let method_arg_count = method_args.child_count()
                        - get_child_nodes_by_kind(&method_args, ",").len()
                        - 2;
                    let method_type = format!("({})I", "I".repeat(method_arg_count));

                    let _method_index = find_or_add_method_ref(
                        &mut constant_pool,
                        &class.name,
                        &method_name,
                        &method_type,
                    );

                    continue;
                }

                let class_or_object_name = access_node
                    .child(0)
                    .unwrap()
                    .utf8_text(source)
                    .unwrap()
                    .to_string();

                let method_name = access_node
                    .child(2)
                    .unwrap()
                    .utf8_text(source)
                    .unwrap()
                    .to_string();

                let method_args = get_child_node_by_kind(&access_node, "argument_list");
                let method_arg_count = method_args.child_count()
                    - get_child_nodes_by_kind(&method_args, ",").len()
                    - 2;
                let method_type = format!("({})I", "I".repeat(method_arg_count));

                // TODO: remove this
                if method_name == "println" {
                    let _method_index = find_or_add_method_ref(
                        &mut constant_pool,
                        "java/io/PrintStream",
                        &method_name,
                        "(I)V",
                    );

                    continue;
                }

                if param_names_and_types.contains_key(&class_or_object_name) {
                    let class_type = param_names_and_types.get(&class_or_object_name).unwrap();

                    let _method_index = find_or_add_method_ref(
                        &mut constant_pool,
                        class_type,
                        &method_name,
                        &method_type,
                    );
                } else {
                    let _method_index = find_or_add_method_ref(
                        &mut constant_pool,
                        &class_or_object_name,
                        &method_name,
                        &method_type,
                    );
                }
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
        let (method_name, method) = parse_method(&method, source, &class.constant_pool);

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

    // pretty_print_tree(&tree.root_node());
    // println!();

    let mut classes = Vec::new();

    for i in 0..tree.root_node().child_count() {
        let node = tree.root_node().child(i).unwrap();

        if node.kind() == "class_declaration" {
            classes.push(parse_class(&node, code.as_bytes()));
        }
    }

    classes
}
