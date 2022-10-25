use crate::java_class::{ConstantPoolEntry, ConstantPoolExt};
use crate::{Instruction, Operator, Primitive, PrimitiveType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Method {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct StackFrame {
    pub pc: usize,
    pub locals: Vec<Primitive>,
    pub arrays: Vec<Vec<Primitive>>,
    pub stack: Vec<Primitive>,
    pub method: Method,
    pub class_name: String,
}

impl StackFrame {
    pub fn math(&mut self, operand_type: PrimitiveType, o: Operator) -> Result<(), String> {
        let value2 = self.pop_primitive()?;
        let value1 = self.pop_primitive()?;

        if !value1.is_type(operand_type) {
            return Err(String::from(
                "mismatched operand type for stack frame math function",
            ));
        }

        self.stack.push(Primitive::eval2(value1, value2, o)?);

        Ok(())
    }

    pub fn pop_primitive(&mut self) -> Result<Primitive, String> {
        match self.stack.pop() {
            Some(p) => Ok(p),
            None => Err("Stack is empty".to_string()),
        }
    }

    pub fn pop_int(&mut self) -> Result<i32, String> {
        match self.pop_primitive()? {
            Primitive::Int(i) => Ok(i),
            _ => Err("Expected int when popping from stack".to_string()),
        }
    }

    pub fn pop_long(&mut self) -> Result<i64, String> {
        match self.pop_primitive()? {
            Primitive::Long(i) => Ok(i),
            _ => Err("Expected long when popping from stack".to_string()),
        }
    }

    pub fn pop_ref(&mut self) -> Result<usize, String> {
        match self.pop_primitive()? {
            Primitive::Reference(r) => Ok(r),
            _ => Err("Expected reference when popping from stack".to_string()),
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

#[derive(Debug)]
pub struct Object {
    pub class_name: String,
    pub fields: HashMap<String, Primitive>,
}

#[derive(Debug)]
pub struct Jvm {
    pub class_area: HashMap<String, Class>,
    pub heap: Vec<Object>,
    pub stack_frames: Vec<StackFrame>,
    pub stdout: String,
}

impl Jvm {
    pub fn new(classes: Vec<Class>) -> Jvm {
        let class_area = classes
            .into_iter()
            .map(|c| (c.name.clone(), c))
            .collect::<HashMap<String, Class>>();

        Jvm {
            class_area,
            heap: Vec::new(),
            stack_frames: Vec::new(),
            stdout: String::new(),
        }
    }

    pub fn stack_trace(&self, exception: String) -> String {
        println!("jvm {:?}", self);

        let mut trace = format!("Exception {}\n", exception);

        for sf in self.stack_frames.iter().rev() {
            trace.push_str(&format!(
                "   at project.class.method(source.java:pc {:?})\n",
                sf.pc
            ));
        }

        trace
    }

    pub fn run(&mut self) -> Result<(), String> {
        // Find the main method and push it onto the stack for execution
        for class in self.class_area.values() {
            if class.methods.contains_key("main([Ljava/lang/String;)V") {
                let main_method = match class.methods.get("main([Ljava/lang/String;)V") {
                    Some(m) => m,
                    None => return Err("Could not find main method".to_string()),
                };

                let stack_frame = StackFrame {
                    pc: 0,
                    locals: Vec::new(),
                    arrays: Vec::new(),
                    stack: Vec::new(),
                    method: main_method.clone(),
                    class_name: class.name.clone(),
                };

                self.stack_frames.push(stack_frame);
            }
        }

        // Perform static initialization for all classes
        for class in self.class_area.values() {
            if class.methods.contains_key("<clinit>()V") {
                let method = class.methods.get("<clinit>()V").unwrap().clone();

                self.stack_frames.push(StackFrame {
                    pc: 0,
                    locals: Vec::new(),
                    arrays: Vec::new(),
                    stack: Vec::new(),
                    method,
                    class_name: class.name.clone(),
                });
            }
        }

        while !self.stack_frames.is_empty() {
            self.step()?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let curr_sf = match self.stack_frames.last_mut() {
            Some(sf) => sf,
            None => return Err(String::from("No stack frames")),
        };
        let instruction = match curr_sf.method.instructions.get(curr_sf.pc) {
            Some(i) => i.clone(),
            None => return Err(String::from("No instruction at current pc")),
        };

        // let indent = " ".repeat(current_stack_frame_index * 2);
        // println!("{}stack: {:?}", indent, curr_sf.stack);
        // println!("{}arrays: {:?}", indent, curr_sf.arrays);
        // println!("{}locals: {:?}", indent, curr_sf.locals);
        // println!("{}heap: {:?}", indent, self.heap);
        // println!("{}{} | {:?}\n", indent, curr_sf.pc, instruction);

        match instruction {
            Instruction::Nop => {}
            Instruction::AConstNull => curr_sf.stack.push(Primitive::Null),
            Instruction::Const(value) => curr_sf.stack.push(value),
            Instruction::LoadConst(index) => {
                curr_sf.stack.push(
                    self.class_area
                        .get(&curr_sf.class_name)
                        .unwrap()
                        .constant_pool
                        .get(index - 1)
                        .unwrap()
                        .get_primitive()?,
                );
            }
            // TODO: Check that the stored or loaded type matches the expected type
            Instruction::Load(index, _type_to_load) => curr_sf
                .stack
                .push(curr_sf.locals.get(index).unwrap().clone()),
            Instruction::ALoad(_stored_type) => {
                let index = curr_sf.pop_int()?;
                let array_ref = curr_sf.pop_ref()?;

                let array = curr_sf.arrays.get(array_ref).expect("array not found");
                let value = array.get(index as usize).unwrap().clone();
                curr_sf.stack.push(value);
            }
            Instruction::Store(index, _type_to_store) => {
                if curr_sf.locals.len() <= index {
                    curr_sf.locals.resize(index + 1, Primitive::Null)
                };
                curr_sf.locals[index] = curr_sf.pop_primitive()?;
            }
            Instruction::AStore(_stored_type) => {
                let value = curr_sf.pop_primitive()?;
                let index = curr_sf.pop_int()?;
                let array_ref = curr_sf.pop_ref()?;

                let array = curr_sf.arrays.get_mut(array_ref).expect("array not found");

                if array.len() <= index as usize {
                    array.resize(index as usize + 1, Primitive::Null)
                };

                array[index as usize] = value;
            }
            Instruction::Pop => {
                curr_sf.stack.pop();
            }
            Instruction::Pop2 => {
                if !curr_sf.pop_primitive()?.is_wide() {
                    curr_sf.stack.pop();
                }
            }
            // TODO: Dup instructions interact with wide types differently
            Instruction::Dup => {
                let value = curr_sf.pop_primitive()?;
                curr_sf.stack.push(value.clone());
                curr_sf.stack.push(value);
            }
            Instruction::DupX1 => {
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;

                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
            }
            Instruction::DupX2 => {
                let value3 = curr_sf.pop_primitive()?;
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
            }
            Instruction::Dup2 => {
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;
                curr_sf.stack.push(value1.clone());
                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
            }
            Instruction::Dup2X1 => {
                let value3 = curr_sf.pop_primitive()?;
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;
                curr_sf.stack.push(value2.clone());
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
            }
            Instruction::Dup2X2 => {
                let value4 = curr_sf.pop_primitive()?;
                let value3 = curr_sf.pop_primitive()?;
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;
                curr_sf.stack.push(value3.clone());
                curr_sf.stack.push(value4.clone());
                curr_sf.stack.push(value1);
                curr_sf.stack.push(value2);
                curr_sf.stack.push(value3);
                curr_sf.stack.push(value4);
            }
            Instruction::Swap => {
                let top = curr_sf.pop_primitive()?;
                let second = curr_sf.pop_primitive()?;
                curr_sf.stack.push(top);
                curr_sf.stack.push(second);
            }
            Instruction::Add(operand_type) => curr_sf.math(operand_type, Operator::Add)?,
            Instruction::Sub(operand_type) => curr_sf.math(operand_type, Operator::Sub)?,
            Instruction::Mul(operand_type) => curr_sf.math(operand_type, Operator::Mul)?,
            Instruction::Div(operand_type) => curr_sf.math(operand_type, Operator::Div)?,
            Instruction::Rem(operand_type) => curr_sf.math(operand_type, Operator::Rem)?,
            Instruction::Neg(operand_type) => curr_sf.math(operand_type, Operator::Neg)?,
            Instruction::Shl(operand_type) => curr_sf.math(operand_type, Operator::Shl)?,
            Instruction::Shr(operand_type) => curr_sf.math(operand_type, Operator::Shr)?,
            Instruction::UShr(operand_type) => curr_sf.math(operand_type, Operator::UShr)?,
            Instruction::And(operand_type) => curr_sf.math(operand_type, Operator::And)?,
            Instruction::Or(operand_type) => curr_sf.math(operand_type, Operator::Or)?,
            Instruction::Xor(operand_type) => curr_sf.math(operand_type, Operator::Xor)?,
            Instruction::IInc(index, constant) => {
                curr_sf.locals[index] = Primitive::eval2(
                    curr_sf.locals.get(index).unwrap().clone(),
                    Primitive::Int(constant as i32),
                    Operator::Add,
                )?;
            }
            Instruction::Convert(start_type, end_type) => {
                let converted = curr_sf
                    .pop_primitive()?
                    .eval(Operator::Convert(start_type, end_type))?;
                curr_sf.stack.push(converted);
            }
            Instruction::LCmp => {
                let second = curr_sf.pop_long()?;
                let first = curr_sf.pop_long()?;

                let result = match first - second {
                    0 => 0,
                    x if x > 0 => 1,
                    _ => -1,
                };

                curr_sf.stack.push(Primitive::Int(result));
            }
            // Instruction::FCmpL => {}
            // Instruction::FCmpG => {}
            // Instruction::DCmpL => {}
            // Instruction::DCmpG => {}
            Instruction::If(branch_offset, comparator) => {
                if curr_sf.pop_primitive()?.compare_to_zero(comparator)? {
                    curr_sf.pc += branch_offset;
                    return Ok(());
                }
            }
            Instruction::IfICmp(branch_offset, comparator) => {
                let value2 = curr_sf.pop_primitive()?;
                let value1 = curr_sf.pop_primitive()?;

                if value1.integer_compare(value2, comparator)? {
                    curr_sf.pc += branch_offset;
                    return Ok(());
                }
            }
            Instruction::Goto(branch_offset) => {
                curr_sf.pc += branch_offset;
                return Ok(());
            }
            Instruction::Jsr(branch_offset) => {
                curr_sf.stack.push(Primitive::Reference(curr_sf.pc + 1));
                curr_sf.pc += branch_offset;
                return Ok(());
            }
            Instruction::Ret(index) => {
                curr_sf.pc = match curr_sf.locals.get(index).unwrap() {
                    Primitive::Reference(x) => *x,
                    _ => return Err(String::from("Invalid return address")),
                };
                return Ok(());
            }
            // Instruction::TableSwitch(usize, usize, usize) => {}, // TODO: Implement table switch and lookup switch
            // Instruction::LookupSwitch(usize, usize, usize) => {},
            Instruction::Return(expected_return_type) => {
                if matches!(expected_return_type, PrimitiveType::Null) {
                    self.stack_frames.pop();
                } else {
                    let return_value = curr_sf.pop_primitive()?;

                    // TODO: remove once stack trace is implemented
                    // return Err(String::from("Attempted to return an invalid type"));

                    if !return_value.is_type(expected_return_type) {
                        return Err(String::from("Attempted to return an invalid type"));
                    }

                    self.stack_frames.pop();
                    let stack_frames_length = self.stack_frames.len();

                    if !self.stack_frames.is_empty() {
                        self.stack_frames[stack_frames_length - 1]
                            .stack
                            .push(return_value);
                    }
                }

                return Ok(());
            }
            Instruction::GetStatic(index) => {
                let (class_name, field_name, _field_type) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .field_ref_parser(&index)
                {
                    Some(x) => x,
                    None => {
                        return Err(String::from("Invalid static field reference for GetStatic"))
                    }
                };

                if self.class_area.contains_key(&class_name) {
                    let value = self
                        .class_area
                        .get(&class_name)
                        .unwrap()
                        .static_fields
                        .get(&field_name)
                        .unwrap()
                        .clone();
                    curr_sf.stack.push(value);
                } else {
                    // TODO: Remove
                    if class_name == "java/lang/System" {
                        // Do nothing
                    } else {
                        return Err(format!(
                            "Unable to find static field {}.{}",
                            class_name, field_name
                        ));
                    }
                }
            }
            Instruction::PutStatic(index) => {
                let value = curr_sf.pop_primitive()?;

                let (class_name, field_name, _field_type) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .field_ref_parser(&index)
                {
                    Some(x) => x,
                    None => {
                        return Err(String::from("Invalid static field reference for PutStatic"))
                    }
                };

                match self.class_area.get_mut(&class_name) {
                    Some(ca) => ca.static_fields.insert(field_name, value),
                    None => return Err(String::from("Unable to find class")),
                };
            }
            Instruction::GetField(index) => {
                let object = curr_sf.pop_ref()?;

                let (_class_name, field_name, _field_type) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .field_ref_parser(&index)
                {
                    Some(x) => x,
                    None => return Err(String::from("Invalid field reference for GetField")),
                };

                let field = self
                    .heap
                    .get(object)
                    .unwrap()
                    .fields
                    .get(&field_name)
                    .unwrap();

                curr_sf.stack.push(field.clone());
            }
            Instruction::PutField(index) => {
                let value = curr_sf.pop_primitive()?;
                let reference = curr_sf.pop_ref()?;

                let (_class_name, field_name, _field_type) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .field_ref_parser(&index)
                {
                    Some(x) => x,
                    None => return Err(String::from("Invalid field reference for PutField")),
                };

                self.heap
                    .get_mut(reference)
                    .unwrap()
                    .fields
                    .insert(field_name, value);
            }
            Instruction::InvokeVirtual(index) | Instruction::InvokeSpecial(index) => {
                // TODO: May need to split into separate InvokeVirtual and InvokeSpecial implementations.
                let (class_name, method_name, method_descriptor) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .method_ref_parser(&index)
                {
                    Some(x) => x,
                    None => {
                        return Err(String::from("Method reference not found for InvokeVirtual"))
                    }
                };

                if !self.class_area.contains_key(&class_name) {
                    // println!("Unable to find method {}/{} : {}", class_name, method_name, method_descriptor);
                    // TODO: Move this to standard library
                    if method_name == "println" {
                        let value_string = curr_sf.pop_primitive()?.pretty_print();
                        println!("{}", value_string);
                        self.stdout.push_str(value_string.as_str());
                    }

                    curr_sf.stack.pop();
                    curr_sf.pc += 1;
                    return Ok(());
                }

                let method = self
                    .class_area
                    .get(&class_name)
                    .unwrap()
                    .methods
                    .get(&format!("{}{}", method_name, method_descriptor))
                    .unwrap()
                    .clone();

                let mut method_parameters = Vec::new();

                let param_string_len = method_descriptor
                    .split(')')
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .len()
                    - 1;

                for _i in 0..param_string_len {
                    method_parameters.push(curr_sf.pop_primitive()?);
                }

                method_parameters.push(curr_sf.pop_primitive()?);

                method_parameters.reverse();

                curr_sf.pc += 1;

                self.stack_frames.push(StackFrame {
                    pc: 0,
                    locals: method_parameters,
                    arrays: Vec::new(),
                    stack: vec![],
                    method,
                    class_name,
                });

                return Ok(());
            }
            Instruction::InvokeStatic(index) => {
                let (class_name, method_name, method_descriptor) = match self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .method_ref_parser(&index)
                {
                    Some(x) => x,
                    None => {
                        return Err(String::from(
                            "Could not find method reference for InvokeStatic",
                        ))
                    }
                };

                let method = self
                    .class_area
                    .get(&class_name)
                    .unwrap()
                    .methods
                    .get(&format!("{}{}", method_name, method_descriptor))
                    .unwrap()
                    .clone();

                let mut method_parameters = Vec::new();

                let param_string_len = method_descriptor
                    .split(')')
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .len()
                    - 1;

                // TODO: Check that the parameters passed to the method are the correct types
                for _i in 0..param_string_len {
                    method_parameters.push(curr_sf.pop_primitive()?);
                }

                method_parameters.reverse();

                curr_sf.pc += 1;

                self.stack_frames.push(StackFrame {
                    pc: 0,
                    locals: method_parameters,
                    arrays: Vec::new(),
                    stack: vec![],
                    method,
                    class_name,
                });

                return Ok(());
            }
            // Instruction::InvokeInterface(index) => {}
            // Instruction::InvokeDynamic(index) => {}
            Instruction::New(index) => {
                let class_name = self
                    .class_area
                    .get(&curr_sf.class_name)
                    .unwrap()
                    .constant_pool
                    .class_parser(&index)
                    .unwrap();

                self.heap.push(Object {
                    class_name,
                    fields: HashMap::new(),
                });

                curr_sf
                    .stack
                    .push(Primitive::Reference(self.heap.len() - 1));
            }
            Instruction::NewArray(_a_type) | Instruction::ANewArray(_a_type) => {
                // TODO: Actually implement ANewArray correctly
                let count = curr_sf.pop_int()?;

                let new_array_ref = curr_sf.arrays.len();
                curr_sf
                    .arrays
                    .insert(new_array_ref, Vec::with_capacity(count as usize));
                curr_sf.stack.push(Primitive::Reference(new_array_ref));
            }
            Instruction::ArrayLength => {
                let array_ref = curr_sf.pop_ref()?;
                let array_length = curr_sf.arrays.get(array_ref).unwrap().len();
                curr_sf.stack.push(Primitive::Int(array_length as i32));
            }
            // Instruction::AThrow => {}
            // Instruction::CheckCast(index) => {}
            // Instruction::InstanceOf(index) => {}
            // Instruction::MonitorEnter => {}
            // Instruction::MonitorExit => {}
            // Instruction::Wide(usize) => {}
            // Instruction::MultiANewArray(index, dimensions) => {}
            Instruction::IfNull(branch_offset) => {
                if curr_sf.pop_primitive()?.is_type(PrimitiveType::Null) {
                    curr_sf.pc += branch_offset;
                    return Ok(());
                }
            }
            Instruction::IfNonNull(branch_offset) => {
                if !curr_sf.pop_primitive()?.is_type(PrimitiveType::Null) {
                    curr_sf.pc += branch_offset;
                    return Ok(());
                }
            }
            // Instruction::Breakpoint => {}
            _ => return Err(String::from("Unsupported instruction")),
        }

        curr_sf.pc += 1;
        Ok(())
    }
}
