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
    pub arrays: HashMap<usize, Vec<Primitive>>,
    pub stack: Vec<Primitive>,
    pub method: Method,
    pub class_name: String,
}

impl StackFrame {
    pub fn math(&mut self, operand_type: PrimitiveType, o: Operator) {
        let value2 = self.pop_primitive();
        let value1 = self.pop_primitive();

        if !value1.is_type(operand_type) {
            panic!("mismatched operand type for stack frame math function");
        }

        self.stack.push(Primitive::eval2(value1, value2, o));
    }

    pub fn pop_primitive(&mut self) -> Primitive {
        self.stack.pop().expect("empty stack")
    }

    pub fn pop_int(&mut self) -> i32 {
        match self.pop_primitive() {
            Primitive::Int(x) => x,
            _ => panic!("sp_int on non-int type"),
        }
    }

    pub fn pop_long(&mut self) -> i64 {
        match self.pop_primitive() {
            Primitive::Long(x) => x,
            _ => panic!("sp_long on non-long type"),
        }
    }

    pub fn pop_ref(&mut self) -> usize {
        match self.pop_primitive() {
            Primitive::Reference(x) => x,
            _ => panic!("sp_ref on non-reference type"),
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
            arrays: HashMap::new(),
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

        // println!("stack: {:?}", curr_sf.stack);
        // println!("arrays: {:?}", curr_sf.arrays);
        // println!("{} | {:?}\n", curr_sf.pc, instruction);

        match instruction {
            Instruction::Nop => {}
            Instruction::AConstNull => curr_sf.stack.push(Primitive::Null),
            Instruction::Const(value) => curr_sf.stack.push(value.clone()),
            Instruction::LoadConst(index) => {
                curr_sf.stack.push(
                    self.class_area[&curr_sf.class_name].constant_pool[index - 1].get_primitive(),
                );
            }
            // TODO: Check that the stored or loaded type matches the expected type
            Instruction::Load(index, _type_to_load) => {
                curr_sf.stack.push(curr_sf.locals[index].clone())
            }
            Instruction::ALoad(_stored_type) => {
                let index = curr_sf.pop_int();
                let array_ref = curr_sf.pop_ref();

                let array = curr_sf.arrays.get(&array_ref).expect("array not found");
                let value = array[index as usize].clone();
                curr_sf.stack.push(value);
            }
            Instruction::Store(index, _type_to_store) => {
                if curr_sf.locals.len() <= index {
                    curr_sf.locals.resize(index + 1, Primitive::Null)
                };
                curr_sf.locals[index] = curr_sf.pop_primitive()
            }
            Instruction::AStore(_stored_type) => {
                let value = curr_sf.pop_primitive();
                let index = curr_sf.pop_int();
                let array_ref = curr_sf.pop_ref();

                let array = curr_sf.arrays.get_mut(&array_ref).expect("array not found");

                if array.len() <= index as usize {
                    array.resize(index as usize + 1, Primitive::Null)
                };

                array[index as usize] = value;
            }
            Instruction::Pop => {
                curr_sf.stack.pop();
            }
            Instruction::Pop2 => {
                if !curr_sf.pop_primitive().is_wide() {
                    curr_sf.stack.pop();
                }
            }
            // TODO: Dup instructions interact with wide types differently
            Instruction::Dup => {
                let value = curr_sf.pop_primitive();
                curr_sf.stack.push(value.clone());
                curr_sf.stack.push(value);
            }
            Instruction::DupX1 => {
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();

                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
            }
            Instruction::DupX2 => {
                let value3 = curr_sf.pop_primitive();
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
            }
            Instruction::Dup2 => {
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();
                curr_sf.stack.push(value1.clone());
                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
            }
            Instruction::Dup2X1 => {
                let value3 = curr_sf.pop_primitive();
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();
                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
            }
            Instruction::Dup2X2 => {
                let value4 = curr_sf.pop_primitive();
                let value3 = curr_sf.pop_primitive();
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value4.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
                curr_sf.stack.push(value4);
            }
            Instruction::Swap => {
                let top = curr_sf.pop_primitive();
                let second = curr_sf.pop_primitive();
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
                    curr_sf.pop_primitive(),
                    Operator::Convert(start_type, end_type),
                );
                curr_sf.stack.push(converted);
            }
            Instruction::LCmp => {
                let second = curr_sf.pop_long();
                let first = curr_sf.pop_long();

                let result = match first - second {
                    0 => 0,
                    x if x > 0 => 1,
                    _ => -1,
                };

                curr_sf.stack.push(Primitive::Int(result));
            }
            Instruction::FCmpL => {}
            Instruction::FCmpG => {}
            Instruction::DCmpL => {}
            Instruction::DCmpG => {}
            Instruction::If(branch_offset, comparator) => {
                if Primitive::compare_to_zero(curr_sf.pop_primitive(), comparator) {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::IfICmp(branch_offset, comparator) => {
                let value2 = curr_sf.pop_primitive();
                let value1 = curr_sf.pop_primitive();

                if Primitive::integer_compare(value1, value2, comparator) {
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
            Instruction::Return(expected_return_type) => {
                if matches!(expected_return_type, PrimitiveType::Null) {
                    self.stack_frames.pop();
                } else {
                    let return_value = curr_sf.pop_primitive();

                    if !return_value.is_type(expected_return_type) {
                        panic!("attempted to return an invalid type");
                    }

                    self.stack_frames.pop();
                    let stack_frames_length = self.stack_frames.len();

                    if !self.stack_frames.is_empty() {
                        self.stack_frames[stack_frames_length - 1]
                            .stack
                            .push(return_value);
                    }
                }

                return;
            }
            Instruction::GetStatic(index) => {}
            Instruction::PutStatic(index) => {}
            Instruction::GetField(index) => {}
            Instruction::PutField(index) => {}
            Instruction::InvokeVirtual(index) => {
                println!("{:?}", curr_sf.stack);

                curr_sf.stack = Vec::new();
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
                    method_parameters.push(curr_sf.pop_primitive());
                }

                curr_sf.pc += 1;

                let frame = StackFrame {
                    pc: 0,
                    locals: method_parameters,
                    arrays: HashMap::new(),
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
            Instruction::NewArray(_a_type) => {
                // TODO: Check that the type is a valid array type
                let count = curr_sf.pop_int();

                let new_array_ref = curr_sf.arrays.len();
                curr_sf
                    .arrays
                    .insert(new_array_ref, Vec::with_capacity(count as usize));
                curr_sf.stack.push(Primitive::Reference(new_array_ref));
            }
            Instruction::ANewArray(index) => {}
            Instruction::ArrayLength => {
                let array_ref = curr_sf.pop_ref();
                let array_length = curr_sf.arrays[&array_ref].len();
                curr_sf.stack.push(Primitive::Int(array_length as i32));
            }
            Instruction::AThrow => {}
            Instruction::CheckCast(index) => {}
            Instruction::InstanceOf(index) => {}
            Instruction::MonitorEnter => {}
            Instruction::MonitorExit => {}
            Instruction::Wide(usize) => {}
            Instruction::MultiANewArray(index, dimensions) => {}
            Instruction::IfNull(branch_offset) => {
                if curr_sf.pop_primitive().is_type(PrimitiveType::Null) {
                    curr_sf.pc += branch_offset;
                    return;
                }
            }
            Instruction::IfNonNull(branch_offset) => {
                if !curr_sf.pop_primitive().is_type(PrimitiveType::Null) {
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
