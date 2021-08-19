mod ast;
mod t_expression;
use ast::*;

fn main() {
    let mut ast = Ast::new();
    ast.from_t_expression("./test.walm");
    println!("{}", ast);
    ast.to_t_expression();
}
