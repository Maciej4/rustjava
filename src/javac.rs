use crate::java_class::{ConstantPoolEntry, ConstantPoolExt};
use crate::jvm::{Class, Method};
use crate::{Comparison, Instruction, Primitive, PrimitiveType};
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

trait NodeExt {
    fn child_by_kind(&self, kind: &str) -> Result<Node, String>;
    fn children_by_kind(&self, kind: &str) -> Vec<Node>;
    fn get_children(&self) -> Vec<Node>;
    fn name_from_identifier(&self, source: &[u8]) -> Result<String, String>;
    fn count_node_kind_recursive(&self, kind: &str) -> usize;
    fn depth(&self) -> usize;
    fn print_tree(&self);
}

impl NodeExt for Node<'_> {
    fn child_by_kind(&self, kind: &str) -> Result<Node, String> {
        match self
            .children(&mut self.walk())
            .find(|child| child.kind() == kind)
        {
            Some(node) => Ok(node),
            None => Err(format!(
                "{} has no children with kind {}",
                self.kind(),
                kind
            )),
        }
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
        match self.child_by_kind("identifier")?.utf8_text(source) {
            Ok(text) => Ok(text.to_string()),
            Err(err) => Err(format!("Failed to parse name of {}: {}", self.kind(), err)),
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

#[derive(Debug, Clone)]
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
struct FieldInfo {
    pub name: String,
    // TODO: add flags
    pub signature: String,
    pub descriptor: PrimitiveType,
    // TODO: add support for arrays and objects
}

#[derive(Debug)]
struct MethodInfo {
    pub name: String,
    // TODO: add flags
    pub signature: String,
    pub variables: SuperLocals,
    pub return_type: PrimitiveType,
}

#[derive(Debug)]
struct ClassInfo {
    pub name: String,
    pub super_class: String,
    // TODO: add flags
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

#[derive(Debug)]
struct ParserContext {
    pub classes: Vec<ClassInfo>,
}

impl ParserContext {
    pub fn find_class(&self, class_name: &str) -> Result<&ClassInfo, String> {
        match self.classes.iter().find(|class| class.name.eq(class_name)) {
            Some(class) => Ok(class),
            None => Err(format!("Class {} not found", class_name)),
        }
    }

    pub fn find_field(&self, class_name: &str, field_name: &String) -> Result<&FieldInfo, String> {
        let class = self.find_class(class_name)?;
        match class.fields.iter().find(|field| field.name.eq(field_name)) {
            Some(field) => Ok(field),
            None => Err(format!(
                "Field {} not found in class {}",
                field_name, class_name
            )),
        }
    }

    pub fn find_method(
        &self,
        class_name: &str,
        method_signature: &String,
    ) -> Result<&MethodInfo, String> {
        match self
            .find_class(class_name)?
            .methods
            .iter()
            .find(|method| method.signature.eq(method_signature))
        {
            Some(method) => Ok(method),
            None => Err(format!(
                "Method {} not found in class {}",
                method_signature, class_name
            )),
        }
    }

    pub fn find_method_by_params(
        &self,
        class_name: &str,
        method_parameters: &String,
    ) -> Result<&MethodInfo, String> {
        match self
            .find_class(class_name)?
            .methods
            .iter()
            .find(|method| method.signature.starts_with(method_parameters))
        {
            Some(method) => Ok(method),
            None => Err(format!(
                "Method with parameters {} not found in class {}",
                method_parameters, class_name
            )),
        }
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
    class_name: &String,
    source: &[u8],
) -> Result<MethodInfo, String> {
    let formal_params = method_node.child_by_kind("formal_parameters")?;

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

    let method_name_or_constructor = method_node.name_from_identifier(source)?;

    let method_name = if method_name_or_constructor.eq(class_name) {
        String::from("<init>")
    } else {
        method_name_or_constructor
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
        name: method_name,
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
        methods.push(parse_method_info(&method_node, &class_name, source)?);
    }

    // TODO: Add constructor_declaration

    Ok(methods)
}

fn parse_expression(
    node: &Node,
    source: &[u8],
    current_class: &String,
    parser_context: &ParserContext,
    super_locals: &SuperLocals,
    constant_pool: &mut Vec<ConstantPoolEntry>,
) -> Result<(Vec<Instruction>, PrimitiveType), String> {
    let mut instructions = vec![];
    let mut expression_type = PrimitiveType::Null;

    println!("Parsing expression: {}", node.kind());

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

            expression_type = PrimitiveType::Int;
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
                    instructions.push(Instruction::Load(index, local_type.clone()));
                    expression_type = local_type;
                }
                None => return Err(format!("Local variable {} not found", name)),
            }
        }
        "assignment_expression" | "variable_declarator" => {
            let variable_index =
                match super_locals.find_local(node.name_from_identifier(source)?.as_str()) {
                    Some(index) => index,
                    None => {
                        return Err(format!(
                            "Local variable {} not found",
                            node.name_from_identifier(source)?
                        ))
                    }
                };
            let variable_type = super_locals.get_local_type(&variable_index)?;

            let expression_node = match node.child(2) {
                Some(node) => node,
                None => return Err(String::from("Assignment expression is missing expression")),
            };

            let (expression_instructions, expr_type) = parse_expression(
                &expression_node,
                source,
                current_class,
                parser_context,
                super_locals,
                constant_pool,
            )?;

            instructions.extend(expression_instructions);
            if !variable_type.matches(&expr_type) {
                return Err(format!(
                    "Assignment expression type mismatch: {:?} != {:?}",
                    variable_type, expr_type
                ));
            }
            expression_type = variable_type.clone();

            let operator = match node.child(1) {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text,
                    Err(err) => {
                        return Err(format!("Failed to parse assignment operator: {}", err))
                    }
                },
                None => return Err(String::from("Assignment expression is missing operator")),
            };

            if operator.len() == 2 {
                instructions.push(Instruction::Load(variable_index, variable_type.clone()));
                let variable_type_clone = variable_type.clone();

                instructions.push(match operator {
                    "+=" => Instruction::Add(variable_type_clone),
                    "-=" => Instruction::Sub(variable_type_clone),
                    "*=" => Instruction::Mul(variable_type_clone),
                    "/=" => Instruction::Div(variable_type_clone),
                    "%=" => Instruction::Rem(variable_type_clone),
                    _ => return Err(format!("Unknown assignment operator: {}", operator)),
                });
            }

            instructions.push(Instruction::Store(variable_index, variable_type));
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

            let (left_instructions, left_type) = parse_expression(
                &left,
                source,
                current_class,
                parser_context,
                super_locals,
                constant_pool,
            )?;

            let (right_instructions, right_type) = parse_expression(
                &right,
                source,
                current_class,
                parser_context,
                super_locals,
                constant_pool,
            )?;

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
                current_class,
                parser_context,
                super_locals,
                constant_pool,
            );
        }
        "object_creation_expression" => {
            let class_name = match node.child_by_kind("type_identifier")?.utf8_text(source) {
                Ok(text) => text.to_string(),
                Err(err) => return Err(format!("Failed to parse class name: {}", err)),
            };

            parser_context.find_class(&class_name)?;
            let class_index = constant_pool.find_or_add_class(&class_name);

            instructions.push(Instruction::New(class_index as usize));
            instructions.push(Instruction::Dup);

            let arguments_node = node.child_by_kind("argument_list")?;
            let mut argument_types = vec![];

            for i in 1..(arguments_node.child_count() - 1) {
                let argument = match arguments_node.child(i) {
                    Some(node) => node,
                    None => return Err(format!("Could not find argument_list child {}", i)),
                };

                let (argument_instructions, argument_type) = parse_expression(
                    &argument,
                    source,
                    current_class,
                    parser_context,
                    super_locals,
                    constant_pool,
                )?;

                if argument_type.matches(&PrimitiveType::Null) {
                    continue;
                }

                instructions.extend(argument_instructions);
                argument_types.push(argument_type);
            }

            let constructor_descriptor = format!(
                "({})V",
                argument_types
                    .iter()
                    .map(|a| a.as_letter())
                    .collect::<String>()
            );

            let constructor_signature = format!("<init>{}", constructor_descriptor);
            parser_context.find_method(&class_name, &constructor_signature)?;

            let method_index = constant_pool.find_or_add_method_ref(
                &class_name,
                "<init>",
                &constructor_descriptor,
            );

            expression_type = PrimitiveType::Null;
            instructions.push(Instruction::InvokeSpecial(method_index));
        }
        "method_invocation" => {
            let arguments_node = node.child_by_kind("argument_list")?;
            let mut argument_types = vec![];

            for i in 1..(arguments_node.child_count() - 1) {
                let argument = match arguments_node.child(i) {
                    Some(node) => node,
                    None => return Err(format!("Could not find argument_list child {}", i)),
                };

                let (argument_instructions, argument_type) = parse_expression(
                    &argument,
                    source,
                    current_class,
                    parser_context,
                    super_locals,
                    constant_pool,
                )?;

                if argument_type.matches(&PrimitiveType::Null) {
                    continue;
                }

                instructions.extend(argument_instructions);
                argument_types.push(argument_type);
            }

            let method_params = format!(
                "({})",
                argument_types
                    .iter()
                    .map(|a| a.as_letter())
                    .collect::<String>()
            );

            // This is the case where the method is inside the same class
            if node.child_count() < 3 {
                let method_name = match node.child_by_kind("identifier")?.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => return Err(format!("Failed to parse method name: {}", err)),
                };

                let method_partial_signature = format!("{}{}", method_name, method_params);
                let method = parser_context
                    .find_method_by_params(current_class, &method_partial_signature)?;

                let method_descriptor =
                    format!("{}{}", method_params, method.return_type.as_letter());

                let method_index = constant_pool.find_or_add_method_ref(
                    current_class,
                    &method_name,
                    &method_descriptor,
                );

                expression_type = method.return_type.clone();
                // TODO: handle non-static methods for methods inside the same class
                instructions.push(Instruction::InvokeStatic(method_index));
            } else {
                // TODO: these two are the same as for field access and should be abstracted
                let class_or_object_name = match node.child(0) {
                    Some(node) => match node.utf8_text(source) {
                        Ok(text) => text.to_string(),
                        Err(err) => {
                            return Err(format!("Failed to parse class or object name: {}", err));
                        }
                    },
                    None => {
                        return Err(String::from(
                            "Method invocation is missing class or object name",
                        ));
                    }
                };

                let method_name = match node.child(2) {
                    Some(node) => match node.utf8_text(source) {
                        Ok(text) => text.to_string(),
                        Err(err) => return Err(format!("Failed to parse method name: {}", err)),
                    },
                    None => return Err(String::from("Method invocation is missing method name")),
                };

                if method_name.eq("println") {
                    let method_index = constant_pool.find_or_add_method_ref(
                        "java/io/PrintStream",
                        "println",
                        "(I)V",
                    );

                    instructions.push(Instruction::InvokeVirtual(method_index));
                    expression_type = PrimitiveType::Null;

                    return Ok((instructions, expression_type));
                }

                let method_partial_signature = format!("{}{}", method_name, method_params);

                if let Some(index) = super_locals.find_local(&class_or_object_name) {
                    // Dynamic method invocation
                    let class_name = match super_locals.reference_classes.get(&index) {
                        Some(class_name) => match constant_pool.class_parser(class_name) {
                            Some(name) => name,
                            None => {
                                return Err(format!(
                                    "Invoked dynamic method on class not in constant pool: {}",
                                    class_or_object_name
                                ))
                            }
                        },
                        None => {
                            return Err(format!(
                                "Dynamic method invocation on non-object: {}",
                                class_or_object_name
                            ));
                        }
                    };

                    let method = parser_context
                        .find_method_by_params(&class_name, &method_partial_signature)?;

                    let method_descriptor =
                        format!("{}{}", method_params, method.return_type.as_letter());

                    let method_index = constant_pool.find_or_add_method_ref(
                        &class_or_object_name,
                        &method_name,
                        &method_descriptor,
                    );

                    expression_type = method.return_type.clone();
                    instructions.push(Instruction::Load(index, PrimitiveType::Reference));
                    instructions.push(Instruction::InvokeVirtual(method_index));
                } else {
                    // Static method invocation
                    let method = parser_context
                        .find_method_by_params(&class_or_object_name, &method_partial_signature)?;

                    let method_descriptor =
                        format!("{}{}", method_params, method.return_type.as_letter());

                    let method_index = constant_pool.find_or_add_method_ref(
                        &class_or_object_name,
                        &method_name,
                        &method_descriptor,
                    );

                    expression_type = method.return_type.clone();
                    instructions.push(Instruction::InvokeStatic(method_index));
                }
            }
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

            println!("class_or_object_name: {}", class_or_object_name);

            let field_name = match node.child(2) {
                Some(node) => match node.utf8_text(source) {
                    Ok(text) => text.to_string(),
                    Err(err) => return Err(format!("Failed to parse field name: {}", err)),
                },
                None => return Err(String::from("Field access is missing field name")),
            };

            if let Some(index) = super_locals.find_local(&class_or_object_name) {
                let class_name = match super_locals.reference_classes.get(&index) {
                    Some(class_name) => match constant_pool.class_parser(class_name) {
                        Some(name) => name,
                        None => {
                            return Err(format!("{} is missing from the constant pool", class_name))
                        }
                    },
                    None => {
                        return Err(format!(
                            "Local variable {} is not a valid class reference",
                            class_or_object_name
                        ))
                    }
                };

                let field = parser_context.find_field(&class_name, &field_name)?;
                let field_index = constant_pool.find_or_add_field_ref(
                    &class_name,
                    &field_name,
                    field.signature.as_str(),
                );

                expression_type = field.descriptor.clone();
                instructions.push(Instruction::Load(index, PrimitiveType::Reference));
                instructions.push(Instruction::GetField(field_index));
            } else {
                let field = parser_context.find_field(&class_or_object_name, &field_name)?;

                let field_index = constant_pool.find_or_add_field_ref(
                    &class_or_object_name,
                    &field_name,
                    field.signature.as_str(),
                );

                expression_type = field.descriptor.clone();
                instructions.push(Instruction::GetStatic(field_index));
            }
        }
        _ => return Err(format!("Unknown expression type {}", node.kind())),
    }

    Ok((instructions, expression_type))
}

