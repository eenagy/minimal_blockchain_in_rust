use std::sync::{Arc, Mutex};
use uint256::Uint256;

struct Stack {
    data: Vec<Uint256>,
}

impl Stack {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, d: Uint256) {
        self.data.push(d);
    }

    fn pop(&mut self) -> Option<Uint256> {
        self.data.pop()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn data(&self) -> &[Uint256] {
        &self.data
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn swap(&mut self, n: usize) {
        let len = self.data.len();
        self.data.swap(len - n, len - 1);
    }

    fn dup(&mut self, n: usize) {
        let value = self.data[len - n].clone();
        self.push(value);
    }

    fn peek(&self) -> Option<&Uint256> {
        self.data.last()
    }

    fn back(&self, n: usize) -> Option<&Uint256> {
        let len = self.data.len();
        if n < len {
            self.data.get(len - n - 1)
        } else {
            None
        }
    }
}

lazy_static::lazy_static! {
    static ref STACK_POOL: Mutex<parking_lot::Mutex<Option<Stack>>> = Mutex::new(parking_lot::Mutex::new(None));
}

fn new_stack() -> Stack {
    let mut stack_pool = STACK_POOL.lock().unwrap();
    if let Some(stack) = stack_pool.take() {
        stack
    } else {
        Stack {
            data: Vec::with_capacity(16),
        }
    }
}

fn return_stack(mut stack: Stack) {
    stack.data.clear();
    let mut stack_pool = STACK_POOL.lock().unwrap();
    *stack_pool = Some(stack);
}
