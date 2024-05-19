use aiyokona::lexer::*;

#[test]
fn test_single_char_tokens() {
    let input = "( ) + - * / ";
    let test_case: Vec<Token> = vec![
        Token::LParen,
        Token::RParen,
        Token::Plus,
        Token::Minus,
        Token::Multiply,
        Token::Divide,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input);
    for token in test_case {
        if let Some(lexer_token) = lexer.next() {
            assert_eq!(lexer_token, token);
        }
    }
}

#[test]
fn test_multi_char_tokens() {
    let input = "let yep print 10";

    let test_case = vec![
        Token::Let,
        Token::Ident("yep".into()),
        Token::PrintMethod,
        Token::Num(10),
    ];

    let mut lexer = Lexer::new(input);
    for test_case_token in test_case {
        if let Some(lexer_token) = lexer.next() {
            assert_eq!(lexer_token, test_case_token);
        }
    }
}

#[test]
pub fn test_lexer() {
    let input = "
            (let ((x 10)
                 (y 6)
                 (res (+ x y)))
             print res)";

    let test_cases: Vec<Token> = vec![
        Token::LParen,
        Token::Let,
        Token::LParen,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Num(10),
        Token::RParen,
        Token::LParen,
        Token::Ident(String::from("y")),
        Token::Num(6),
        Token::RParen,
        Token::LParen,
        Token::Ident(String::from("res")),
        Token::LParen,
        Token::Plus,
        Token::Ident(String::from("x")),
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::RParen,
        Token::RParen,
        Token::PrintMethod,
        Token::Ident(String::from("res")),
        Token::RParen,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input);
    for test in test_cases {
        match lexer.next() {
            Some(token) => assert_eq!(token, test),
            _ => (),
        }
    }
}

#[test]
fn test_lexer_with_negative_value() {
    let test_cases = vec![
        Token::LParen,
        Token::Let,
        Token::LParen,
        Token::LParen,
        Token::Ident("n".to_string()),
        Token::Num(-2),
        Token::RParen,
        Token::RParen,
        Token::PrintMethod,
        Token::Ident("n".to_string()),
        Token::RParen,
        Token::Eof,
    ];

    let input = "(let ((n -2)) print n)";
    let mut lexer = Lexer::new(input);

    for test in test_cases {
        if let Some(token) = lexer.next() {
            assert_eq!(token, test);
        }
    }
}
