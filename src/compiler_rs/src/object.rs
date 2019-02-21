use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div, Rem};

use bigdecimal::*;
use crate::table::Table;

type Contents = Vec<BigDecimal>;
pub const NOTHING : &[BigDecimal] = &[];


fn to_decimal(num: f64) -> BigDecimal {
    return BigDecimal::from_str(&num.to_string()).unwrap()
}


fn from_string(string: String) -> Contents {
    let mut result = vec![];
    for ch in string.chars() {
        result.push(
            BigDecimal::from_str(&(ch as i32).to_string()).unwrap()
        );
    }
    return result;
}


fn from_number(number: BigDecimal) -> Contents {
    return vec![number];
}


pub fn num_from_str(number: &str) -> BigDecimal {
    BigDecimal::from_str(number).unwrap()
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Print,
    Println,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    Call,
    Load,
    Store,
    GetAttr,
    SetAttr,
    Execute,
    Pass
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Problem {
    IncompatibleTypes,
    ValueError,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Str,
    Num,
    List,
    Function,
    Instance,
    Problem(Problem),
    Nothing,
    Command(Instruction)
}


pub trait Object: Sized + Clone + Debug + Display + Add + Sub + Mul + Div + Rem {
    // initializers
    fn new(value_type: Type, contents: Contents) -> Self;

    fn from_string(string: String) -> Self {
        return Self::new(Type::Str, from_string(string));
    }

    fn from_str(string: &str) -> Self {
        return Self::new(Type::Str, from_string(string.to_string()));
    }

    fn from_f64(decimal: f64) -> Self {
        return Self::new(Type::Num, from_number(to_decimal(decimal)));
    }

    fn from_number(decimal: BigDecimal) -> Self {
        return Self::new(Type::Num, from_number(decimal));
    }

    fn from_instruction(instruction: Instruction) -> Self {
        return Self::new(Type::Command(instruction), NOTHING.to_vec());
    }

    fn from_problem(problem: Problem) -> Self {
        return Self::new(Type::Problem(problem), NOTHING.to_vec());
    }

    fn from_vector(vector: Vec<Self>) -> Self {
        let mut instance = Self::new(Type::List, NOTHING.to_vec());
        instance.set_list(vector);
        return instance;
    }

    fn from_nothing() -> Self {
        Self::new(
            Type::Nothing,
            NOTHING.to_vec()
            )
    }

    fn from_foreign_function(function: fn(Self) -> Self) -> Self {
        let mut instance = Self::new(
            Type::Function,
            NOTHING.to_vec()
            );
        instance.set_foreign_function(function);
        return instance;
    }

    // helper functions
    fn get_type(&self) -> Type;
    fn get_list(&self) -> Vec<Self>;
    fn get_contents(&self) -> Contents;
    fn get_attributes(&self) -> Table<Self>;
    fn get_foreign_function(&self) -> fn(Self) -> Self;

    fn set_type(&mut self, object_type: Type);
    fn set_list(&mut self, list: Vec<Self>);
    fn set_contents(&mut self, contents: Contents);
    fn set_attributes(&mut self, attributes: Table<Self>);
    fn set_foreign_function(&mut self, function: fn(Self) -> Self);

    // getters
    fn get_attr(&self, name: String) -> Self {
        let table = self.get_attributes();
        let raw_attr = table.get(name);
        let attr = match raw_attr {
            Some(s) => s,
            None => Self::new(Type::Nothing, NOTHING.to_vec())
        };
        return attr;
    }

    fn as_number(&self) -> BigDecimal {
        self.get_contents()[0].clone()
    }

    fn as_string(&self) -> String {
        let mut result = "".to_string();

        for ch in self.get_contents() {
            let character = match ch.to_i32() {
                Some(i) => i as u8 as char,
                None => ' '
            };
            result += &character.to_string();
        }
        return result.to_string();
    }
    
    fn as_list(&self) -> Vec<Self> {
        self.get_list()
    }

    fn as_instruction(&self) -> Instruction {
        match self.get_type() {
            Type::Command(i) => i,
            _ => Instruction::Pass
        }
    }
    
    fn as_instance(&self) -> Table<Self> {
        return self.get_attributes();
    }
    
    
    fn as_foreign_function(&self) -> fn(Self) -> Self {
        return self.get_foreign_function();
    }
    
    // setters
    fn set_attr(&mut self, name: String, object: Self) {
        let mut table = self.get_attributes();
        table.set(name, object);
        self.set_attributes(table);
    }


    fn list_push(&mut self, object: Self) {
        self.set_type(Type::List);

        let mut list = self.get_list();
        list.push(object);
        self.set_list(list);
    }

    fn list_pop(&mut self) -> Self {
        match self.get_list().pop() {
            Some(e) => e,
            None => Self::new(Type::Nothing, NOTHING.to_vec())
        }
    }

    fn call_foreign_function(&mut self, parameter: Self) -> Self {
        self.get_foreign_function()(parameter)
    }

    fn format(&self) -> String {
        let object_type = self.get_type();
        match object_type {
            Type::Str => format!("{}", self.as_string()),
            Type::Num => format!("{}", self.as_number()),
            Type::List => {
                let mut result = "[".to_string();
                for item in self.as_list() {
                    result += &item.format();
                    result += ", ";
                }
                result.pop();
                result.pop();
                result + "]"
                },
            Type::Instance => {
                // format!("{:?}", self.as_instance())
                let mut result = "<".to_string();
                for key in self.get_attributes().keys() {
                    result += &key;
                    result += ":";
                    result += &self.get_attr(key).format();
                    result += ", ";
                }
                result.pop();
                result.pop();
                result + ">"
            },
            Type::Function => format!("Function"),
            Type::Nothing => format!("None"),
            Type::Problem(p) => format!("{:?}", p),
            Type::Command(c) => format!("{:?}", c),
            _ => "".to_string()
        }
    }

    fn print(&self) {
        print!("{}", self.format());
    }

    fn println(&self) {
        println!("{}", self.format());
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    value_type: Type,
    function: fn(Self) -> Self,
    contents: Contents,
    list: Vec<Self>,
    attributes: Table<Self>,
}

impl Display for Value {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.print();
        Ok(())
    }
}


impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        match self.value_type {
            Type::Num => Value::from_number(self.as_number() + rhs.as_number()),
            Type::Str => Value::from_string(self.as_string() + &rhs.as_string()),
            _ => Value::from_problem(Problem::ValueError)
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() - rhs.as_number())
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() * rhs.as_number())
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() / rhs.as_number())
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        if self.value_type != rhs.value_type {
            return Value::from_problem(Problem::IncompatibleTypes);
        }

        if self.value_type != Type::Num {
            return Value::from_problem(Problem::ValueError);
        }

        return Value::from_number(self.as_number() % rhs.as_number())
    }
}


impl Object for Value {
    fn new(value_type: Type, contents: Contents) -> Self {
        Self {
            value_type: value_type,
            contents: contents,
            function: |object: Self| object,
            list: vec![],
            attributes: Table::new()
        }
    }

    fn get_type(&self) -> Type {self.value_type.clone()}
    fn get_list(&self) -> Vec<Self> {self.list.clone()}
    fn get_contents(&self) -> Contents {self.contents.clone()}
    fn get_attributes(&self) -> Table<Self> {self.attributes.clone()}
    fn get_foreign_function(&self) -> fn(Self) -> Self {self.function.clone()}

    fn set_type(&mut self, value_type: Type) {self.value_type = value_type}
    fn set_list(&mut self, list: Vec<Self>) {self.list = list}
    fn set_contents(&mut self, contents: Contents) {self.contents = contents}
    fn set_attributes(&mut self, attributes: Table<Self>) {self.attributes = attributes}
    fn set_foreign_function(&mut self, function: fn(Self) -> Self) {self.function = function}
}