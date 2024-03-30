use crate::utils::eval;
use std::arch::asm;

pub enum OpCode{
    OpReturn = 0,
}

impl From<u8> for OpCode{
    fn from(v: u8) -> Self{
        match v{
            0 => OpCode::OpReturn,
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
}

impl Chunk{
    pub fn new() -> Self{
        Self {
            code: Vec::new(),
        }
    }
    pub fn write(&mut self, byte: u8){
        self.code.push(byte);
    }
    pub fn write_op(&mut self, op: OpCode){
        self.write(op.into());
    }
    
    pub fn free(&mut self){
        self.code = Vec::new();
    }

    pub fn disassemble(&self){
        let mut offset = 0;
        while offset < self.code.len(){
            offset = self.disassemble_instruction(offset);
        }
    }
    fn disassemble_instruction(&self, offset: usize) -> usize{
        print!("=== Instruction ===\n");
        if offset > 0 && self.code[offset] == self.code[offset - 1]{
            print!("   | ");
        }else{
            print!("{:04} ", offset);
        }
        let instruction = self.code[offset];
        match instruction.into(){
            OpCode::OpReturn => {
                println!("OP_RETURN");
                return offset + 1;
            }
        }
    }

    fn sInstruction(&self, name: &str, offset: usize) -> usize{
        println!("{}", name);
        return offset + 1;
    }
}

#[cfg(test)]
mod tests_ {
    use super::*;
    use std::io::Write;
    use super::OpCode::*;
    #[test]
    fn test_1() {
        let mut chunk = Chunk::new();
        chunk.write_op(OpReturn);
        chunk.disassemble();
    }
}
