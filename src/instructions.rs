mod interpreter;

pub fn op_add(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(result) = y.checked_add(&x) {
        *y = result;
        Ok(Vec::new())
    } else {
        Err(Error::Overflow)
    }
}

pub fn op_sub(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(y_value), Some(x_value)) = (y, x) {
        *y = y_value.sub(x_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_mul(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(y_value), Some(x_value)) = (y, x) {
        *y = y_value.mul(x_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_div(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(y_value), Some(x_value)) = (y, x) {
        if x_value.is_zero() {
            return Err(Error::DivisionByZero);
        }

        *y = y_value.div(x_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_mod(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(y_value), Some(x_value)) = (y, x) {
        *y = y_value.modulo(x_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_smod(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(y_value), Some(x_value)) = (y, x) {
        *y = y_value.signed_modulo(x_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_exp(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let base = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let exponent = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(exponent_value), Some(base_value)) = (exponent, base) {
        *exponent = exponent_value.pow(base_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_sign_extend(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let back = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let num = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(num_value), Some(back_value)) = (num, back) {
        num_value.extend_sign(&back_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_not(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(x_value) = x {
        x_value.not();
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_lt(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        if x_value.lt(&y_value) {
            y_value.set_one();
        } else {
            y_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_gt(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        if x_value.gt(&y_value) {
            y_value.set_one();
        } else {
            y_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_slt(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        if x_value.slt(&y_value) {
            y_value.set_one();
        } else {
            y_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_sgt(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        if x_value.sgt(&y_value) {
            y_value.set_one();
        } else {
            y_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_eq(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        if x_value.eq(y_value) {
            y_value.set_one();
        } else {
            y_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_iszero(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(x_value) = x {
        if x_value.is_zero() {
            x_value.set_one();
        } else {
            x_value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_and(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        y_value.and(&x_value, y_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_or(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        y_value.or(&x_value, y_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_xor(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value)) = (x, y) {
        y_value.xor(&x_value, y_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_byte(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let th = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let val = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(th_value), Some(val_value)) = (th, val) {
        val_value.byte(&th_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_addmod(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let z = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value), Some(z_value)) = (x, y, z) {
        if z_value.is_zero() {
            z_value.clear();
        } else {
            z_value.add_mod(&x_value, &y_value, z_value);
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_mulmod(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let y = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let z = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let (Some(x_value), Some(y_value), Some(z_value)) = (x, y, z) {
        z_value.mul_mod(&x_value, &y_value, z_value);
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_shl(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let shift = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let value = match scope.stack.peek_mut() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(shift_value) = shift {
        if shift_value.lt_uint64(256) {
            value.lsh(value, shift_value.uint64() as usize);
        } else {
            value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_shr(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let shift = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let value = match scope.stack.peek_mut() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(shift_value) = shift {
        if shift_value.lt_uint64(256) {
            value.rsh(value, shift_value.uint64() as usize);
        } else {
            value.clear();
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_sar(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let shift = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let value = match scope.stack.peek_mut() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(shift_value) = shift {
        if shift_value.gt_uint64(256) {
            if value.sign() >= 0 {
                value.clear();
            } else {
                value.set_all_one();
            }
        } else {
            let n = shift_value.uint64() as usize;
            value.srsh(value, n);
        }
        Ok(Vec::new())
    } else {
        Err(Error::MissingOperand)
    }
}

pub fn op_keccak256(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let size = match scope.stack.peek_mut() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let data = scope
        .memory
        .get_ptr(offset.uint64() as i64, size.uint64() as i64);

    if interpreter.hasher.is_none() {
        interpreter.hasher = Some(crypto::new_keccak_state());
    } else {
        interpreter.hasher.as_mut().unwrap().reset();
    }

    interpreter.hasher.as_mut().unwrap().write(data);
    interpreter
        .hasher
        .as_mut()
        .unwrap()
        .read(&mut interpreter.hasher_buf);

    let evm = interpreter.evm;
    if evm.config.enable_preimage_recording {
        evm.state_db.add_preimage(&interpreter.hasher_buf, data);
    }

    size.set_bytes(&interpreter.hasher_buf);
    Ok(Vec::new())
}

pub fn op_address(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let address = Address::from(scope.contract.address().bytes());
    scope.stack.push(Uint256::from_bytes(&address));
    Ok(Vec::new())
}

pub fn op_balance(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let slot = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let address = Address::from(slot.bytes20());
    let balance = interpreter.evm.state_db.get_balance(&address);
    slot.set_from_big(&balance);
    Ok(Vec::new())
}

pub fn op_origin(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let origin = interpreter.evm.origin().bytes();
    scope.stack.push(Uint256::from_bytes(&origin));
    Ok(Vec::new())
}

pub fn op_caller(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let caller = Address::from(scope.contract.caller().bytes());
    scope.stack.push(Uint256::from_bytes(&caller));
    Ok(Vec::new())
}

pub fn op_call_value(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let value = match uint256::FromBig(scope.contract.value()) {
        Some(v) => v,
        None => return Err(Error::ConversionError),
    };
    scope.stack.push(value);
    Ok(Vec::new())
}

pub fn op_call_data_load(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let x = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    if let Some(offset) = x.uint64() {
        let data = get_data(&scope.contract.input, offset, 32);
        x.set_bytes(&data);
    } else {
        x.clear();
    }

    Ok(Vec::new())
}

pub fn op_call_data_size(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let size = Uint256::from_uint64(scope.contract.input.len() as u64);
    scope.stack.push(size);

    Ok(Vec::new())
}

pub fn op_call_data_copy(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mem_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let data_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let length = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let data_offset64 = data_offset.uint64().unwrap_or(u64::MAX);
    let mem_offset64 = mem_offset.uint64().unwrap_or(0);
    let length64 = length.uint64().unwrap_or(0);
    scope.memory.set(
        mem_offset64,
        length64,
        &get_data(&scope.contract.input, data_offset64, length64),
    );

    Ok(Vec::new())
}

pub fn op_return_data_size(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let size = Uint256::from_uint64(interpreter.return_data.len() as u64);
    scope.stack.push(size);

    Ok(Vec::new())
}

pub fn op_return_data_copy(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mem_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let data_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let length = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let offset64 = match data_offset.uint64_with_overflow() {
        Some((offset, _)) => offset,
        None => return Err(ErrReturnDataOutOfBounds),
    };
    let end = data_offset.clone() + length.clone();
    let end64 = match end.uint64_with_overflow() {
        Some((end, _)) => end,
        None => return Err(ErrReturnDataOutOfBounds),
    };

    let return_data_len = interpreter.return_data.len();
    if return_data_len < end64 as usize {
        return Err(ErrReturnDataOutOfBounds);
    }

    scope.memory.set(
        mem_offset.uint64(),
        length.uint64(),
        &interpreter.return_data[offset64 as usize..end64 as usize],
    );

    Ok(Vec::new())
}

pub fn op_ext_code_size(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let slot = match scope.stack.peek() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let code_size =
        Uint256::from_uint64(interpreter.evm.state_db.get_code_size(slot.bytes20()) as u64);
    slot.set_from_big(&code_size);

    Ok(Vec::new())
}

pub fn op_code_size(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let code_size = Uint256::from_uint64(scope.contract.code.len() as u64);
    scope.stack.push(code_size);

    Ok(Vec::new())
}

pub fn op_code_copy(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mem_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let code_offset = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let length = match scope.stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let uint64_code_offset = match code_offset.uint64_with_overflow() {
        Some((offset, _)) => offset,
        None => return Err(Error::Overflow),
    };

    let code_copy = get_data(&scope.contract.code, uint64_code_offset, length.uint64());
    scope
        .memory
        .set(mem_offset.uint64(), length.uint64(), &code_copy);

    Ok(Vec::new())
}

pub fn op_ext_code_copy(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let stack = &mut scope.stack;
    let a = match stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let mem_offset = match stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let code_offset = match stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };
    let length = match stack.pop() {
        Some(value) => value,
        None => return Err(Error::StackUnderflow),
    };

    let uint64_code_offset = match code_offset.uint64_with_overflow() {
        Some((offset, _)) => offset,
        None => return Err(Error::Overflow),
    };

    let addr = common::Address::from_bytes20(a.bytes20());
    let code_copy = get_data(
        &interpreter.evm.state_db.get_code(&addr),
        uint64_code_offset,
        length.uint64(),
    );
    scope
        .memory
        .set(mem_offset.uint64(), length.uint64(), &code_copy);

    Ok(Vec::new())
}

fn get_data(data: &[u8], offset: u64, length: u64) -> Vec<u8> {
    let start = offset as usize;
    let end = (offset + length) as usize;

    if start >= data.len() || end > data.len() {
        Vec::new()
    } else {
        data[start..end].to_vec()
    }
}

pub fn op_gasprice(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let gas_price = interpreter.evm.gas_price.clone();
    let v = uint256::Uint256::from_big(gas_price);
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_blockhash(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let num = match scope.stack.peek() {
        Some(value) => value.clone(),
        None => return Err(Error::StackUnderflow),
    };
    let (num64, overflow) = num.uint64_with_overflow();
    if overflow {
        num.clear();
        return Ok(Vec::new());
    }

    let upper = interpreter.evm.context.block_number.uint64();
    let (lower, hash) = if upper < 257 {
        (0, uint256::Uint256::default())
    } else {
        (upper - 256, interpreter.evm.context.get_hash(upper))
    };

    if num64 >= lower && num64 < upper {
        num.set_bytes(hash.bytes());
    } else {
        num.clear();
    }

    Ok(Vec::new())
}

pub fn op_coinbase(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let coinbase = interpreter.evm.context.coinbase.clone();
    let v = uint256::Uint256::from_bytes(coinbase.bytes());
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_timestamp(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let timestamp = interpreter.evm.context.time;
    let v = uint256::Uint256::from_u64(timestamp);
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_number(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let block_number = interpreter.evm.context.block_number.clone();
    let v = uint256::Uint256::from_big(block_number);
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_difficulty(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let difficulty = interpreter.evm.context.difficulty.clone();
    let v = uint256::Uint256::from_big(difficulty);
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_random(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let random = interpreter.evm.context.random.clone();
    let v = uint256::Uint256::from_bytes(random.bytes());
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_gaslimit(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let gas_limit = interpreter.evm.context.gas_limit;
    let v = uint256::Uint256::from_u64(gas_limit);
    scope.stack.push(v);

    Ok(Vec::new())
}

pub fn op_pop(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    scope.stack.pop();

    Ok(Vec::new())
}

pub fn op_mload(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let v = scope.stack.peek();
    let offset = v.to_u64();
    let data = scope.memory.get(offset as i64, 32)?;
    v.set_bytes(&data);

    Ok(Vec::new())
}

pub fn op_mstore(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let val = scope.stack.pop()?;
    let mstart = scope.stack.pop()?;
    scope.memory.set32(mstart.to_u64(), &val)?;

    Ok(Vec::new())
}

pub fn op_mstore8(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let val = scope.stack.pop()?;
    let off = scope.stack.pop()?;
    scope.memory.store[off.to_u64() as usize] = val.to_u8() as u8;

    Ok(Vec::new())
}

pub fn op_sload(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let loc = scope.stack.peek();
    let hash = common::hash::Hash::from_slice(loc.to_bytes32().as_slice()).unwrap();
    let val = interpreter
        .evm
        .state_db
        .get_state(scope.contract.address(), &hash)?;
    loc.set_bytes(val.as_slice());

    Ok(Vec::new())
}

pub fn op_sstore(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    if interpreter.read_only {
        return Err(Error::WriteProtection);
    }

    let loc = scope.stack.pop()?;
    let val = scope.stack.pop()?;
    interpreter.evm.state_db.set_state(
        scope.contract.address(),
        loc.to_bytes32(),
        val.to_bytes32(),
    )?;

    Ok(Vec::new())
}

pub fn op_jump(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    if interpreter.evm.abort.load(Ordering::SeqCst) {
        return Err(Error::StopToken);
    }

    let pos = scope.stack.pop()?;
    if !scope.contract.valid_jumpdest(&pos) {
        return Err(Error::InvalidJump);
    }

    *pc = pos.to_u64().wrapping_sub(1); // pc will be increased by the interpreter loop

    Ok(Vec::new())
}

pub fn op_jumpi(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    if interpreter.evm.abort.load(Ordering::SeqCst) {
        return Err(Error::StopToken);
    }

    let cond = scope.stack.pop()?;
    let pos = scope.stack.pop()?;
    if !cond.is_zero() {
        if !scope.contract.valid_jumpdest(&pos) {
            return Err(Error::InvalidJump);
        }

        *pc = pos.to_u64().wrapping_sub(1); // pc will be increased by the interpreter loop
    }

    Ok(Vec::new())
}

pub fn op_jumpdest(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}

pub fn op_pc(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let pc_value = uint256::Uint256::from(*pc);
    scope.stack.push(pc_value);

    Ok(Vec::new())
}

pub fn op_msize(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let memory_size = uint256::Uint256::from(scope.memory.len());
    scope.stack.push(memory_size);

    Ok(Vec::new())
}

pub fn op_gas(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let contract_gas = uint256::Uint256::from(scope.contract.gas);
    scope.stack.push(contract_gas);

    Ok(Vec::new())
}

pub fn op_create(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    if interpreter.read_only {
        return Err(Error::WriteProtection);
    }

    let value = scope.stack.pop()?;
    let offset = scope.stack.pop()?;
    let size = scope.stack.pop()?;
    let input = scope
        .memory
        .get_copy(offset.to_u64() as i64, size.to_u64() as i64);
    let gas = scope.contract.gas;

    if interpreter.evm.chain_rules.is_eip150 {
        // Adjust gas for EIP-150
        let gas_adjustment = gas / 64;
        gas -= gas_adjustment;
    }

    scope.contract.use_gas(gas);

    let big_val = if !value.is_zero() {
        Some(value.to_bigint())
    } else {
        None
    };

    let (res, addr, return_gas, suberr) =
        interpreter
            .evm
            .create(scope.contract.clone(), input, gas, big_val);

    let mut stackvalue = uint256::Uint256::zero();
    if interpreter.evm.chain_rules.is_homestead && suberr == ErrCodeStoreOutOfGas {
        stackvalue.clear();
    } else if suberr.is_some() && suberr != Some(ErrCodeStoreOutOfGas) {
        stackvalue.clear();
    } else {
        stackvalue.set_bytes(addr.bytes());
    }

    scope.stack.push(stackvalue);
    scope.contract.gas += return_gas;

    if suberr == Some(ErrExecutionReverted) {
        interpreter.return_data = res.clone(); // Set REVERT data to return data buffer
        return Ok(res);
    }

    interpreter.return_data = None; // Clear dirty return data buffer

    Ok(Vec::new())
}

pub fn op_call(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mut stack = &mut scope.stack;
    let temp = stack.pop()?;
    let gas = interpreter.evm.call_gas_temp;
    let addr = stack.pop()?;
    let value = stack.pop()?;
    let in_offset = stack.pop()?;
    let in_size = stack.pop()?;
    let ret_offset = stack.pop()?;
    let ret_size = stack.pop()?;
    let to_addr = common::Address::from(addr.bytes20());
    let args = scope
        .memory
        .get_ptr(in_offset.to_u64() as i64, in_size.to_u64() as i64);

    if interpreter.read_only && !value.is_zero() {
        return Err(Error::WriteProtection);
    }

    let mut big_val = big0;
    if !value.is_zero() {
        let call_stipend = params::CALL_STIPEND;
        big_val = value.to_bigint();
        gas += call_stipend;
    }

    let (ret, return_gas, err) =
        interpreter
            .evm
            .call(scope.contract.clone(), to_addr, args, gas, big_val);

    if err.is_some() {
        temp.clear();
    } else {
        temp.set_one();
    }

    stack.push(temp.clone());

    if err.is_none() || err == Some(ErrExecutionReverted) {
        scope
            .memory
            .set(ret_offset.to_u64(), ret_size.to_u64(), &ret);
    }

    scope.contract.gas += return_gas;

    interpreter.return_data = Some(ret.clone());

    Ok(ret)
}

pub fn op_call_code(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mut stack = &mut scope.stack;
    let temp = stack.pop()?;
    let gas = interpreter.evm.call_gas_temp;
    let addr = stack.pop()?;
    let value = stack.pop()?;
    let in_offset = stack.pop()?;
    let in_size = stack.pop()?;
    let ret_offset = stack.pop()?;
    let ret_size = stack.pop()?;
    let to_addr = common::Address::from(addr.bytes20());
    let args = scope
        .memory
        .get_ptr(in_offset.to_u64() as i64, in_size.to_u64() as i64);

    let mut big_val = big0;
    if !value.is_zero() {
        let call_stipend = params::CALL_STIPEND;
        big_val = value.to_bigint();
        gas += call_stipend;
    }

    let (ret, return_gas, err) =
        interpreter
            .evm
            .call_code(scope.contract.clone(), to_addr, args, gas, big_val);

    if err.is_some() {
        temp.clear();
    } else {
        temp.set_one();
    }

    stack.push(temp.clone());

    if err.is_none() || err == Some(ErrExecutionReverted) {
        scope
            .memory
            .set(ret_offset.to_u64(), ret_size.to_u64(), &ret);
    }

    scope.contract.gas += return_gas;

    interpreter.return_data = Some(ret.clone());

    Ok(ret)
}

pub fn op_static_call(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let mut stack = &mut scope.stack;
    let temp = stack.pop()?;
    let gas = interpreter.evm.call_gas_temp;
    let addr = stack.pop()?;
    let in_offset = stack.pop()?;
    let in_size = stack.pop()?;
    let ret_offset = stack.pop()?;
    let ret_size = stack.pop()?;
    let to_addr = common::Address::from(addr.bytes20());
    let args = scope
        .memory
        .get_ptr(in_offset.to_u64() as i64, in_size.to_u64() as i64);

    let (ret, return_gas, err) =
        interpreter
            .evm
            .static_call(scope.contract.clone(), to_addr, args, gas);

    if err.is_some() {
        temp.clear();
    } else {
        temp.set_one();
    }

    stack.push(temp.clone());

    if err.is_none() || err == Some(ErrExecutionReverted) {
        scope
            .memory
            .set(ret_offset.to_u64(), ret_size.to_u64(), &ret);
    }

    scope.contract.gas += return_gas;

    interpreter.return_data = Some(ret.clone());

    Ok(ret)
}

pub fn op_return(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let offset = scope.stack.pop()?;
    let size = scope.stack.pop()?;
    let ret = scope
        .memory
        .get_ptr(offset.to_u64() as i64, size.to_u64() as i64);

    Err(Error::StopToken)
}

pub fn op_revert(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let offset = scope.stack.pop()?;
    let size = scope.stack.pop()?;
    let ret = scope
        .memory
        .get_ptr(offset.to_u64() as i64, size.to_u64() as i64);

    interpreter.return_data = Some(ret.clone());

    Err(Error::ExecutionReverted)
}

pub fn op_undefined(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    let opcode = scope.contract.code[*pc] as u8;
    Err(Error::InvalidOpCode { opcode })
}

pub fn op_stop(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    Err(Error::StopToken)
}

pub fn op_selfdestruct(
    pc: &mut u64,
    interpreter: &mut interpreter::EVMInterpreter,
    scope: &mut interpreter::ScopeContext,
) -> Result<Vec<u8>, Error> {
    if interpreter.read_only {
        return Err(Error::WriteProtection);
    }

    let beneficiary = scope.stack.pop()?;
    let balance = interpreter
        .evm
        .state_db
        .get_balance(&scope.contract.address());
    interpreter
        .evm
        .state_db
        .add_balance(beneficiary.bytes20(), balance);
    interpreter.evm.state_db.suicide(scope.contract.address());

    if let Some(tracer) = &interpreter.evm.config.tracer {
        tracer.capture_enter(
            SELFDESTRUCT,
            scope.contract.address(),
            beneficiary.bytes20(),
            &[],
            0,
            balance,
        );
        tracer.capture_exit(&[], 0, None);
    }

    Err(Error::StopToken)
}

fn make_log(size: usize) -> ExecutionFunc {
    Box::new(
        move |pc: &mut u64,
              interpreter: &mut EVMInterpreter,
              scope: &mut ScopeContext|
              -> Result<Vec<u8>, Error> {
            if interpreter.read_only {
                return Err(Error::WriteProtection);
            }

            let mut topics = vec![common::Hash::default(); size];
            let mut stack = &mut scope.stack;
            let m_start = stack.pop()?;
            let m_size = stack.pop()?;

            for i in 0..size {
                let addr = stack.pop()?;
                topics[i] = addr.bytes32();
            }

            let d = scope
                .memory
                .get_copy(m_start.to_u64() as i64, m_size.to_u64() as i64);
            let log = types::Log {
                address: scope.contract.address().clone(),
                topics,
                data: d,
                block_number: interpreter.evm.context.block_number().to_u64(),
                ..Default::default()
            };

            interpreter.evm.state_db.add_log(log);

            Ok(vec![])
        },
    )
}

fn op_push1(
    pc: &mut u64,
    interpreter: &mut EVMInterpreter,
    scope: &mut ScopeContext,
) -> Result<Vec<u8>, Error> {
    let code_len = scope.contract.code.len() as u64;
    *pc += 1;

    if *pc < code_len {
        let push_byte = uint256::Int::from_uint64(scope.contract.code[*pc]);
        scope.stack.push(push_byte);
    } else {
        scope.stack.push(uint256::Int::default());
    }

    Ok(vec![])
}

fn make_push(size: u64, push_byte_size: usize) -> ExecutionFunc {
    Box::new(
        move |pc: &mut u64,
              interpreter: &mut EVMInterpreter,
              scope: &mut ScopeContext|
              -> Result<Vec<u8>, Error> {
            let code_len = scope.contract.code.len();
            let start_min = std::cmp::min(code_len, *pc as usize + 1);
            let end_min = std::cmp::min(code_len, start_min + push_byte_size);

            let push_bytes =
                common::right_pad_bytes(&scope.contract.code[start_min..end_min], push_byte_size);
            let integer = uint256::Int::from_bytes(&push_bytes);
            scope.stack.push(integer);

            *pc += size;

            Ok(vec![])
        },
    )
}

fn make_dup(size: i64) -> ExecutionFunc {
    Box::new(
        move |pc: &mut u64,
              interpreter: &mut EVMInterpreter,
              scope: &mut ScopeContext|
              -> Result<Vec<u8>, Error> {
            scope.stack.dup(size as usize);

            Ok(vec![])
        },
    )
}

fn make_swap(size: i64) -> ExecutionFunc {
    Box::new(
        move |pc: &mut u64,
              interpreter: &mut EVMInterpreter,
              scope: &mut ScopeContext|
              -> Result<Vec<u8>, Error> {
            scope.stack.swap(size as usize);

            Ok(vec![])
        },
    )
}