#[derive(Debug)]
struct ExpressionInfo {
    pub comparison: Comparison,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct ConnectiveInfo {
    pub comparisons: Vec<BlockType>,
}

#[derive(Debug)]
enum BlockType {
    And(ConnectiveInfo),
    Or(ConnectiveInfo),
    Parenthesis(ConnectiveInfo),
    Expression(ExpressionInfo),
}

impl BlockType {
    /// Pretty print the block type and its children
    pub fn pretty_print_tree(&self, depth: usize) {
        let mut indent = "  ".repeat(depth);

        match self {
            BlockType::And(info) => {
                println!("{}AND", indent);
                for comparison in &info.comparisons {
                    comparison.pretty_print_tree(depth + 1);
                }
            }
            BlockType::Or(info) => {
                println!("{}OR", indent);
                for comparison in &info.comparisons {
                    comparison.pretty_print_tree(depth + 1);
                }
            }
            BlockType::Parenthesis(info) => {
                println!("{}PARENTHESIS", indent);
                for comparison in &info.comparisons {
                    comparison.pretty_print_tree(depth + 1);
                }
            }
            BlockType::Expression(info) => {
                println!("{}COMPARISON", indent);
                for instruction in &info.instructions {
                    println!("{}  {:?}", indent, instruction);
                }
                println!("{}  {:?}", indent, info.comparison);
            }
        }
    }

