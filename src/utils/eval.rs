#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::scanner;
use logos::Logos;

pub fn testutils(mut lexer: logos::Lexer<scanner::Token>) {
    
    println!("{:?}", lexer.next()); 
    
}
#[derive(Debug)]
pub enum Literal{
    Number(f64),
    String {value: scanner::Token},
    Identifier {value: scanner::Token},
    Boolean {value: scanner::Token},
    Nil {value: scanner::Token}
}

pub enum Expression{
    Literal {value: Box<Literal>},
    Unary {operator: scanner::Token, right: Box<Expression>},
    Binary {left: Box<Expression>, operator: scanner::Token, right: Box<Expression>},
    Grouping {Expression: Box<Expression>}
}

impl Expression{
    pub fn to_string(&self) -> String{
        match self{
            Expression::Binary {left, operator, right} => {
                return format!("({:?} {:?} {:?})", operator, left.to_string(), right.to_string());
            },
            Expression::Unary {operator, right} => {
                return format!("({:?} {:?})", operator, right.to_string());
            },
            Expression::Literal {value} => {
                return format!("{:?}", value);
            },
            Expression::Grouping {Expression} => {
                return format!("(Group {:?})", Expression.to_string());
            }
        }
    }
    pub fn printPretty(&self){
        println!("{}", self.to_string()); 
    }
}

#[cfg(test)]
mod tests_ {
    use super::*;
    use std::io::Write;
    use super::Expression::*;
    use super::Literal::*;
    // Todo: Fix this. I want result to be (- (123) (456)) 
    #[test]
    fn test_1() {
        let onetwothree = Expression::Literal{value: Box::from(Number(123.0))};
        let fourfivesix = Expression::Literal{value: Box::from(Number(456.0))};
        let minus_token = scanner::Token::Subtract;
        let ast = Expression::Binary{
            left: Box::new(onetwothree),
            operator: minus_token,
            right: Box::new(fourfivesix),
        };
        ast.printPretty();
    }
}
