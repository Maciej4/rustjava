use crate::java_class::{ConstantPoolEntry, ConstantPoolExt};
use crate::jvm::{Class, Method};
use crate::{Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

trait NodeExt {
    fn child_by_kind(&self, kind: &str) -> Option<Node>;
    fn children_by_kind(&self, kind: &str) -> Vec<Node>;
    fn get_children(&self) -> Vec<Node>;
    fn count_node_kind_recursive(&self, kind: &str) -> usize;
    fn depth(&self) -> usize;
    fn print_tree(&self);
}

impl NodeExt for Node<'_> {
    fn child_by_kind(&self, kind: &str) -> Option<Node> {
        self.children(&mut self.walk())
            .find(|child| child.kind() == kind)
    }

    fn children_by_kind(&self, kind: &str) -> Vec<Node> {
        self.children(&mut self.walk())
            .filter(|child| child.kind() == kind)
            .collect()
    }

    fn get_children(&self) -> Vec<Node> {
        self.children(&mut self.walk()).collect()
    }

    fn count_node_kind_recursive(&self, kind: &str) -> usize {
        let mut count = 0;
        for child in self.get_children() {
            if child.kind() == kind {
                count += 1;
            }
            count += child.count_node_kind_recursive(kind);
        }
        count
    }

    fn depth(&self) -> usize {
        let mut depth = 0;
        let mut current_node = *self;
        while let Some(parent) = current_node.parent() {
            depth += 1;
            current_node = parent;
        }
        depth
    }

    fn print_tree(&self) {
        let mut stack = vec![*self];
        while let Some(node) = stack.pop() {
            println!(
                "{}{} [{}..{}]",
                "  ".repeat(node.depth()),
                node.kind(),
                node.start_byte(),
                node.end_byte()
            );

            for i in (0..node.child_count()).rev() {
                stack.push(node.child(i).unwrap());
            }
        }
    }
}

fn type_node_to_primitive_type(node: Node) -> Result<PrimitiveType, String> {
    match node.kind() {
        // TODO: support boolean type and properly implement array type
        "boolean_type" => Err(String::from("Boolean formal parameter not supported")),
        "array_type" => Ok(PrimitiveType::Reference),
        "void_type" => Ok(PrimitiveType::Null),
        "integral_type" | "floating_point_type" => {
            let node_deep =
                match node.child(0) {
                    Some(node) => node,
                    None => return Err(String::from(
                        "Integral or floating point type formal parameter is missing internal type",
                    )),
                };

            match node_deep.kind() {
                "byte" => Ok(PrimitiveType::Byte),
                "short" => Ok(PrimitiveType::Short),
                "int" => Ok(PrimitiveType::Int),
                "long" => Ok(PrimitiveType::Long),
                "char" => Ok(PrimitiveType::Char),
                "float" => Ok(PrimitiveType::Float),
                "double" => Ok(PrimitiveType::Double),
                _ => Err(format!(
                    "Formal parameter with unknown integral or floating point type: {}",
                    node_deep.kind()
                )),
            }
        }
        _ => Err(format!(
            "Formal parameter with unknown type: {}",
            node.kind()
        )),
    }
}

#[derive(Debug)]
struct MethodInfo {
    name: String,
    signature: String,
    variables: HashMap<String, PrimitiveType>,
    return_type: PrimitiveType,
}

fn parse_method_args(method_node: &Node, source: &[u8]) -> Result<MethodInfo, String> {
    let method_name = match method_node.child_by_kind("identifier") {
        Some(node) => match node.utf8_text(source) {
            Ok(text) => text.to_string(),
            Err(err) => return Err(format!("Failed to parse method identifier: {}", err)),
        },
        None => return Err(String::from("Method is missing identifier")),
    };

    let formal_params = match method_node.child_by_kind("formal_parameters") {
        Some(formal_params_node) => formal_params_node,
        None => return Err(String::from("Formal parameters not found")),
    };

    let mut param_names = vec![];
    let mut param_types = vec![];

    for param in formal_params.children_by_kind("formal_parameter") {
        let param_name = match param.child_by_kind("identifier") {
            Some(node) => match node.utf8_text(source) {
                Ok(text) => text.to_string(),
                Err(err) => {
                    return Err(format!(
                        "Failed to parse formal parameter identifier: {}",
                        err
                    ))
                }
            },
            None => return Err(String::from("Formal parameter is missing identifier")),
        };

        let param_type = match param.child(0) {
            Some(node) => match type_node_to_primitive_type(node) {
                Ok(parameter_type) => parameter_type,
                Err(err) => return Err(err),
            },
            None => return Err(String::from("Formal parameter is missing type")),
        };

        param_names.push(param_name);
        param_types.push(param_type);
    }

    let method_return_type = match method_node.child(1) {
        Some(method_return_type_node) => match type_node_to_primitive_type(method_return_type_node) {
            Ok(method_return_type) => method_return_type,
            Err(err) => return Err(err),
        },
        None => return Err(String::from("Method return type not found")),
    };

    let mut signature = format!(
        "{}({}){}",
        method_name,
        param_types
            .iter()
            .map(|t| t.as_letter())
            .collect::<String>(),
        method_return_type.as_letter()
    );

    // TODO: remove this when the standard library is implemented
    if signature == "main(R)V" {
        signature = "main([Ljava/lang/String;)V".to_string();
    }

    Ok(MethodInfo {
        name: method_name,
        signature,
        variables: param_names
            .into_iter()
            .zip(param_types.into_iter())
            .collect(),
        return_type: method_return_type,
    })
}

fn generate_method_list(class_node: &Node, source: &[u8]) -> Result<Vec<MethodInfo>, String> {
    let mut methods = vec![];

    for method_node in class_node.children_by_kind("method_declaration") {
        let method_info = match parse_method_args(&method_node, source) {
            Ok(method_info) => method_info,
            Err(err) => return Err(err),
        };

        methods.push(method_info);
    }

    Ok(methods)
}

pub fn parse_to_class(code: String) -> Result<Vec<Class>, String> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(&code, None).expect("Error parsing Java code");

    let root_node = tree.root_node();
    let source = code.as_bytes();

    root_node.print_tree();
    println!();

    let class = root_node.child_by_kind("class_declaration").unwrap();
    let class_body = class.child_by_kind("class_body").unwrap();

    println!("Methods: {:?}", generate_method_list(&class_body, source));

    Err(String::from("Not implemented"))
}