    /// Flatten the connective block into a single connective
    /// i.e. And(And(Expr, Expr), Expr) -> And(Expr, Expr, Expr)
    /// or Or(Or(Expr, Expr), Expr) -> Or(Expr, Expr, Expr)
    /// This should also strip unnecessary parenthesis.
    pub fn flatten(&self) -> BlockType {
        match self {
            BlockType::And(info) => {
                let mut comparisons = Vec::new();
                for comparison in &info.comparisons {
                    match comparison.flatten() {
                        BlockType::And(info) => {
                            for comparison in info.comparisons {
                                comparisons.push(comparison);
                            }
                        }
                        comparison => comparisons.push(comparison),
                    }
                }
                BlockType::And(ConnectiveInfo { comparisons })
            }
            BlockType::Or(info) => {
                let mut comparisons = Vec::new();
                for comparison in &info.comparisons {
                    match comparison.flatten() {
                        BlockType::Or(info) => {
                            for comparison in info.comparisons {
                                comparisons.push(comparison);
                            }
                        }
                        comparison => comparisons.push(comparison),
                    }
                }
                BlockType::Or(ConnectiveInfo { comparisons })
            }
            BlockType::Parenthesis(info) => {
                let mut comparisons = Vec::new();
                for comparison in &info.comparisons {
                    match comparison.flatten() {
                        BlockType::Parenthesis(info) => {
                            for comparison in info.comparisons {
                                comparisons.push(comparison);
                            }
                        }
                        comparison => comparisons.push(comparison),
                    }
                }
                if comparisons.len() == 1 {
                    comparisons.remove(0)
                } else {
                    BlockType::Parenthesis(ConnectiveInfo { comparisons })
                }
                // BlockType::Parenthesis(ConnectiveInfo { comparisons })
            }
            BlockType::Expression(info) => BlockType::Expression(ExpressionInfo {
                comparison: info.comparison.clone(),
                instructions: info.instructions.clone(),
            }),
        }
    }
}

fn partial_parse_if(
    node: &Node,
    source: &[u8],
    current_class: &String,
    parser_context: &ParserContext,
    super_locals: &SuperLocals,
    constant_pool: &mut Vec<ConstantPoolEntry>,
    depth: u32,
) -> Result<BlockType, String> {
    let mut instructions = Vec::new();

    println!("partial_parse_if: {}", node.kind());

    if node.kind() == "parenthesized_expression" {
        let child = match node.child(1) {
            Some(node) => node,
            None => return Err(String::from("Parenthesized expression is missing child")),
        };

        let block = partial_parse_if(
            &child,
            source,
            current_class,
            parser_context,
            super_locals,
            constant_pool,
            depth + 1,
        )?;

        return Ok(BlockType::Parenthesis(ConnectiveInfo {
            comparisons: vec![block],
        }));
    }

    if node.kind() == "binary_expression" {
        let left = match node.child(0) {
            Some(node) => node,
            None => return Err(String::from("Binary expression is missing left side")),
        };

        let right = match node.child(2) {
            Some(node) => node,
            None => return Err(String::from("Binary expression is missing right side")),
        };

        let operator = match node.child(1) {
            Some(node) => match node.utf8_text(source) {
                Ok(text) => text,
                Err(err) => return Err(format!("Failed to parse binary operator: {}", err)),
            },
            None => return Err(String::from("Binary expression is missing operator")),
        };

        if operator.eq("&&") || operator.eq("||") {
            let left_block = partial_parse_if(
                &left,
                source,
                current_class,
                parser_context,
                super_locals,
                constant_pool,
                depth,
            )?;

            let right_block = partial_parse_if(
                &right,
                source,
                current_class,
                parser_context,
                super_locals,
                constant_pool,
                depth,
            )?;

            return Ok(match operator {
                "&&" => BlockType::And(ConnectiveInfo {
                    comparisons: vec![left_block, right_block],
                }),
                "||" => BlockType::Or(ConnectiveInfo {
                    comparisons: vec![left_block, right_block],
                }),
                _ => return Err(format!("Unknown operator {}", operator)),
            });
        }

        let (left_instructions, left_type) = parse_expression(
            &left,
            source,
            current_class,
            parser_context,
            super_locals,
            constant_pool,
        )?;

        let (right_instructions, right_type) = parse_expression(
            &right,
            source,
            current_class,
            parser_context,
            super_locals,
            constant_pool,
        )?;

        instructions.extend(left_instructions);
        instructions.extend(right_instructions);

        let comparison = match operator {
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            ">" => Comparison::GreaterThan,
            ">=" => Comparison::GreaterThanOrEqual,
            "<" => Comparison::LessThan,
            "<=" => Comparison::LessThanOrEqual,
            _ => return Err(format!("Unknown comparison operator {}", operator)),
        };

        return Ok(BlockType::Expression(ExpressionInfo {
            comparison,
            instructions,
        }));
    }

    todo!()
}

/// Notes on parsing if statements:
// a && b && c
// not(a) -> end; not(b) -> end; not(c) -> end;

// a || b || c
// a -> start; b -> start; not(c) -> end;

// (a || b || c) && (d || e || f)
// a -> next block; b -> next block; not(c) -> end;   &&   d -> start; e -> start; not(f) -> end;

// (a && b && c) || (d && e && f)
// not(a) -> next block; not(b) -> next block; c -> start;   ||   not(d) -> start; not(e) -> start; not(f) -> end;

// And statements are parsed first, then or statements

fn parse_if(
    node: &Node,
    source: &[u8],
    current_class: &String,
    parser_context: &ParserContext,
    super_locals: &SuperLocals,
    constant_pool: &mut Vec<ConstantPoolEntry>,
    depth: u32,
) -> Result<Vec<Instruction>, String> {
    let child = match node.child_by_kind("parenthesized_expression")?.child(1) {
        Some(node) => node,
        None => return Err(String::from("If statement doesn't have a condition")),
    };

    child.print_tree();

    let expression_tree = partial_parse_if(
        &child,
        source,
        current_class,
        parser_context,
        super_locals,
        constant_pool,
        depth,
    )?
    .flatten();

    expression_tree.pretty_print_tree(0);

    Err(String::from("Finished parsing if"))
}

fn parse_code_block(
    node: &Node,
    source: &[u8],
    current_class: &String,
    parser_context: &ParserContext,
    super_locals: &SuperLocals,
    constant_pool: &mut Vec<ConstantPoolEntry>,
) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    let mut locals = (*super_locals).clone();

