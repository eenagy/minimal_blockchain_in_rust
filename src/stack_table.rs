const STACK_LIMIT: i32 = 100; // Assuming a constant stack limit

pub fn min_swap_stack(n: i32) -> i32 {
    min_stack(n, n)
}

pub fn max_swap_stack(n: i32) -> i32 {
    max_stack(n, n)
}

pub fn min_dup_stack(n: i32) -> i32 {
    min_stack(n, n + 1)
}
pub fn max_dup_stack(n: i32) -> i32 {
    max_stack(n, n + 1)
}

pub fn max_stack(pop: i32, push: i32) -> i32 {
    STACK_LIMIT + pop - push
}

pub fn min_stack(pops: i32, push: i32) -> i32 {
    pops
}
