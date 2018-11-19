// use cranelift;

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/huaji.rs"));
}

#[derive(Debug)]
pub enum Expr {
    Assign(String, Box<Expr>),
    Literal(String),
}

fn main() {
    loop {
        let mut code = String::new();
        std::io::stdin().read_line(&mut code).unwrap();
        let code = code.trim().to_string();
        let result = parser::statements(&code);
        match result {
            Ok(expr) => println!("Result: {:?}", expr),
            Err(err) => compile_error(code.clone(), err),
        }
    }
}

fn compile_error(code: String, parser::ParseError { line, column, offset, expected }: parser::ParseError) {
    eprintln!("at line {}, column {}: ", line, column);
    eprintln!("error: parse error");
    eprintln!("{}   | {}", line, code.lines().nth(line - 1).unwrap());
    eprint!(  "    - ");
    for _ in 0..offset {
        eprint!(" ");
    }
    eprintln!("^");
    eprintln!("=> note: {:?} expected", expected);
}