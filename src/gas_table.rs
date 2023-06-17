fn pure_memory_gas_cost(
    evm: &EVM,
    contract: &Contract,
    stack: &Stack,
    mem: &Memory,
    memory_size: u64,
) -> Result<u64, Error> {
    memory_gas_cost(mem, memory_size)
}
type GasCostFunc = fn(evm: &EVM, contract: &Contract, stack: &Stack, mem: &Memory, memorySize: u64) -> Result<u64, Error>;

pub const gas_return: GasCostFunc = pure_memory_gas_cost

pub fn gas_static_call(
    evm: &mut EVM,
    contract: &Contract,
    stack: &Stack,
    mem: &Memory,
    memory_size: u64,
) -> Result<u64, Error> {
    let gas = memory_gas_cost(mem, memory_size)?;
    evm.call_gas_temp = call_gas(evm.chain_rules.is_eip150, contract.gas, gas, stack.back(0))?;
    let (gas, overflow) = math::safe_add(gas, evm.call_gas_temp);
    if overflow {
        return Err(Error::GasUintOverflow);
    }
    Ok(gas)
}

pub fn gas_selfdestruct(
    evm: &mut EVM,
    contract: &Contract,
    stack: &Stack,
    mem: &Memory,
    memory_size: u64,
) -> Result<u64, Error> {
    let mut gas: u64 = 0;
    if evm.chain_rules.is_eip150 {
        gas = params::SELFDESTRUCT_GAS_EIP150;
        let address = common::Address(stack.back(0).bytes20());

        if evm.chain_rules.is_eip158 {
            if evm.state_db.empty(address)
                && evm.state_db.get_balance(contract.address()).sign() != 0
            {
                gas += params::CREATE_BY_SELFDESTRUCT_GAS;
            }
        } else if !evm.state_db.exist(address) {
            gas += params::CREATE_BY_SELFDESTRUCT_GAS;
        }
    }

    if !evm.state_db.has_suicided(contract.address()) {
        evm.state_db.add_refund(params::SELFDESTRUCT_REFUND_GAS);
    }
    Ok(gas)
}