    for child in node.get_children() {
        println!("Parsing child: {}", child.kind());

        match child.kind() {
            "local_variable_declaration" => {
                let variable_declarator = child.child_by_kind("variable_declarator")?;
                let variable_name = variable_declarator.name_from_identifier(source)?;
                let type_node = match child.child(0) {
                    Some(node) => node,
                    None => return Err(String::from("Local variable declaration is missing type")),
                };
                let variable_type = type_node_to_primitive_type(type_node)?;
                locals.add_local(&variable_name, variable_type.clone());

                if variable_declarator.child_count() == 3 {
                    let (expression_instructions, expression_type) = parse_expression(
                        &variable_declarator,
                        source,
                        current_class,
                        parser_context,
                        &locals,
                        constant_pool,
                    )?;

                    instructions.extend(expression_instructions);

                    if !variable_type.matches(&expression_type) {
                        return Err(format!(
                            "Variable type {} does not match expression type {}",
                            variable_type.as_letter(),
                            expression_type.as_letter()
                        ));
                    }
                }
            }
            "expression_statement" => {
                let expression = match child.child(0) {
                    Some(node) => node,
                    None => return Err(String::from("Expression statement is missing expression")),
                };

                let (expression_instructions, _) = parse_expression(
                    &expression,
                    source,
                    current_class,
                    parser_context,
                    &locals,
                    constant_pool,
                )?;

                instructions.extend(expression_instructions);
            }
            "if_statement" => {
                instructions.extend(parse_if(
                    &child,
                    source,
                    current_class,
                    parser_context,
                    &locals,
                    constant_pool,
                    0,
                )?);
            }
            "return_statement" => {
                let return_expression = match child.child(1) {
                    Some(node) => node,
                    None => return Err(String::from("Return statement is missing expression")),
                };

                let (expression_instructions, expression_type) = parse_expression(
                    &return_expression,
                    source,
                    current_class,
                    parser_context,
                    &locals,
                    constant_pool,
                )?;

                // TODO: Check that the return type matches the method return type

                instructions.extend(expression_instructions);
                instructions.push(Instruction::Return(expression_type));
            }
            _ => {}
        }
    }

