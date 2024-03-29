#[cfg(test)]
pub mod tests {
    use logos::Logos;
    use crate::scanner;
    #[test]
    fn parse_number() {
        let source = "2 + 2";
        let mut lexer = scanner::Token::lexer(source);
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number))); 
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Add)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number)));
    }

    #[test]
    fn parse_string() {
        let source = "\"sometext\"";
        let mut lexer = scanner::Token::lexer(source);
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::String)));
    }
    #[test]
    fn ar(){
        let source = "2 + 2 - 3 * 4 / 5";
        let mut lexer = scanner::Token::lexer(source);
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number))); 
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Add)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Subtract)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Multiply)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Divide)));
        assert_eq!(lexer.next(), Some(Ok(scanner::Token::Number)));
    }
}
