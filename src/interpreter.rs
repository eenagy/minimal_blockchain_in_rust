mod jump_table;

pub struct ScopeContext {
    memory: Memory,
    stack: Stack,
    contract: Contract,
}

pub struct EVM {} //TODO

pub struct EVMInterpreter<'a> {
    evm: &'a EVM,
    table: jump_table::JumpTable,

    hasher: crypto::KeccakState,
    hasher_buf: common::Hash,

    read_only: bool,
    return_data: Vec<u8>,
}

impl EVMInterpreter {
    fn new(evm: EVM) -> Self {
        let table: Option<jump_table::JumpTable> = jump_table::JumpTable::new();
        EVMInterpreter { evm, table }
    }

    // Run loops and evaluates the contract's code with the given input data and returns
    // the return byte-slice and an error if one occurred.
    //
    // It's important to note that any errors returned by the interpreter should be
    // considered a revert-and-consume-all-gas operation except for
    // ErrExecutionReverted which means revert-and-keep-gas-left.
    pub fn run(
        &mut self,
        contract: &Contract,
        input: &[u8],
        read_only: bool,
    ) -> Result<Vec<u8>, Error> {
        // Increment the call depth which is restricted to 1024
        self.evm.depth += 1;
        defer! { self.evm.depth -= 1 };

        // Make sure the read_only is only set if we aren't in read_only yet.
        // This also makes sure that the read_only flag isn't removed for child calls.
        if read_only && !self.read_only {
            self.read_only = true;
            defer! { self.read_only = false };
        }

        // Reset the previous call's return data. It's unimportant to preserve the old buffer
        // as every returning call will return new data anyway.
        self.return_data.clear();

        // Don't bother with the execution if there's no code.
        if contract.code.is_empty() {
            return Ok(Vec::new());
        }

        let mut pc = 0u64; // program counter
        let mut cost = 0u64;

        defer! { stack::return_stack(stack) };

        contract.input = input.to_vec();

        while let Some(op) = contract.get_op(pc) {
            let operation = self.table[op as usize];
            cost = operation.constant_gas;

            // Validate stack
            if let Err(stack_error) =
                validate_stack(stack.len(), operation.min_stack, operation.max_stack)
            {
                return Err(stack_error.into());
            }

            if !contract.use_gas(cost) {
                return Err(Error::OutOfGas);
            }

            if let Some(dynamic_gas) = operation.dynamic_gas {
                let memory_size = if let Some(memory_size_fn) = operation.memory_size {
                    let (mem_size, overflow) = memory_size_fn(stack)?;
                    if overflow {
                        return Err(Error::GasUintOverflow);
                    }
                    to_word_size(mem_size)
                        .checked_mul(32)
                        .ok_or(Error::GasUintOverflow)?
                } else {
                    0
                };

                let dynamic_cost =
                    dynamic_gas(self.evm, contract, stack, &mut self.memory, memory_size)?;
                cost += dynamic_cost;

                if !contract.use_gas(dynamic_cost) {
                    return Err(Error::OutOfGas);
                }

                if memory_size > 0 {
                    self.memory.resize(memory_size);
                }
            }
            let (result, execute_err) = operation.execute(&mut pc, self, call_context)?;

            if let Some(err) = execute_err {
                err.into()
            }

            pc += 1;
        }

        if err == Some(ErrStopToken) {
            err = None;
        }

        Ok(result)
    }
}
