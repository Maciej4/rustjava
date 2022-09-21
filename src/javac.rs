use crate::jvm::{Class, Method};
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Tree};

/// Iterate over a tree's nodes and print them.
fn pretty_print_tree(tree: &Tree) {
    let mut stack = vec![tree.root_node()];
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

fn parse_method(node: &Node, source: &[u8]) -> (String, Method) {
    let method_name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let method = Method {
        instructions: vec![],
    };

    (method_name, method)
}

fn parse_class(node: &Node, source: &[u8]) -> Class {
    let name = get_child_node_by_kind(node, "identifier")
        .utf8_text(source)
        .unwrap()
        .to_string();

    let class_body = get_child_node_by_kind(node, "class_body");

    let unparsed_methods = get_child_nodes_by_kind(&class_body, "method_declaration");

    let mut methods = HashMap::new();

    for method in unparsed_methods {
        let (method_name, method) = parse_method(&method, source);

        methods.insert(method_name, method);
    }

    println!("{:?}", methods);

    Class {
        name,
        constant_pool: vec![],
        static_fields: HashMap::new(),
        methods,
    }
}

pub fn parse_java_code_to_classes(code: String) {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(&code, None).unwrap();

    pretty_print_tree(&tree);

    println!();

    let class_name = tree
        .root_node()
        .child(0)
        .unwrap()
        .child(1)
        .unwrap()
        .utf8_text(code.as_bytes())
        .unwrap();

    println!("class name: {}", class_name);
    println!();

    let n = tree.root_node().child(0).unwrap();
    pretty_print_node_full(&n, code.as_bytes());

    // Get all of the classes in the tree
    let mut classes = Vec::new();

    for i in 0..tree.root_node().child_count() {
        let node = tree.root_node().child(i).unwrap();

        if node.kind() == "class_declaration" {
            classes.push(parse_class(&node, code.as_bytes()));
        }
    }

    println!("classes: {:?}", classes);
}
