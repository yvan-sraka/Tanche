use super::ast::*;
use std::cmp::Ordering;
use std::collections::HashMap;

use std::fs::File;
type Line = std::io::Lines<std::io::BufReader<File>>;

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
    pub fn from_t_expression(&mut self, lines: Line) {
        let mut indent = 0;
        let mut lvl = 0;
        for mut line in lines.flatten() {
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
                    Ordering::Less => {
                        println!("decrement by {}", lvl - new_lvl);
                        for _ in 0..(lvl - new_lvl) {
                            self.decrement();
                        }
                    },
                    Ordering::Greater => self.increment(),
                    _ => {}
                }
                lvl = new_lvl;
            }
            self.insert(&line);
        }
    }
    pub fn to_t_expression(&self) -> String {
        let mut ret = String::new();
        let mut increments: HashMap<usize, usize> = HashMap::new();
        increments.insert(0, 0);
        for edge in self.edges.iter() {
            let inc = increments[&edge.0] + 1;
            let spaces = (0..(4 * inc) - 4).map(|_| " ").collect::<String>();
            increments.insert(edge.1, inc);
            ret.push_str(&spaces);
            ret.push_str(&self.nodes[&edge.1]);
            ret.push('\n');
        }
        ret
    }
}
