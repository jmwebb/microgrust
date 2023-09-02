#![allow(unused)] // for now

use std::fmt;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops;
use std::rc::Rc;

// enum BackwardFn {
//     Default,
//     Add(Box<dyn Fn(&mut Value, &mut Value, f32)>),
// }

pub struct Value {
    data: f32,
    grad: f32,
    // _prev: Vec<usize>,
    // _prev: Vec<Rc<RefCell<Value>>>,
    _prev: Vec<Value>,
    _backward: fn(),
    // _backward: BackwardFn,
    // _offset: usize,
}

// static mut DEPENDENCY_GRAPH: Vec<Value> = Vec::new();

impl Value {
    // fn new(data: f32) -> Rc<RefCell<Self>> {
        // Rc::new(RefCell::new(Value {
        //     data,
        //     grad: 0.0,
        //     _prev: Vec::new(),
        //     // _backward: BackwardFn::Default,
        //     // _offset: DEPENDENCY_GRAPH.len(),
        // }))
        // let out  = Value {
        //     data,
        //     grad: 0.0,
        //     _backward: BackwardFn::Default,
        //     _prev: Vec::new(),
        //     _offset: DEPENDENCY_GRAPH.len(),
        // };
        // unsafe {
        //     DEPENDENCY_GRAPH.push(out);
        // }
        // out
    // }
    // fn default_backward() {
        
    // }

    fn new(data: f32) -> Self {
        Value {
            data,
            grad: 0.0,
            _prev: Vec::new(),
            _backward: || {},
        }
    }

    fn add_backward(&mut self) {
        self._prev[0].grad += self.grad;
        self._prev[1].grad += self.grad;
    }


    // ╔══════════════════════════╗
    // ║        Backward          ║
    // ╚══════════════════════════╝
    // fn backward(&mut self) {
    //     match &mut self._backward {
    //         BackwardFn::Add(f) => {
    //             let mut self_ = DEPENDENCY_GRAPH[self._prev[0]];
    //             let mut rhs_ = DEPENDENCY_GRAPH[self._prev[1]];
    //             f(self_, rhs_, self.grad); 
    //         }
    //     }
    // }
} 



// ╔══════════════════════════╗
// ║        Addition          ║
// ╚══════════════════════════╝
// impl ops::Add<Rc<RefCell<Value>>> for Value {
//     type Output = Rc<RefCell<Value>>;

    // fn add(self, rhs: Rc<RefCell<Value>>) -> Rc<RefCell<Value>> {
    //     let mut result = Value::new(self.borrow().data + rhs.borrow().data);
        // Note: Because we are using Rc<RefCell<Value>> 
        // we are cloning a pointer to self and rhs
        // and not the actual Values
        // result._prev.push(self.clone());
        // result._prev.push(rhs.clone());

        // result._backward = BackwardFn::Add(Box::new(|self_, rhs_, grad| {
        //     self_.grad += grad;
        //     rhs_.grad += grad;
        // }));

        // result
    // }
// }
impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        let mut out = Value::new(self.data + rhs.data);
        out._prev.push(self);
        out._prev.push(rhs);
        out._backward = Value::add_backward;
        out
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

// ╔══════════════════════════╗
// ║          Display         ║
// ╚══════════════════════════╝
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
    let a = 1.0;
    let b = Value::new(2.0);
    let c = a + b;
    println!("c = {:?}", c);
    println!("c = {:?}", c._prev);
}
