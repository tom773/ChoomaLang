use crate::utils::eval;
use std::arch::asm;

use super::eval::ValArray;
use super::eval::Value;

pub enum OpCode{
    OpConstant = 0,
    OpReturn = 1,
}

impl From<u8> for OpCode{
    fn from(v: u8) -> Self{
        match v{
            0 => OpCode::OpConstant,
            1 => OpCode::OpReturn,
            _ => panic!("Invalid OpCode"),
        }
    }
}

impl From<OpCode> for u8{
    fn from(v: OpCode) -> Self{
        v as u8
    }
}

pub struct Chunk{
    code: Vec<u8>,
    constants: ValArray, 
}

impl Chunk{
    pub fn new() -> Self{
        Self {
            code: Vec::new(),
            constants: ValArray::new(),
        }
    }
    pub fn write(&mut self, byte: u8){
        self.code.push(byte);
    }
    pub fn write_op(&mut self, op: OpCode){
        self.write(op.into());
    }

    pub fn read(&self, index: usize) -> u8{
        self.code[index]
    }

    pub fn add_constant(&mut self, value: ValArray) {
        for i in 0..value.values.len(){
            self.constants.write(value.get(i));
        }
    }

    pub fn get_const_len(&self) -> usize{
        self.constants.values.len()
    }
    
    pub fn free(&mut self){
        self.code = Vec::new();
    }

    pub fn print_code(&mut self){
        for i in 0..self.code.len(){
            print!("{:?}", self.code[i]);
        }
    }

    pub fn disassemble(&mut self){
        let mut offset = 0;
        println!("{:?}", self.print_code());
        println!("Instruction Count: {}\n", self.code.len());    
        while offset < self.code.len(){
            offset = self.disassemble_instruction(offset); 
        }
    }
    fn disassemble_instruction(&self, offset: usize) -> usize{
        let instruction: OpCode = self.code[offset].into(); 
        match instruction{
            
            OpCode::OpConstant => {self.constant_instruction("OP_CONSTANT", offset)},
            OpCode::OpReturn => {self.sInstruction("OP_RETURN", offset)},
        }
    }

    fn sInstruction(&self, name: &str, offset: usize) -> usize{
        println!("=Simple Instruction=\n{}\n", name);
        return offset + 1;
    }
    
    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        print!("=Constant Instruction=");
        print!("\n{:?}\n", self.constants.values[offset]);
        
        print!("\n");
        return offset + 1;
    }}

#[cfg(test)]
mod tests_ {
    use super::*;
    use std::io::Write;
    use super::OpCode::*;
    use super::eval::Value;
    #[test]
    fn test_1() {
        let mut chunk = Chunk::new();
        let mut val = ValArray::new();
        val.write(Value::Number(1.2));
        val.write(Value::Str("Choom"));
        val.write(Value::Number(3.5));
        chunk.add_constant(val);
        chunk.write_op(OpConstant);
        chunk.write_op(OpReturn);
        chunk.write_op(OpConstant);
        chunk.disassemble();
    }
}