    Ok(instructions)
}

fn parse_method(
    node: &Node,
    source: &[u8],
    current_class: &String,
    parser_context: &ParserContext,
    constant_pool: &mut Vec<ConstantPoolEntry>,
    method_info: &MethodInfo,
) -> Result<Method, String> {
    let super_locals = method_info.variables.clone();
    let code_block = match node.child_by_kind("block") {
        Ok(node) => node,
        Err(err) => return Err(format!("Failed to parse code block: {}", err)),
    };

    let mut instructions = parse_code_block(
        &code_block,
        source,
        current_class,
        parser_context,
        &super_locals,
        constant_pool,
    )?;

    if method_info.return_type.matches(&PrimitiveType::Null) {
        let last_instruction = match instructions.last() {
            Some(instruction) => instruction,
            None => return Err(String::from("Method has no instructions")),
        };
        match last_instruction {
            Instruction::Return(_return_type) => {}
            _ => instructions.push(Instruction::Return(PrimitiveType::Null)),
        }
    }

    Ok(Method { instructions })
}

fn parse_class(
    node: &Node,
    source: &[u8],
    parser_context: &ParserContext,
) -> Result<Class, String> {
    let class_name = node.name_from_identifier(source)?;
    let class_body = match node.child_by_kind("class_body") {
        Ok(node) => node,
        Err(err) => return Err(format!("Failed to parse class body: {}", err)),
    };
    let class_info = parser_context.find_class(&class_name)?;
    let mut constant_pool = Vec::new();
    let mut methods = HashMap::new();
    let method_nodes = class_body.children_by_kind("method_declaration");

    for (i, method) in method_nodes.iter().enumerate() {
        let method_info = match class_info.methods.get(i) {
            Some(method) => method,
            None => return Err(format!("Failed to find method info for method {}", i)),
        };
        let method_signature = method_info.signature.clone();
        println!("parsing method {}", method_signature);

        let parsed_method = parse_method(
            method,
            source,
            &class_name,
            parser_context,
            &mut constant_pool,
            method_info,
        )?;

        methods.insert(method_signature, parsed_method);
    }

    Ok(Class {
        name: class_name,
        constant_pool,
        static_fields: Default::default(),
        methods,
    })
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

    let class_info = ClassInfo {
        name: class_name,
        super_class: "java/lang/Object".to_string(),
        fields: vec![],
        methods: generate_method_list(&class_body, source)?,
    };

    // TODO: generate method list for every class in project
    let parser_context = ParserContext {
        classes: vec![class_info],
    };

    let parsed_class = parse_class(&class, source, &parser_context)?;
    // println!("Parsed class: {:?}", parsed_class);

    Ok(vec![parsed_class])
}
