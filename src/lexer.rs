#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64), StringLit(String), Ident(String),
    Let, Print, If, Else, While, True, False,
    Plus, Minus, Star, Slash, Equals, EqEq, Bang, NotEq, Less, Greater,
    LParen, RParen, LBrace, RBrace, Semicolon, Colon, EOF,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }
            '+' => { tokens.push(Token::Plus);      chars.next(); }
            '-' => { tokens.push(Token::Minus);     chars.next(); }
            '*' => { tokens.push(Token::Star);      chars.next(); }
            '/' => { tokens.push(Token::Slash);     chars.next(); }
            '(' => { tokens.push(Token::LParen);    chars.next(); }
            ')' => { tokens.push(Token::RParen);    chars.next(); }
            '{' => { tokens.push(Token::LBrace);    chars.next(); }
            '}' => { tokens.push(Token::RBrace);    chars.next(); }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            ':' => { tokens.push(Token::Colon);     chars.next(); }
            '<' => { tokens.push(Token::Less);      chars.next(); }
            '>' => { tokens.push(Token::Greater);   chars.next(); }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') { chars.next(); tokens.push(Token::EqEq); }
                else { tokens.push(Token::Equals); }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') { chars.next(); tokens.push(Token::NotEq); }
                else { tokens.push(Token::Bang); }
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' { num.push(d); chars.next(); } else { break; }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' { chars.next(); break; }
                    s.push(c); chars.next();
                }
                tokens.push(Token::StringLit(s));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' { word.push(c); chars.next(); } else { break; }
                }
                let token = match word.as_str() {
                    "let" => Token::Let, "print" => Token::Print,
                    "if"  => Token::If,  "else"  => Token::Else,
                    "while" => Token::While,
                    "true" => Token::True, "false" => Token::False,
                    _ => Token::Ident(word),
                };
                tokens.push(token);
            }
            _ => { chars.next(); }
        }
    }
    tokens.push(Token::EOF);
    tokens
}