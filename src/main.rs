mod ast;
mod t_expression;
use ast::*;

fn main() {
    Ast::new().from_t_expression("./test.walm");
}
