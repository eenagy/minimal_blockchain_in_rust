fn memory_return(stack: &Stack) -> (u64, bool) {
    calc_mem_size_64(stack.back(0), stack.back(1))
}

fn memory_revert(stack: &Stack) -> (u64, bool) {
    calc_mem_size_64(stack.back(0), stack.back(1))
}

fn memory_log(stack: &Stack) -> (u64, bool) {
    calc_mem_size_64(stack.back(0), stack.back(1))
}

fn memory_call(stack: &Stack) -> (u64, bool) {
    let (x, overflow1) = calc_mem_size64(stack.back(5), stack.back(6));
    if overflow1 {
        return (0, true);
    }

    let (y, overflow2) = calc_mem_size64(stack.back(3), stack.back(4));
    if overflow2 {
        return (0, true);
    }

    if x > y {
        return (x, false);
    }

    (y, false)
}
