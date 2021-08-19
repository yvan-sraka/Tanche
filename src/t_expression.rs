use std::collections::HashMap;
use super::ast::*;
use std::cmp::Ordering;
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

fn trim(line: &mut String) -> usize {
    let l1 = line.len();
    let trimmed: &str = line.trim_start();
    let trim_start = trimmed.as_ptr() as usize - line.as_ptr() as usize;
    let trim_len = trimmed.len();
    if trim_start != 0 {
        line.drain(..trim_start);
    }
    line.truncate(trim_len);
    l1 - line.len()
}

impl Ast {
    pub fn from_t_expression(&mut self, file_path: &str) {
        // File hosts must exist in current path before this produces output
        let lines = read_lines(file_path);
        if lines.is_err() {
            panic!("Error reading file");
        }
        let lines = lines.unwrap();
        let mut indent = 0;
        let mut lvl = 0;
        for res in lines {
            if let Ok(mut line) = res {
                let spaces = trim(&mut line);
                if line.is_empty() {
                    continue;
                }
                if indent == 0 && spaces != 0 {
                    indent = spaces;
                    lvl = 1;
                    self.increment();
                } else if indent != 0 && spaces % indent != 0 {
                    panic!("Invalid indentation detected");
                } else if indent != 0 {
                    let new_lvl = spaces / indent;
                    match new_lvl.cmp(&lvl) {
                        Ordering::Less => self.decrement(),
                        Ordering::Greater => self.increment(),
                        _ => {}
                    }
                    lvl = new_lvl;
                }
                self.insert(&line);
            } else {
                panic!("Error reading line")
            }
        }
    }
    pub fn to_t_expression(&self) {
        let mut increments: HashMap<usize, usize> = HashMap::new();
        increments.insert(0, 0);
        for edge in self.edges.iter() {
            let inc = increments[&edge.0] + 1;
            let spaces = (0..(4 * inc) - 4).map(|_| " ").collect::<String>();
            increments.insert(edge.1, inc);
            println!("{}{}", spaces, self.nodes[&edge.1]);
        }
    }
}
