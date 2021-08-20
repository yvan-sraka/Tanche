mod ast;
mod s_expression;
mod t_expression;
use ast::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
type Line = io::Lines<io::BufReader<File>>;

fn read_lines<P>(filename: P) -> io::Result<Line>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sample_t_expression() {
    let mut ast = Ast::new();
    let file_path = "test_p.walm";
    let lines = match read_lines(file_path) {
        Ok(lines) => lines,
        _ => panic!("Error reading file"),
    };
    ast.from_t_expression(lines);
    println!("{}", ast);
    ast.to_t_expression();
}

fn sample_s_expression() {
    let mut ast = Ast::new();
    let file_path = "test_s.walm";
    let lines = match read_lines(file_path) {
        Ok(lines) => lines,
        _ => panic!("Error reading file"),
    };
    ast.from_s_expression(lines);
    println!("{}", ast);
    ast.to_s_expression();
}

fn main() {
    sample_t_expression();
    sample_s_expression();
}
