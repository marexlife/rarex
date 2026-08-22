mod lex;

fn main() {
    let source_code = String::new();
    let tokens = lex::Lexer::new(source_code).lex();

    for token in tokens {
        
    }
}
