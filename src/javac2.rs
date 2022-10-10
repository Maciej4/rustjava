use crate::java_class::ConstantPoolEntry;
use crate::jvm::{Class, Method};
use crate::{Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use std::fmt::format;
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

fn formal_parameter_to_primitive_type(node: Node) -> Result<PrimitiveType, String> {
    let type_node = match node.child(0) {
        Some(node) => node,
        None => return Err(String::from("Formal parameter node with no type")),
    };

    match type_node.kind() {
        // TODO: support boolean type and properly implement array type
        "boolean_type" => Err(String::from("Boolean formal parameter not supported")),
        "array_type" => Ok(PrimitiveType::Reference),
        "void_type" => Ok(PrimitiveType::Null),
        "integral_type" | "floating_point_type" => {
            let type_node_deep = match type_node.child(0) {
                Some(node) => node,
                None => return Err(String::from("Formal parameter node with no type")),
            };

            match type_node_deep.kind() {
                "byte" => Ok(PrimitiveType::Byte),
                "short" => Ok(PrimitiveType::Short),
                "int" => Ok(PrimitiveType::Int),
                "long" => Ok(PrimitiveType::Long),
                "char" => Ok(PrimitiveType::Char),
                "float" => Ok(PrimitiveType::Float),
                "double" => Ok(PrimitiveType::Double),
                _ => Err(format!(
                    "Formal parameter with unknown integral or floating point type: {}",
                    type_node_deep.kind()
                )),
            }
        }
        _ => Err(format!(
            "Formal parameter with unknown type: {}",
            type_node.kind()
        )),
    }
}

fn parse_method_args(method_node: &Node) -> Result<Vec<PrimitiveType>, String> {
    let formal_params = match method_node.child_by_kind("formal_parameters") {
        Some(formal_params_node) => formal_params_node,
        None => return Err(String::from("Formal parameters not found")),
    };

    let mut param_types = vec![];

    for param in formal_params.children_by_kind("formal_parameter") {
        let param_type = match formal_parameter_to_primitive_type(param) {
            Ok(param_type) => param_type,
            Err(err) => return Err(err),
        };

        param_types.push(param_type);
    }

    Ok(param_types)
}

fn generate_method_list(root_node: &Node, source: &[u8]) -> Result<Vec<String>, String> {
    Ok(vec![])
}

pub fn parse_to_class(code: String) -> Result<Vec<Class>, String> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(&code, None).expect("Error parsing Java code");

    let root_node = tree.root_node();
    let mut cursor = root_node.walk();

    root_node.print_tree();

    let class = root_node.child_by_kind("class_declaration").unwrap();
    let class_body = class.child_by_kind("class_body").unwrap();
    let methods = class_body.children_by_kind("method_declaration");

    println!("Methods: {:?}", methods);

    let method0 = methods[0];

    println!("Method0: {:?}", method0);

    method0.print_tree();

    let x = parse_method_args(&method0);
    let y = parse_method_args(&methods[1]);

    println!("Method args 0: {:?}", x);
    println!("Method args 1: {:?}", y);

    Err(String::from("Not implemented"))
}
