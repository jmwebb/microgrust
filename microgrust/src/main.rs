#![allow(unused)] // for now

use std::collections::HashSet;
use std::fmt;
use std::ops;

struct Value {
    data: f32,
    _prev: Vec<Value>,
}

impl Value {
    fn new(data: f32) -> Self {
        Value {
            data,
            _prev: Vec::new(),
        }
    }
    
}

// ╔══════════════════════════╗
// ║        Addition          ║
// ╚══════════════════════════╝
impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        Value {
            data: self.data + rhs.data,
            _prev: vec![self, rhs],
        }
    }
}

impl ops::Add<f32> for Value {
    type Output = Value;

    fn add(self, rhs: f32) -> Value {
        let rhs = Value::new(rhs);
        self + rhs
    }
}

impl ops::Add<Value> for f32 {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        rhs + self
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

fn main() {
    let b = Value::new(1.0);
    let a = 2.0;
    let c = a + b;
    println!("c = {:?}", c);
    println!("c = {:?}", c._prev);
}
