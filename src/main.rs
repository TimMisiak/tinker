mod eval;
mod grammar;
mod repl;
mod ast;

fn main() {
    println!("Welcome to Tinker!");
    loop {
        let parsed = repl::read_command();
        println!("Parse tree: {:?}", parsed);
        let prog = ast::ast_from_parse_tree(parsed);
        println!("Abstract syntax tree: {:?}", prog);
        let result = eval::evaluate_program(prog);
        println!("Result: {}", result);
    }
}
