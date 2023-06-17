pub const GAS_QUICK_STEP: u64 = 2;
pub const GAS_FASTEST_STEP: u64 = 3;
pub const GAS_FAST_STEP: u64 = 5;
pub const GAS_MID_STEP: u64 = 8;
pub const GAS_SLOW_STEP: u64 = 10;
pub const GAS_EXT_STEP: u64 = 20;

// TODO change this
fn call_gas(
    is_eip150: bool,
    available_gas: u64,
    base: u64,
    call_cost: &Uint256,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut gas = available_gas;
    if is_eip150 {
        gas = gas.saturating_sub(base);
        gas = gas.saturating_sub(gas / 64);
        if call_cost.bits() > 64 || gas < call_cost.low_u64() {
            return Ok(gas);
        }
    }
    if !call_cost.is_u64() {
        return Err(Box::new(ErrGasUintOverflow));
    }
    Ok(call_cost.low_u64())
}
