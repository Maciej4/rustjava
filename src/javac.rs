use crate::java_class::{ConstantPoolEntry, ConstantPoolExt};
use crate::jvm::{Class, Method};
use crate::{Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

trait NodeExt {
    fn child_by_kind(&self, kind: &str) -> Option<Node>;
    fn children_by_kind(&self, kind: &str) -> Vec<Node>;
    fn get_children(&self) -> Vec<Node>;
    fn name_from_identifier(&self, source: &[u8]) -> Result<String, String>;
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

    fn name_from_identifier(&self, source: &[u8]) -> Result<String, String> {
        let child = match self.child_by_kind("identifier") {
            Some(child) => child,
            None => return Err(format!("Could not find identifier for {}", self.kind())),
        };

        match child.utf8_text(source) {
            Ok(text) => Ok(text.to_string()),
            Err(err) => Err(format!("Failed to parse name of {}: {}", child.kind(), err)),
        }
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

#[derive(Debug)]
struct SuperLocals {
    pub local_names: Vec<String>,
    pub local_types: Vec<PrimitiveType>,
    // TODO: add support for arrays
    pub reference_classes: HashMap<usize, usize>, // index of local, class name
}

impl SuperLocals {
    pub fn find_local(&self, name: &str) -> Option<usize> {
        self.local_names
            .iter()
            .position(|local_name| local_name == name)
    }

    pub fn get_local_type(&self, index: &usize) -> Result<PrimitiveType, String> {
        match self.local_types.get(*index) {
            Some(local_type) => Ok(local_type.clone()),
            None => Err(format!("Local variable with index {} not found", index)),
        }
    }

    pub fn add_local(&mut self, name: &str, local_type: PrimitiveType) {
        self.local_names.push(name.to_string());
        self.local_types.push(local_type);
    }
}

#[derive(Debug)]
struct MethodInfo {
    pub class_name: String,
    pub method_name: String,
    pub signature: String,
    pub variables: SuperLocals,
    pub return_type: PrimitiveType,
}

#[derive(Debug)]
struct FieldInfo {
    pub class_name: String,
    pub field_name: String,
    pub signature: String,
    pub field_type: PrimitiveType,
}

#[derive(Debug)]
struct ParserContext {
    pub classes: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<FieldInfo>,
}

impl ParserContext {
    pub fn find_method(&self, class_name: String, method_name: String) -> Option<&MethodInfo> {
        self.methods
            .iter()
            .find(|method| method.class_name == class_name && method.method_name == method_name)
    }

    pub fn find_field(&self, class_name: String, field_name: String) -> Option<&FieldInfo> {
        self.fields
            .iter()
            .find(|field| field.class_name == class_name && field.field_name == field_name)
    }
}

fn type_node_to_primitive_type(node: Node) -> Result<PrimitiveType, String> {
    match node.kind() {
        // TODO: Properly implement array type
        // L = fully qualified class name
        // [ = array
        "boolean_type" => Ok(PrimitiveType::Boolean),
        "array_type" => Ok(PrimitiveType::Reference),
        "type_identifier" => Ok(PrimitiveType::Reference),
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

fn parse_method_info(
    method_node: &Node,
    class_name: String,
    source: &[u8],
) -> Result<MethodInfo, String> {
    let method_name = method_node.name_from_identifier(source)?;

    let formal_params = match method_node.child_by_kind("formal_parameters") {
        Some(formal_params_node) => formal_params_node,
        None => return Err(String::from("Method is missing formal_parameters node")),
    };

    let mut param_names = vec![];
    let mut param_types = vec![];

    for param in formal_params.children_by_kind("formal_parameter") {
        let param_name = param.name_from_identifier(source)?;

        let param_type = match param.child(0) {
            Some(node) => type_node_to_primitive_type(node)?,
            None => return Err(String::from("Formal parameter is missing type")),
        };

        param_names.push(param_name);
        param_types.push(param_type);
    }

    let method_return_type = match method_node.child(1) {
        Some(method_return_type_node) => type_node_to_primitive_type(method_return_type_node)?,
        None => return Err(String::from("Method missing return type")),
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

    let variables = SuperLocals {
        local_names: param_names,
        local_types: param_types,
        reference_classes: HashMap::new(), // TODO: Implement this
    };

    Ok(MethodInfo {
        method_name,
        class_name,
        signature,
        variables,
        return_type: method_return_type,
    })
}

fn generate_method_list(class_node: &Node, source: &[u8]) -> Result<Vec<MethodInfo>, String> {
    let mut methods = vec![];

    let class_declaration_node = match class_node.parent() {
        Some(node) => node,
        None => return Err(String::from("Class body node has no parent")),
    };

    let class_name = class_declaration_node.name_from_identifier(source)?;

    for method_node in class_node.children_by_kind("method_declaration") {
        methods.push(parse_method_info(&method_node, class_name.clone(), source)?);
    }

    // TODO: Add constructor_declaration

    Ok(methods)
}

fn parse_expression(
    node: &Node,
    source: &[u8],
    parser_context: &ParserContext,
    super_locals: &SuperLocals,
    constant_pool: &mut Vec<ConstantPoolEntry>,
) -> Result<(Vec<Instruction>, PrimitiveType), String> {
    let mut instructions = vec![];
    let mut expression_type = PrimitiveType::Null;

    match node.kind() {
        "(" | "," | ")" => {}
        "decimal_integer_literal" => {
            let value = match node.utf8_text(source) {
                Ok(text) => match text.parse::<i32>() {
                    Ok(value) => value,
                    Err(err) => return Err(format!("Failed to parse integer literal: {}", err)),
                },
                Err(err) => {
                    return Err(format!("Failed to parse decimal integer literal: {}", err))
                }
            };

            instructions.push(Instruction::Const(Primitive::Int(value)));
        }
        "identifier" => {
            let name = match node.utf8_text(source) {
                Ok(text) => text.to_string(),
                Err(err) => return Err(format!("Failed to parse identifier: {}", err)),
            };

            match super_locals.find_local(&name) {
                Some(index) => {
                    let local_type = super_locals.get_local_type(&index)?;
                    instructions.push(Instruction::Load(index, local_type));
                }
                None => return Err(format!("Local variable {} not found", name)),
            }
        }
        "binary_expression" => {
            let left = match node.child(0) {
                Some(node) => node,
                None => return Err(String::from("Binary expression is missing left operand")),
            };

            let operator = match node.child(1) {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => return Err(format!("Failed to parse binary operator: {}", err)),
                },
                None => return Err(String::from("Binary expression is missing operator")),
            };

            let right = match node.child(2) {
                Some(node) => node,
                None => return Err(String::from("Binary expression is missing right operand")),
            };

            let (left_instructions, left_type) =
                parse_expression(&left, source, parser_context, super_locals, constant_pool)?;

            let (right_instructions, right_type) =
                parse_expression(&right, source, parser_context, super_locals, constant_pool)?;

            if !left_type.matches(&right_type) {
                // TODO: implement automatic type widening
                return Err(format!(
                    "Binary expression has mismatched types: {:?} and {:?}",
                    left_type, right_type
                ));
            }

            instructions.extend(left_instructions);
            instructions.extend(right_instructions);
            expression_type = left_type;

            instructions.push(match operator.as_str() {
                "+" => Instruction::Add(expression_type.clone()),
                "-" => Instruction::Sub(expression_type.clone()),
                "*" => Instruction::Mul(expression_type.clone()),
                "/" => Instruction::Div(expression_type.clone()),
                "%" => Instruction::Rem(expression_type.clone()),
                _ => return Err(format!("Unknown binary operator {}", operator)),
            })
        }
        "parenthesized_expression" => {
            let expression = match node.child(1) {
                Some(node) => node,
                None => {
                    return Err(String::from(
                        "Parenthesized expression is missing expression",
                    ))
                }
            };

            return parse_expression(
                &expression,
                source,
                parser_context,
                super_locals,
                constant_pool,
            );
        }
        "method_invocation" => {
            todo!()
        }
        "object_creation_expression" => {
            let class_name = match node.child_by_kind("type_identifier") {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => return Err(format!("Failed to parse class name: {}", err)),
                },
                None => {
                    return Err(String::from(
                        "Object creation expression is missing class name",
                    ))
                }
            };

            let class_index = match constant_pool.find_class(&class_name) {
                Some(index) => index,
                None => return Err(format!("Class {} not found in constant pool", class_name)),
            };

            instructions.push(Instruction::New(class_index as usize));
            instructions.push(Instruction::Dup);

            let arguments = match node.child_by_kind("argument_list") {
                Some(node) => node,
                None => {
                    return Err(String::from(
                        "Object creation expression is missing arguments",
                    ))
                }
            };

            let mut argument_types = vec![];

            for i in 0..arguments.child_count() {
                let argument = match arguments.child(i) {
                    Some(node) => node,
                    None => return Err(format!("Failed to parse argument {}", i)),
                };

                let (argument_instructions, argument_type) = parse_expression(
                    &argument,
                    source,
                    parser_context,
                    super_locals,
                    constant_pool,
                )?;

                instructions.extend(argument_instructions);
                argument_types.push(argument_type);
            }

            let arguments_count =
                arguments.child_count() - arguments.children_by_kind(",").len() - 2;

            let method_type = format!(
                "({})V",
                argument_types
                    .iter()
                    .map(|a| a.as_letter())
                    .collect::<String>()
            );

            let method_index = 0; // TODO: find method index
                                  // TODO: set expression_type to the return type of the method

            instructions.push(Instruction::InvokeSpecial(method_index));
        }
        "field_access" => {
            let class_or_object_name = match node.child(0) {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => {
                        return Err(format!("Failed to parse class or object name: {}", err))
                    }
                },
                None => return Err(String::from("Field access is missing class or object name")),
            };

            let field_name = match node.child(2) {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => return Err(format!("Failed to parse field name: {}", err)),
                },
                None => return Err(String::from("Field access is missing field name")),
            };

            if let Some(index) = super_locals.find_local(&class_or_object_name) {
                let field_index = 0; // TODO: find field index
                                     // TODO: set expression_type to the type of the field

                instructions.push(Instruction::Load(index, PrimitiveType::Reference));
                instructions.push(Instruction::GetField(field_index));
            } else {
                let field_index = 0; // TODO: find field index
                                     // TODO: set expression_type to the type of the field

                instructions.push(Instruction::GetStatic(field_index));
            }
        }
        _ => return Err(format!("Unknown expression type {}", node.kind())),
    }

    Ok((instructions, expression_type))
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
    let class_name = class.name_from_identifier(source)?;

    println!("Methods: {:?}", generate_method_list(&class_body, source));

    // TODO: generate method list for every class in project
    let parser_context = ParserContext {
        classes: vec![class_name],
        methods: generate_method_list(&class_body, source)?,
        fields: vec![],
    };

    Err(String::from("Not implemented"))
}
