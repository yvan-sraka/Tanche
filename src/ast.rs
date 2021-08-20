use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

pub struct Ast {
    curr_index: usize,
    pub nodes: HashMap<usize, String>,
    pub edges: Vec<(usize, usize)>,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nodes: ({:?}\nedges: {:?})", self.nodes, self.edges)
    }
}

impl Ast {
    pub fn decrement(&mut self) {
        for edge in self.edges.iter() {
            if edge.1 == self.curr_index {
                self.curr_index = edge.0;
            }
        }
        self.curr_index = 0;
    }
    pub fn increment(&mut self) {
        self.curr_index = self.edges.last().unwrap_or(&(0, 0)).1;
    }
    pub fn insert(&mut self, val: &str) {
        let i = self.nodes.len() + 1;
        self.nodes.insert(i, val.to_owned());
        self.edges.push((self.curr_index, i));
    }
    pub fn nexts(&self, index: usize) -> Vec::<usize> {
        let mut ret = Vec::new();
        for edge in self.edges.iter() {
            if edge.0 == index {
                ret.push(edge.1);
            }
        }
        ret
    }
    pub fn new() -> Ast {
        Ast {
            nodes: HashMap::new(),
            edges: Vec::new(),
            curr_index: 0,
        }
    }
}
