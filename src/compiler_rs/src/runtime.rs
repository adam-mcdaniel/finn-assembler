use crate::object::*;
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

    pub fn is_bottom_scope(&self) -> bool {
        match self.outer_scope {
            Some(_) => true,
            None => false
        }
    }

    pub fn define(&mut self, name: String, object: Pair<Value, Scope>) {
        self.table.set(name, object)
    }

    pub fn get(&mut self, name: String) -> Pair<Value, Scope> {
        match self.table.get(name.clone()) {
            Some(v) => return v,
            None => {
                // if !self.is_bottom_scope() {
                // }
                (*self.outer_scope.clone().unwrap()).get(name)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    scope: Scope,
    contents: Vec<Pair<Value, Scope>>,
    instructions: Value,
    outer_stack: Option<Box<StackFrame>>
}

impl StackFrame {
    pub fn new(outer_stack: Option<Box<StackFrame>>, scope: Scope, instructions: Value) -> Self {
        return Self {
            scope,
            contents: vec![],
            instructions,
            outer_stack
        }
    }

    pub fn from_instructions(instructions: Value) -> Self {
        Self {
            scope: Scope::new(None),
            contents: vec![],
            instructions,
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

    // void run() {
    //     pair<Object, Scope> object_and_scope;
    //     Object object;
    //     Object data;
    //     string name;
    //     for (Object instruction : this->instructions.as_list()) {
    //         switch (instruction.get_type()) {
    //             case Instruction:
    //                 switch (int(instruction.as_number())) {
    //                     case 0: this->pop_value().print(); break;
    //                     case 1: this->pop_value().println(); break;
    //                     case 2: this->push(this->pop_value() + this->pop_value()); break;
    //                     case 3: this->push(this->pop_value() * this->pop_value()); break;
    //                     case 4: this->push(this->pop_value() - this->pop_value()); break;
    //                     case 5: this->push(this->pop_value() / this->pop_value()); break;
    //                     case 6: this->push(this->pop_value() % this->pop_value()); break;
    //                     case 7: this->call(this->pop()); break;
    //                     case 8:
    //                         // object_and_scope = this->load(this->pop().as_string());
    //                         this->push(this->load(this->pop_value().as_string())); break;
    //                     case 9: this->store(this->pop(), this->pop_value().as_string()); break;
    //                     case 10:
    //                         name = this->pop_value().as_string();
    //                         object = this->pop_value();
    //                         this->push(object.get_attr(name));
    //                         break;
    //                     case 11:
    //                         name = this->pop_value().as_string();
    //                         data = this->pop_value();
    //                         object = this->pop_value();
    //                         object.set_attr(data, name);
    //                         this->push(object);
    //                         break;
    //                     case 12: break;
    //                 }
    //                 break;

    //             default:
    //                 // cout << "data: ";
    //                 // instruction.println();
    //                 // cout << "type: " << instruction.get_type() << endl;
    //                 this->push(instruction);
    //                 break;
    //         }
    //     }
    // }

    pub fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    pub fn has_outer_stack(&self) -> bool {
        match self.outer_stack {
            Some(_) => true,
            None => false
        }
    }

    pub fn call(&mut self, object_and_scope: Pair<Value, Scope>) {
        let mut s = StackFrame::new(
            Some(Box::new(self.clone())),
            object_and_scope.second,
            object_and_scope.first
        );

        s.run();
        while !s.is_empty() {
            self.push(s.pop());
        }
    }

    pub fn load(&mut self, name: String) -> Pair<Value, Scope> {
        self.scope.get(name)
    }

    pub fn store(&mut self, name: String, object: Pair<Value, Scope>) {
        self.scope.define(name, object)
    }

    pub fn push(&mut self, object_and_scope: Pair<Value, Scope>) {
        self.contents.push(object_and_scope);
    }

    pub fn push_value(&mut self, object: Value) {
        self.contents.push(Pair {
            first: object,
            second: self.scope.clone()
        });
    }

    pub fn pop(&mut self) -> Pair<Value, Scope> {
        if self.is_empty() && self.has_outer_stack() {
            self.outer_stack.clone().unwrap().pop()
        } else {

            let back: Pair<Value, Scope> = self.contents.last().unwrap().clone();
            self.contents.pop();
            back.clone()
        }
    }


    pub fn pop_value(&mut self) -> Value {
        if self.is_empty() && self.has_outer_stack() {
            self.outer_stack.clone().unwrap().pop().first

        } else {
            let back: Pair<Value, Scope> = self.contents.last().unwrap().clone();
            self.contents.pop();
            back.clone().first
        }
    }
}
