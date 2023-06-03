mod eval;
mod grammar;
mod repl;

fn main() {
    println!("Welcome to Tinker!");
    loop {
        let command = repl::read_command();
        let result = eval::evaluate_program(command);
        println!("{}", result);
    }
}
