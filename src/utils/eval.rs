#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::scanner;
use logos::Logos;

#[derive(Debug, Copy, Clone)]
pub enum Value{
    Number(f64),
    Str(&'static str),
    Bool(bool),
    Nil,
}

pub struct ValArray{
    pub values: Vec<Value>,
}

impl ValArray{

    pub fn new() -> Self{
        Self {
            values: Vec::new(),
        }
    }
    pub fn write(&mut self, value: Value){
        
        self.values.push(value);
    }
    pub fn free(&mut self){
        self.values = Vec::new();
    }
    pub fn get(&self, index: usize) -> Value{
        self.values[index]
    }
    pub fn set(&mut self, index: usize, value: Value){
        self.values[index] = value;
    }
    pub fn print_value(&self, which: usize) {
        print!("{:?}\n", self.values[which]);
    }

}


#[cfg(test)]
mod tests_ {
    use super::*;
    
    // Todo: Fix this. I want result to be (- (123) (456)) 
    #[test]
    fn test_1() {
        
    }
}
