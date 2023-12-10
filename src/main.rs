mod parser;
mod semantics;

fn main() {
    let shell_ast = parser::generate_shell_ast("echo 22");    
}
