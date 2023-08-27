#![allow(unused)] // for now

use std::collections::HashSet;
use std::fmt;
use std::ops;

struct Value {
    data: i32,
    // _prev: HashSet<Value>,
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        Value {
            data: self.data + rhs.data,
            // _prev: Some(Box::new(self)),
        }
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
    let a = Value {data: 1};
    let b = Value {data: 2};
    let c = a + b;
    println!("c = {:?}", c);
}
