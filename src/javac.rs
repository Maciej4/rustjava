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
        _ => {
            panic!("Unknown expression type {}", node.kind());
        }
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

fn parse_method_args(node: &Node, source: &[u8]) -> String {
    let formal_parameters = get_child_node_by_kind(node, "formal_parameters");

    let mut parameters = String::new();
    // let mut parameter_names = vec![];
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
    }

    // for i in 0..formal_params_to_parse.len() - 1 {
    //     let parameter = match formal_params_to_parse.get(i).unwrap().kind() {
    //         "integral_type" => formal_params_to_parse.get(i).unwrap().child(0).unwrap(),
    //         "floating_point_type" => formal_params_to_parse.get(i).unwrap().child(0).unwrap(),
    //         "boolean_type" => *formal_params_to_parse.get(i).unwrap(),
    //         "void_type" => *formal_params_to_parse.get(i).unwrap(),
    //         _ => {panic!("Unknown parameter type {}", formal_params_to_parse.get(i).unwrap().kind())}
    //     };
    //
    //     parameter_names.push(
    //         get_child_node_by_kind(&parameter, "identifier")
    //             .utf8_text(source)
    //             .unwrap()
    //             .to_string(),
    //     );
    // }

    let c = parameters.pop().unwrap();

    parameters.push(')');
    parameters.push(c);

    parameters
}

fn parse_method(node: &Node, source: &[u8]) -> (String, Method) {
    let mut method_name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    method_name.push_str(parse_method_args(node, source).as_str());

    let method_code_block = get_child_node_by_kind(node, "block");

    // TODO: passed parameters should be added to the locals
    let mut instructions = parse_code_block(&method_code_block, source, vec![]);

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

fn parse_class(node: &Node, source: &[u8]) -> Class {
    let name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let class_body = get_child_node_by_kind(node, "class_body");

    // TODO: generate constant pool

    let unparsed_methods = get_child_nodes_by_kind(&class_body, "method_declaration");

    let mut methods = HashMap::new();

    for method in unparsed_methods {
        let (method_name, method) = parse_method(&method, source);

        methods.insert(method_name, method);
    }

    Class {
        name,
        constant_pool: vec![],
        static_fields: HashMap::new(),
        methods,
    }
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
