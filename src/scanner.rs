pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32
}

impl Scanner {
    pub fn new(_source: &str) -> Self {
        Self {
            source: _source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        while (!self.is_at_end()) {
            let start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), LiteralValue::NilValue, self.line));

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self){
       let c = self.advance();

       macro_rules! add_token {
           ($char:expr, $token:ident) => {
                $char =>self.add_token($token);
           }
       }

       match c {
           '(' => self.add_token(TokenType::LeftParen),
           ')' => self.add_token(TokenType::RightParen),
           '{' => self.add_token(TokenType::LeftBrace),
           '}' => self.add_token(TokenType::RightBrace),
           ',' => self.add_token(TokenType::Comma),
           '.' => self.add_token(TokenType::Dot),
           '-' => self.add_token(TokenType::Minus),
           '+' => self.add_token(TokenType::Plus),
           ';' => self.add_token(TokenType::Semicolon),
           '*' => self.add_token(TokenType::Star),
           '!' => self.add_token(TokenType::Bang),
           '=' => self.add_token(TokenType::Equal),
           '<' => self.add_token(TokenType::Less),
           '>' => self.add_token(TokenType::Greater),
           _ => println!("Unexpected character: {}", c)
       }
    }

    fn advance(self: &mut Self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        let text = self.source.chars().skip(self.start).take(self.current - self.start).collect::<String>();
        self.tokens.push(Token::new(token_type, text, LiteralValue::NilValue, self.line));
    }
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralValue,
    pub line: u32,
}

impl Token {
    
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i32),
    FloatValue(f64),
    BoolValue(bool),
    StringValue(String),
    NilValue,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace,
    RightBrace, Comma, Dot,
    Minus, Plus, Semicolon, Slash,
    Star, Bang,
    
    BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less,
    LessEqual, Identifier,
    
    String, Number, And, Class,
    Else, False, Fun, For,
    
    If, Nil, Or, Print,
    Return, Super, This,
    True, Var, While,
    
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
var test = 1;
var test2 = test + 1;
*/
