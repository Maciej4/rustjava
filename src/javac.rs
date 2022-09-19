use crate::jvm::Class;
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

pub fn parse_java_code_to_class(file_name: String, code: String) -> Class {
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

    Class {
        name: class_name.to_string(),
        constant_pool: vec![],
        static_fields: HashMap::new(),
        methods: HashMap::new(),
    }
}
