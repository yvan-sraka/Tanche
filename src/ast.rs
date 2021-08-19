use std::collections::HashMap;
use std::vec::Vec;

pub struct Ast {
    nodes: HashMap<usize, String>,
    edges: Vec<(usize, usize)>,
}

impl Ast {
    pub fn decrement(&self) {
        println!("decrement")
    }
    pub fn increment(&self) {
        println!("increment")
    }
    pub fn insert(&self, val: &str) {
        println!("push {}", val);
    }
    pub fn new() -> Ast {
        Ast {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
}
