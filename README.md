# S/T-Expression language reading

Create a graph containing a language T-expressed or s-expressed.

```rust
mod ast;
mod s_expression;
mod t_expression;
use ast::*;

fn sample_t_expression() {
    let mut ast = Ast::new();
    ast.from_t_expression(lines_iter_in("t_expression.wlan"));
    println!("{}", ast);
    println!("{}", ast.to_t_expression());
}

fn sample_s_expression() {
    let mut ast = Ast::new();
    ast.from_s_expression(lines_iter_in("s_expression.wlan"));
    println!("{}", ast);
    println!("{}", ast.to_s_expression());
}
```

In this example:
- `Ast::new()` create our graph
- `lines_iter_in("s_expression.wlan")` return an iterator generating each lines of the given file
- The functions `from_*_expression` take an iterator of text content as input.
- The functions `from_*_expression` directly return a text String.