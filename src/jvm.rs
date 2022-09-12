use crate::java_class::ConstantPoolEntry;
use crate::{Comparison, Instruction, Operator, Primitive, PrimitiveType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Method {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct StackFrame {
    pub pc: usize,
    pub locals: Vec<Primitive>,
    pub stack: Vec<Primitive>,
    pub method: Method,
    pub class_name: String,
}

impl StackFrame {
    pub fn math(&mut self, _op_type: PrimitiveType, o: Operator) {
        let a = self.stack.pop().expect("empty stack");
        let b = self.stack.pop().expect("empty stack");

        // TODO: Check that a is the same type as op_type

        self.stack.push(Primitive::eval2(b, a, o));
    }

    pub fn comp(&mut self, comparator: Comparison) -> bool {
        let a = self.stack.pop().expect("empty stack");

        match a {
            Primitive::Int(x) => match comparator {
                Comparison::Equal => x == 0,
                Comparison::NotEqual => x != 0,
                Comparison::LessThan => x < 0,
                Comparison::GreaterThanOrEqual => x >= 0,
                Comparison::GreaterThan => x > 0,
                Comparison::LessThanOrEqual => x <= 0,
            },
            Primitive::Long(x) => match comparator {
                Comparison::Equal => x == 0,
                Comparison::NotEqual => x != 0,
                Comparison::LessThan => x < 0,
                Comparison::GreaterThanOrEqual => x >= 0,
                Comparison::GreaterThan => x > 0,
                Comparison::LessThanOrEqual => x <= 0,
            },
            Primitive::Float(x) => match comparator {
                Comparison::Equal => x == 0.0,
                Comparison::NotEqual => x != 0.0,
                Comparison::LessThan => x < 0.0,
                Comparison::GreaterThanOrEqual => x >= 0.0,
                Comparison::GreaterThan => x > 0.0,
                Comparison::LessThanOrEqual => x <= 0.0,
            },
            Primitive::Double(x) => match comparator {
                Comparison::Equal => x == 0.0,
                Comparison::NotEqual => x != 0.0,
                Comparison::LessThan => x < 0.0,
                Comparison::GreaterThanOrEqual => x >= 0.0,
                Comparison::GreaterThan => x > 0.0,
                Comparison::LessThanOrEqual => x <= 0.0,
            },
            _ => panic!("unsupported type for comparison"),
        }
    }

    pub fn i_comp(&mut self, comparator: Comparison) -> bool {
        let a = self.stack.pop().expect("empty stack");
        let b = self.stack.pop().expect("empty stack");

        match (b, a) {
            (Primitive::Int(x), Primitive::Int(y)) => match comparator {
                Comparison::Equal => x == y,
                Comparison::NotEqual => x != y,
                Comparison::LessThan => x < y,
                Comparison::GreaterThanOrEqual => x >= y,
                Comparison::GreaterThan => x > y,
                Comparison::LessThanOrEqual => x <= y,
            },
            _ => panic!("comparing non-int types"),
        }
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub constant_pool: Vec<ConstantPoolEntry>,
    pub static_fields: HashMap<String, Primitive>,
    pub methods: HashMap<String, Method>,
}

pub struct Object {
    pub class_name: String,
    pub fields: HashMap<String, Primitive>,
}

pub struct JVM {
    pub class_area: HashMap<String, Class>,
    pub heap: Vec<Object>,
    pub stack_frames: Vec<StackFrame>,
}

impl JVM {
    pub fn new(classes: Vec<Class>) -> JVM {
        let class_area = classes
            .into_iter()
            .map(|c| (c.name.clone(), c))
            .collect::<HashMap<String, Class>>();

        JVM {
            class_area,
            heap: Vec::new(),
            stack_frames: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let method = self.class_area["Main"].methods["main([Ljava/lang/String;)V"].clone();

        self.stack_frames.push(StackFrame {
            pc: 0,
            locals: Vec::new(),
            stack: Vec::new(),
            method,
            class_name: "Main".to_string(),
        });

        while !self.stack_frames.is_empty() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let current_stack_frame_index = self.stack_frames.len() - 1;
        let curr_sf = &mut self.stack_frames[current_stack_frame_index];
        let instruction = curr_sf.method.instructions[curr_sf.pc].clone();

        match instruction {
            Instruction::Nop => {}
            Instruction::AConstNull => curr_sf.stack.push(Primitive::Null),
            Instruction::Const(value) => curr_sf.stack.push(value.clone()),
            Instruction::LoadConst(index) => {
                curr_sf.stack.push(
                    self.class_area[&curr_sf.class_name].constant_pool[index].get_primitive(),
                );
            }
            // TODO: Check that the stored or loaded type matches the expected type
            Instruction::Load(index, _type_to_load) => {
                curr_sf.stack.push(curr_sf.locals[index].clone())
            }
            // Instruction::ALoad(stored_type) => {}
            Instruction::Store(index, _type_to_store) => {
                if curr_sf.locals.len() <= index {
                    curr_sf.locals.resize(index + 1, Primitive::Null)
                };
                curr_sf.locals[index] = curr_sf.stack.pop().expect("empty stack")
            }
            // Instruction::AStore(stored_type) => {}
            Instruction::Pop => {
                curr_sf.stack.pop();
            }
            Instruction::Pop2 => {
                if !curr_sf.stack.pop().expect("empty stack").is_wide() {
                    curr_sf.stack.pop();
                }
            }
            // TODO: Dub instructions interact with wide types differently
            Instruction::Dup => {
                let top = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(top);
            }
            Instruction::DupX1 => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(second);
                curr_sf.stack.push(top);
            }
            Instruction::DupX2 => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                let third = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(third);
                curr_sf.stack.push(second);
                curr_sf.stack.push(top);
            }
            Instruction::Dup2 => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(second.clone());
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(second);
                curr_sf.stack.push(top);
            }
            Instruction::Dup2X1 => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                let third = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(second.clone());
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(third);
                curr_sf.stack.push(second);
                curr_sf.stack.push(top);
            }
            Instruction::Dup2X2 => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                let third = curr_sf.stack.pop().expect("empty stack");
                let fourth = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(second.clone());
                curr_sf.stack.push(top.clone());
                curr_sf.stack.push(fourth);
                curr_sf.stack.push(third);
                curr_sf.stack.push(second);
                curr_sf.stack.push(top);
            }
            Instruction::Swap => {
                let top = curr_sf.stack.pop().expect("empty stack");
                let second = curr_sf.stack.pop().expect("empty stack");
                curr_sf.stack.push(top);
                curr_sf.stack.push(second);
            }
            Instruction::Add(operand_type) => curr_sf.math(operand_type, Operator::Add),
            Instruction::Sub(operand_type) => curr_sf.math(operand_type, Operator::Sub),
            Instruction::Mul(operand_type) => curr_sf.math(operand_type, Operator::Mul),
            Instruction::Div(operand_type) => curr_sf.math(operand_type, Operator::Div),
            Instruction::Rem(operand_type) => curr_sf.math(operand_type, Operator::Rem),
            Instruction::Neg(operand_type) => curr_sf.math(operand_type, Operator::Neg),
            Instruction::Shl(operand_type) => curr_sf.math(operand_type, Operator::Shl),
            Instruction::Shr(operand_type) => curr_sf.math(operand_type, Operator::Shr),
            Instruction::UShr(operand_type) => curr_sf.math(operand_type, Operator::UShr),
            Instruction::And(operand_type) => curr_sf.math(operand_type, Operator::And),
            Instruction::Or(operand_type) => curr_sf.math(operand_type, Operator::Or),
            Instruction::Xor(operand_type) => curr_sf.math(operand_type, Operator::Xor),
            Instruction::IInc(index, constant) => {
                curr_sf.locals[index] = Primitive::eval2(
                    curr_sf.locals[index].clone(),
                    Primitive::Int(constant as i32),
                    Operator::Add,
                );
            }
            Instruction::Convert(start_type, end_type) => {
                let converted = Primitive::eval(
                    curr_sf.stack.pop().expect("empty stack"),
                    Operator::Convert(start_type.clone(), end_type.clone()),
                );
                curr_sf.stack.push(converted);
            }
            // TODO: Implement branching
            Instruction::LCmp => {}
            Instruction::FCmpL => {}
            Instruction::FCmpG => {}
            Instruction::DCmpL => {}
            Instruction::DCmpG => {}
            Instruction::If(branch_offset, comparator) => {
                if curr_sf.comp(comparator) {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::IfICmp(branch_offset, comparator) => {
                if curr_sf.i_comp(comparator) {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::Goto(branch_offset) => {
                curr_sf.pc += branch_offset;
                return;
            }
            Instruction::Jsr(branch_offset) => {
                curr_sf.stack.push(Primitive::Reference(curr_sf.pc + 1));
                curr_sf.pc += branch_offset;
                return;
            }
            Instruction::Ret(index) => {
                curr_sf.pc = match curr_sf.locals[index] {
                    Primitive::Reference(x) => x,
                    _ => panic!("invalid return address"),
                };
                return;
            }
            // Instruction::TableSwitch(usize, usize, usize) => {}, // TODO: Implement table switch and lookup switch
            // Instruction::LookupSwitch(usize, usize, usize) => {},
            Instruction::Return(_return_type) => {
                // TODO: Check that the return type matches the method's return type

                let return_value = curr_sf.stack.pop().expect("empty stack");
                let stack_frames_length = self.stack_frames.len() - 1;
                self.stack_frames.pop();

                if !self.stack_frames.is_empty() {
                    self.stack_frames[stack_frames_length - 1]
                        .stack
                        .push(return_value);
                }

                return;
            }
            Instruction::GetStatic(index) => {}
            Instruction::PutStatic(index) => {}
            Instruction::GetField(index) => {}
            Instruction::PutField(index) => {}
            Instruction::InvokeVirtual(index) => {
                println!("{:?}", curr_sf.stack);
            }
            Instruction::InvokeSpecial(index) => {}
            Instruction::InvokeStatic(index) => {
                let (class_name, method_name, method_descriptor) =
                    ConstantPoolEntry::method_ref_parser(
                        index,
                        &self.class_area[&curr_sf.class_name].constant_pool[..],
                    );

                let method = self.class_area[&class_name].methods
                    [&format!("{}{}", method_name, method_descriptor)]
                    .clone();

                let mut method_parameters: Vec<Primitive> = Vec::new();

                let param_string_len =
                    method_descriptor.split(')').collect::<Vec<&str>>()[0].len() - 1;

                // TODO: Check that the parameters passed to the method are the correct types
                for _i in 0..param_string_len {
                    method_parameters.push(curr_sf.stack.pop().expect("empty stack"));
                }

                curr_sf.pc += 1;

                let frame = StackFrame {
                    pc: 0,
                    locals: method_parameters,
                    stack: vec![],
                    method,
                    class_name: curr_sf.class_name.clone(),
                };

                self.stack_frames.push(frame);

                return;
            }
            Instruction::InvokeInterface(index) => {}
            Instruction::InvokeDynamic(index) => {}
            Instruction::New(index) => {}
            Instruction::NewArray(a_type) => {}
            Instruction::ANewArray(index) => {}
            Instruction::ArrayLength => {}
            Instruction::AThrow => {}
            Instruction::CheckCast(index) => {}
            Instruction::InstanceOf(index) => {}
            Instruction::MonitorEnter => {}
            Instruction::MonitorExit => {}
            Instruction::Wide(usize) => {}
            Instruction::MultiANewArray(index, dimensions) => {}
            Instruction::IfNull(branch_offset) => {
                if let Primitive::Null = curr_sf.stack.pop().expect("empty stack") {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::IfNonNull(branch_offset) => {
                if let Primitive::Null = curr_sf.stack.pop().expect("empty stack") {
                    // Do nothing
                } else {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::Breakpoint => {}
            _ => panic!("unsupported instruction"),
        }

        curr_sf.pc += 1;
    }
}
