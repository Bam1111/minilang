mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::tokenize;
use parser::Parser;
use evaluator::Interpreter;

fn main() {
    let source = r#"
        let x: int = 10;
        let y: int = 5;
        print(x + y);
        print(x * y);
        if x > y {
            print("x is greater");
        } else {
            print("y is greater");
        }
        let count: int = 0;
        while count < 3 {
            print(count);
            count = count + 1;
        }
    "#;

    let tokens = tokenize(source);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    let mut interpreter = Interpreter::new();
    interpreter.run(&ast);
}
