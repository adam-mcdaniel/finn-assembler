use crate::object::*;
use crate::value::*;
use crate::table::Table;

#[derive(Debug, Clone)]
pub struct Pair<A, B> {
    first: A,
    second: B
}

#[derive(Debug, Clone)]
pub struct Scope {
    table: Table<Pair<Value, Scope>>,
    outer_scope: Option<Box<Scope>>
}

impl Scope {
    pub fn new(outer_scope: Option<Box<Scope>>) -> Self {
        return Self {
            table: Table::new(),
            outer_scope
        }
    }

    fn is_bottom_scope(&self) -> bool {
        match self.outer_scope {
            Some(_) => true,
            None => false
        }
    }

    fn define(&mut self, name: String, object: Pair<Value, Scope>) {
        self.table.set(name, object)
    }

    fn get(&mut self, name: String) -> Pair<Value, Scope> {
        match self.table.get(name.clone()) {
            Some(v) => return v,
            None => {
                // if !self.is_bottom_scope() {
                // }
                (*match self.outer_scope.clone() {
                    Some(v) => v,
                    None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
                }).get(name)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    scope: Scope,
    contents: Vec<Pair<Value, Scope>>,
    instructions: Value,
    outer_pops: i32,
    outer_stack: Option<Box<StackFrame>>
}

impl StackFrame {
    pub fn new(outer_stack: Option<Box<StackFrame>>, scope: Scope, instructions: Value) -> Self {
        return Self {
            scope,
            contents: vec![],
            instructions,
            outer_pops: 0,
            outer_stack
        }
    }

    pub fn from_instructions(instructions: Value) -> Self {
        Self {
            scope: Scope::new(None),
            contents: vec![],
            instructions,
            outer_pops: 0,
            outer_stack: None
        }
    }

    pub fn run(&mut self) {
        for instruction in self.instructions.as_list() {
            match instruction.as_instruction() {
                Instruction::Print => self.pop_value().print(),
                Instruction::Println => self.pop_value().println(),
                Instruction::Add => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a + b);
                    },
                Instruction::Mul => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a * b);
                    },
                Instruction::Sub => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a - b);
                    },
                Instruction::Div => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a / b);
                    },
                Instruction::Mod => {
                    let a = self.pop_value();
                    let b = self.pop_value();
                    self.push_value(a % b);
                    },
                Instruction::Call => {
                    let f = self.pop();
                    self.call(f);
                    },
                Instruction::Load => {
                    let name = self.pop_value().as_string();
                    let value = self.load(name);
                    self.push(value);
                },
                Instruction::Store => {
                    let name = self.pop_value().as_string();
                    let value = self.pop();
                    self.store(name, value);
                },
                Instruction::GetAttr => {
                    let name = self.pop_value().as_string();
                    let object = self.pop_value();
                    self.push_value(object.get_attr(name));
                },
                Instruction::SetAttr => {
                    let name = self.pop_value().as_string();
                    let data = self.pop_value();
                    let mut object = self.pop_value();
                    object.set_attr(name, data);
                    self.push_value(object);
                },
                Instruction::Execute => {
                    let mut foreign_function = self.pop_value();
                    let argument = self.pop_value();
                    self.push_value(
                        foreign_function.call_foreign_function(argument)
                        );
                },
                Instruction::Pass => self.push_value(instruction)
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    fn has_outer_stack(&self) -> bool {
        match self.outer_stack {
            Some(_) => true,
            None => false
        }
    }

    fn call(&mut self, object_and_scope: Pair<Value, Scope>) {
        let mut s = StackFrame::new(
            Some(Box::new(self.clone())),
            object_and_scope.second,
            object_and_scope.first
        );

        s.run();
        for x in 0..s.outer_pops {
            self.pop();
        }

        while !s.is_empty() {
            self.push(s.pop());
        }
    }

    fn load(&mut self, name: String) -> Pair<Value, Scope> {
        self.scope.get(name)
    }

    fn store(&mut self, name: String, object: Pair<Value, Scope>) {
        self.scope.define(name, object)
    }

    fn push(&mut self, object_and_scope: Pair<Value, Scope>) {
        self.contents.push(object_and_scope);
    }

    fn push_value(&mut self, object: Value) {
        self.contents.push(Pair {
            first: object,
            second: self.scope.clone()
        });
    }

    fn pop(&mut self) -> Pair<Value, Scope> {
        if self.is_empty() && self.has_outer_stack() {
            match &mut self.outer_stack {
                Some(s) => {
                    self.outer_pops += 1;
                    s.pop()
                },
                None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
            }
        } else {

            let back: Pair<Value, Scope> = match self.contents.last() {
                Some(v) => v.clone(),
                None => return Pair{first: Value::from_nothing(), second: Scope::new(None)}
            };
            
            self.contents.pop();
            back.clone()
        }
    }

    fn pop_value(&mut self) -> Value {
        if self.is_empty() && self.has_outer_stack() {
            match &mut self.outer_stack {
                Some(s) => {
                    self.outer_pops += 1;
                    s.pop().first
                },
                None => return Value::from_nothing()
            }
        } else {
            let back: Pair<Value, Scope> = self.contents.last().unwrap().clone();
            self.contents.pop();
            back.clone().first
        }
    }
}
