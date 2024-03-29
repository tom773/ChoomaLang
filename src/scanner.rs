use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token{
        
    // Literals
    #[regex("[0-9]+")] Number,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")] Identifier,
    #[regex(r#""[^"]*""#)] String,

    // Single character tokens
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[token("{")] LeftBrace,
    #[token("}")] RightBrace,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token(".")] Period,
    
    // Operators
    #[token("+")] Add,
    #[token("-")] Subtract,
    #[token("*")] Multiply,
    #[token("/")] Divide,

    // Chooma
    #[token("chooma")] Chooma,

    // One or two character tokens
    #[token("!=")] BangEqual,
    #[token("=")] Equal,
    #[token("==")] EqualEqual,
    #[token(">")] Greater,
    #[token(">=")] GreaterEqual,
    #[token("<")] Less,
    #[token("<=")] LessEqual,
    #[token("!")] Bang,

    // Keywords
    #[token("and")] And,
    #[token("class")] Class,
    #[token("else")] Else,
    #[token("false")] False,
    #[token("for")] For,
    #[token("fun")] Fun,
    #[token("if")] If,
    #[token("nil")] Nil,
    #[token("or")] Or,
    #[token("print")] Print,
    #[token("return")] Return,
    #[token("super")] Super,
    #[token("this")] This,
    #[token("true")] True,
    #[token("var")] Var,
    #[token("while")] While,

}


