mod gas;
mod gas_table;
mod gas_table;
mod instructions;
mod interpreter;
mod memory;
mod memory_table;
mod stack;
mod stack_table;

type ExecutionFunc = fn(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    call_context: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error>;

type GasFunc = fn(
    evm: &EVM,
    contract: &Contract,
    stack: &stack::Stack,
    memory: &stack::Memory,
    requested_memory_size: u64,
) -> Result<u64, Error>;

type MemorySizeFunc = fn(stack: &Stack) -> (u64, bool);

struct Operation {
    execute: ExecutionFunc,
    constant_gas: u64,
    dynamic_gas: GasFunc,
    min_stack: i32,
    max_stack: i32,
    memory_size: MemorySizeFunc,
}

pub type JumpTable = [Option<Operation>; 256];
fn validate(jt: JumpTable) -> JumpTable {
    for (i, op) in jt.iter().enumerate() {
        if op.is_none() {
            panic!(format!("op {:x} is not set", i));
        }
    }
    jt
}

fn deafult_instruction_set() -> JumpTable {
    let mut tbl: [Option<Operation>; 256] = [None; 256];
    tbl[OpCode::STOP as usize] = Some(Operation {
        execute: instructions::op_stop,
        constant_gas: 0,
        min_stack: stack_table::min_stack(0, 0),
        max_stack: stack_table::max_stack(0, 0),
        ..Default::default()
    });

    jump_table[OpCode::ADD as usize] = Some(Operation {
        execute: instructions::op_add,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::MUL as usize] = Some(Operation {
        execute: instructions::op_mul,
        constant_gas: gas::GasFastStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SUB as usize] = Some(Operation {
        execute: instructions::op_sub,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::DIV as usize] = Some(Operation {
        execute: instructions::op_div,
        constant_gas: gas::GasFastStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SDIV as usize] = Some(Operation {
        execute: instructions::opSdiv,
        constant_gas: gas::GasFastStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::MOD as usize] = Some(Operation {
        execute: instructions::op_mod,
        constant_gas: gas::GasFastStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SMOD as usize] = Some(Operation {
        execute: instructions::op_smod,
        constant_gas: gas::GasFastStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::ADDMOD as usize] = Some(Operation {
        execute: instructions::add_mod,
        constant_gas: gas::GasMidStep,
        min_stack: stack_table::min_stack(3, 1),
        max_stack: stack_table::max_stack(3, 1),
    });

    jump_table[OpCode::MULMOD as usize] = Some(Operation {
        execute: instructions::mul_mod,
        constant_gas: gas::GasMidStep,
        min_stack: stack_table::min_stack(3, 1),
        max_stack: stack_table::max_stack(3, 1),
    });

    jump_table[OpCode::EXP as usize] = Some(Operation {
        execute: instructions::op_exp,
        constant_gas: gas::GasExpFrontier,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SIGNEXTEND as usize] = Some(Operation {
        execute: instructions::op_sign_extend,
        constant_gas: gas::GasMidStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::LT as usize] = Some(Operation {
        execute: instructions::op_lt,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::GT as usize] = Some(Operation {
        execute: instructions::op_gt,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SLT as usize] = Some(Operation {
        execute: instructions::op_slt,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::SGT as usize] = Some(Operation {
        execute: instructions::op_sgt,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::EQ as usize] = Some(Operation {
        execute: instructions::op_eq,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });

    jump_table[OpCode::ISZERO as usize] = Some(Operation {
        execute: instructions::op_iszero,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });

    jump_table[OpCode::AND as usize] = Some(Operation {
        execute: instructions::op_and,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });
    jump_table[OpCode::XOR as usize] = Some(Operation {
        execute: instructions::op_xor,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });
    jump_table[OpCode::OR as usize] = Some(Operation {
        execute: instructions::op_or,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });
    jump_table[OpCode::NOT as usize] = Some(Operation {
        execute: instructions::op_not,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    jump_table[OpCode::BYTE as usize] = Some(Operation {
        execute: instructions::op_byte,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
    });
    jump_table[OpCode::KECCAK256 as usize] = Some(Operation {
        execute: instructions::op_keccak256,
        constant_gas: params.Keccak256Gas,
        dynamic_gas: gasKeccak256,
        min_stack: stack_table::min_stack(2, 1),
        max_stack: stack_table::max_stack(2, 1),
        memory_size: memoryKeccak256,
    });

    jump_table[OpCode: ADDRESS as usize] = Some(Operation {
        execute: instructions::op_address,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });

    jump_table[OpCode: BALANCE as usize] = Some(Operation {
        execute: instructions::op_balance,
        constant_gas: params.BalanceGasFrontier,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    jump_table[OpCode: ORIGIN as usize] = Some(Operation {
        execute: instructions::op_origin,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CALLER as usize] = Some(Operation {
        execute: instructions::op_caller,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CALLVALUE as usize] = Some(Operation {
        execute: instructions::op_call_value,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CALLDATALOAD as usize] = Some(Operation {
        execute: instructions::op_call_data_load,
        constant_gas: gas::GasFastestStep,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    jump_table[OpCode: CALLDATASIZE as usize] = Some(Operation {
        execute: instructions::op_call_data_size,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CALLDATACOPY as usize] = Some(Operation {
        execute: instructions::op_call_data_copy,
        constant_gas: gas::GasFastestStep,
        dynamic_gas: gasCallDataCopy,
        min_stack: stack_table::min_stack(3, 0),
        max_stack: stack_table::max_stack(3, 0),
        memory_size: memoryCallDataCopy,
    });
    jump_table[OpCode: CODESIZE as usize] = Some(Operation {
        execute: instructions::op_code_size,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CODECOPY as usize] = Some(Operation {
        execute: instructions::op_code_copy,
        constant_gas: gas::GasFastestStep,
        dynamic_gas: gasCodeCopy,
        min_stack: stack_table::min_stack(3, 0),
        max_stack: stack_table::max_stack(3, 0),
        memory_size: memoryCodeCopy,
    });
    jump_table[OpCode: GASPRICE as usize] = Some(Operation {
        execute: instructions::op_gasprice,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: CODESIZE as usize] = Some(Operation {
        execute: instructions::op_code_size,
        constant_gas: params.ExtcodeSizeGasFrontier,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    // duplicate
    jump_table[OpCode: CODECOPY as usize] = Some(Operation {
        execute: instructions::op_code_copy,
        constant_gas: params.ExtcodeCopyBaseFrontier,
        dynamic_gas: gasExtCodeCopy,
        min_stack: stack_table::min_stack(4, 0),
        max_stack: stack_table::max_stack(4, 0),
        memory_size: memoryExtCodeCopy,
    });
    jump_table[OpCode: BLOCKHASH as usize] = Some(Operation {
        execute: instructions::op_blockhash,
        constant_gas: gas::GasExtStep,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    jump_table[OpCode: COINBASE as usize] = Some(Operation {
        execute: instructions::coinbase,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: TIMESTAMP as usize] = Some(Operation {
        execute: instructions::op_timestamp,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: NUMBER as usize] = Some(Operation {
        execute: instructions::op_number,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: DIFFICULTY as usize] = Some(Operation {
        execute: instructions::op_difficulty,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: GASLIMIT as usize] = Some(Operation {
        execute: instructions::op_gaslimit,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: POP as usize] = Some(Operation {
        execute: instructions::op_pop,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(1, 0),
        max_stack: stack_table::max_stack(1, 0),
    });
    jump_table[OpCode: MLOAD as usize] = Some(Operation {
        execute: instructions::op_mload,
        constant_gas: gas::GasFastestStep,
        dynamic_gas: gasMLoad,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
        memory_size: memoryMLoad,
    });
    jump_table[OpCode: MSTORE as usize] = Some(Operation {
        execute: instructions::op_mstore,
        constant_gas: gas::GasFastestStep,
        dynamic_gas: gasMStore,
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
        memory_size: memoryMStore,
    });
    jump_table[OpCode: MSTORE8 as usize] = Some(Operation {
        execute: instructions::op_mstore8,
        constant_gas: gas::GasFastestStep,
        dynamic_gas: gasMStore8,
        memory_size: memoryMStore8,
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
    });
    jump_table[OpCode: SLOAD as usize] = Some(Operation {
        execute: instructions::op_sload,
        constant_gas: params.SloadGasFrontier,
        min_stack: stack_table::min_stack(1, 1),
        max_stack: stack_table::max_stack(1, 1),
    });
    jump_table[OpCode: SSTORE as usize] = Some(Operation {
        execute: instructions::op_sstore,
        dynamic_gas: gasSStore,
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
    });
    jump_table[OpCode: JUMP as usize] = Some(Operation {
        execute: instructions::op_jump,
        constant_gas: gas::GasMidStep,
        min_stack: stack_table::min_stack(1, 0),
        max_stack: stack_table::max_stack(1, 0),
    });
    jump_table[OpCode: JUMPI as usize] = Some(Operation {
        execute: instructions::op_jumpi,
        constant_gas: gas::GasSlowStep,
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
    });
    jump_table[OpCode: PC as usize] = Some(Operation {
        execute: instructions::op_pc,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: MSIZE as usize] = Some(Operation {
        execute: instructions::op_msize,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: GAS as usize] = Some(Operation {
        execute: instructions::op_gas,
        constant_gas: gas::GasQuickStep,
        min_stack: stack_table::min_stack(0, 1),
        max_stack: stack_table::max_stack(0, 1),
    });
    jump_table[OpCode: JUMPDEST as usize] = Some(Operation {
        execute: instructions::op_jumpdest,
        constant_gas: params.JumpdestGas,
        min_stack: stack_table::min_stack(0, 0),
        max_stack: stack_table::max_stack(0, 0),
    });
    let push_opcodes = [
        Opcode::PUSH1,
        Opcode::PUSH2,
        Opcode::PUSH3,
        Opcode::PUSH4,
        Opcode::PUSH5,
        Opcode::PUSH6,
        Opcode::PUSH7,
        Opcode::PUSH8,
        Opcode::PUSH9,
        Opcode::PUSH10,
        Opcode::PUSH11,
        Opcode::PUSH12,
        Opcode::PUSH13,
        Opcode::PUSH14,
        Opcode::PUSH15,
        Opcode::PUSH16,
        Opcode::PUSH17,
        Opcode::PUSH18,
        Opcode::PUSH19,
        Opcode::PUSH20,
        Opcode::PUSH21,
        Opcode::PUSH22,
        Opcode::PUSH23,
        Opcode::PUSH24,
        Opcode::PUSH25,
        Opcode::PUSH26,
        Opcode::PUSH27,
        Opcode::PUSH28,
        Opcode::PUSH29,
        Opcode::PUSH30,
        Opcode::PUSH31,
        Opcode::PUSH32,
    ];

    for (index, opcode) in push_opcodes.iter().enumerate().take(32) {
        jump_table[opcode] = Some(Operation {
            execute: instructions::make_push(index + 1, index + 1),
            constant_gas: gas::GasFastestStep,
            min_stack: stack_table::min_stack(0, 1),
            max_stack: stack_table::max_stack(0, 1),
        });
    }
    let dup_opcodes = [
        Opcode::DUP1,
        Opcode::DUP2,
        Opcode::DUP3,
        Opcode::DUP4,
        Opcode::DUP5,
        Opcode::DUP6,
        Opcode::DUP7,
        Opcode::DUP8,
        Opcode::DUP8,
        Opcode::DUP9,
        Opcode::DUP10,
        Opcode::DUP11,
        Opcode::DUP12,
        Opcode::DUP13,
        Opcode::DUP14,
        Opcode::DUP15,
        Opcode::DUP16,
    ];
    for (index, opcode) in dup_opcodes.iter().enumerate().take(16) {
        jump_table[opcode] = Some(Operation {
            execute: instructions::make_dup(index + 1),
            constant_gas: gas::GasFastestStep,
            min_stack: stack_table::minDupStack(index + 1),
            max_stack: stack_table::maxDupStack(index + 1),
        });
    }
    let swap_opcodes = [
        Opcode::SWAP1,
        Opcode::SWAP2,
        Opcode::SWAP3,
        Opcode::SWAP4,
        Opcode::SWAP5,
        Opcode::SWAP6,
        Opcode::SWAP7,
        Opcode::SWAP8,
        Opcode::SWAP8,
        Opcode::SWAP9,
        Opcode::SWAP10,
        Opcode::SWAP11,
        Opcode::SWAP12,
        Opcode::SWAP13,
        Opcode::SWAP14,
        Opcode::SWAP15,
        Opcode::SWAP16,
    ];
    for (index, opcode) in swap_opcodes.iter().enumerate().take(16) {
        jump_table[opcode] = Some(Operation {
            execute: instructions::make_swap(index + 1),
            constant_gas: gas::GasFastestStep,
            min_stack: stack_table::minSwapStack(index + 1),
            max_stack: stack_table::maxSwapStack(index + 1),
        });
    }
    jump_table[OpCode: LOG0 as usize] = Some(Operation {
        execute: instructions::makeLog(0),
        dynamic_gas: instructions::makeGasLog(0),
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
        memory_size: memoryLog,
    });
    jump_table[OpCode: LOG1 as usize] = Some(Operation {
        execute: instructions::makeLog(1),
        dynamic_gas: instructions::makeGasLog(1),
        min_stack: stack_table::min_stack(3, 0),
        max_stack: stack_table::max_stack(3, 0),
        memory_size: memoryLog,
    });
    jump_table[OpCode: LOG2 as usize] = Some(Operation {
        execute: instructions::makeLog(2),
        dynamic_gas: instructions::makeGasLog(2),
        min_stack: stack_table::min_stack(4, 0),
        max_stack: stack_table::max_stack(4, 0),
        memory_size: memoryLog,
    });
    jump_table[OpCode: LOG3 as usize] = Some(Operation {
        execute: instructions::makeLog(3),
        dynamic_gas: instructions::makeGasLog(3),
        min_stack: stack_table::min_stack(5, 0),
        max_stack: stack_table::max_stack(5, 0),
        memory_size: memoryLog,
    });
    jump_table[OpCode: LOG4 as usize] = Some(Operation {
        execute: instructions::makeLog(4),
        dynamic_gas: instructions::makeGasLog(4),
        min_stack: stack_table::min_stack(6, 0),
        max_stack: stack_table::max_stack(6, 0),
        memory_size: memoryLog,
    });
    jump_table[OpCode: CREATE as usize] = Some(Operation {
        execute: instructions::op_create,
        constant_gas: params.CreateGas,
        dynamic_gas: gasCreate,
        min_stack: stack_table::min_stack(3, 1),
        max_stack: stack_table::max_stack(3, 1),
        memory_size: memoryCreate,
    });
    jump_table[OpCode: CALL as usize] = Some(Operation {
        execute: instructions::op_call,
        constant_gas: params.CallGasFrontier,
        dynamic_gas: gasCall,
        min_stack: stack_table::min_stack(7, 1),
        max_stack: stack_table::max_stack(7, 1),
        memory_size: memoryCall,
    });
    jump_table[OpCode: CALLCODE as usize] = Some(Operation {
        execute: instructions::op_call_code,
        constant_gas: params.CallGasFrontier,
        dynamic_gas: gasCallCode,
        min_stack: stack_table::min_stack(7, 1),
        max_stack: stack_table::max_stack(7, 1),
        memory_size: memory_table::memory_call,
    });
    jump_table[OpCode: RETURN as usize] = Some(Operation {
        execute: instructions::op_return,
        dynamic_gas: gas_table::gas_return,
        min_stack: stack_table::min_stack(2, 0),
        max_stack: stack_table::max_stack(2, 0),
        memory_size: memory_table::memory_return,
    });
    jump_table[OpCode: SELFDESTRUCT as usize] = Some(Operation {
        execute: instructions::op_selfdestruct,
        dynamic_gas: gas_table::gasSelfdestruct,
        min_stack: stack_table::min_stack(1, 0),
        max_stack: stack_table::max_stack(1, 0),
    });

    // Fill all unassigned slots with opUndefined.
    for (i, entry) in tbl.iter_mut().enumerate() {
        if entry.is_none() {
            *entry = Some(Operation {
                execute: instructions::op_undefined,
                max_stack: stack_table::max_stack(0, 0),
            });
        }
    }

    validate(tbl)
}

type JumpTable = [Option<ExecutionFunc>; OpCode::OpCodeCount];

impl JumpTable {
    fn new() -> Self {
        deafult_instruction_set()
    }
}
