use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div, Rem};

use bigdecimal::*;
use crate::table::Table;

pub type Contents = Vec<BigDecimal>;
pub const NOTHING : &[BigDecimal] = &[];


fn to_decimal(num: f64) -> BigDecimal {
    match BigDecimal::from_str(&num.to_string()) {
        Ok(v) => v,
        Err(_) => BigDecimal::from_str("0").unwrap()
    }
}


fn from_string(string: String) -> Contents {
    let mut result = vec![];
    for ch in string.chars() {
        result.push(
            match BigDecimal::from_str(&(ch as i32).to_string()) {
                Ok(s) => s,
                Err(_) => return NOTHING.to_vec()
            }
        );
    }
    return result;
}


fn from_number(number: BigDecimal) -> Contents {
    return vec![number];
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
