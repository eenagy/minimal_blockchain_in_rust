// OpCode is an EVM opcode
pub type OpCode = u8;

// IsPush specifies if an opcode is a PUSH opcode.
pub fn is_push(op: OpCode) -> bool {
    OpCode::PUSH1 <= op && op <= OpCode::PUSH32
}

// 0x0 range - arithmetic ops.
pub const STOP: OpCode = 0x0;
pub const ADD: OpCode = 0x1;
pub const MUL: OpCode = 0x2;
pub const SUB: OpCode = 0x3;
pub const DIV: OpCode = 0x4;
pub const SDIV: OpCode = 0x5;
pub const MOD: OpCode = 0x6;
pub const SMOD: OpCode = 0x7;
pub const ADDMOD: OpCode = 0x8;
pub const MULMOD: OpCode = 0x9;
pub const EXP: OpCode = 0xa;
pub const SIGNEXTEND: OpCode = 0xb;

// 0x10 range - comparison ops.
pub const LT: OpCode = 0x10;
pub const GT: OpCode = 0x11;
pub const SLT: OpCode = 0x12;
pub const SGT: OpCode = 0x13;
pub const EQ: OpCode = 0x14;
pub const ISZERO: OpCode = 0x15;
pub const AND: OpCode = 0x16;
pub const OR: OpCode = 0x17;
pub const XOR: OpCode = 0x18;
pub const NOT: OpCode = 0x19;
pub const BYTE: OpCode = 0x1a;
pub const SHL: OpCode = 0x1b;
pub const SHR: OpCode = 0x1c;
pub const SAR: OpCode = 0x1d;

// 0x20 range - crypto.
pub const KECCAK256: OpCode = 0x20;

// 0x30 range - closure state.
pub const ADDRESS: OpCode = 0x30;
pub const BALANCE: OpCode = 0x31;
pub const ORIGIN: OpCode = 0x32;
pub const CALLER: OpCode = 0x33;
pub const CALLVALUE: OpCode = 0x34;
pub const CALLDATALOAD: OpCode = 0x35;
pub const CALLDATASIZE: OpCode = 0x36;
pub const CALLDATACOPY: OpCode = 0x37;
pub const CODESIZE: OpCode = 0x38;
pub const CODECOPY: OpCode = 0x39;
pub const GASPRICE: OpCode = 0x3a;
pub const EXTCODESIZE: OpCode = 0x3b;
pub const EXTCODECOPY: OpCode = 0x3c;
pub const RETURNDATASIZE: OpCode = 0x3d;
pub const RETURNDATACOPY: OpCode = 0x3e;
pub const EXTCODEHASH: OpCode = 0x3f;

// 0x40 range - block operations.
pub const BLOCKHASH: OpCode = 0x40;
pub const COINBASE: OpCode = 0x41;
pub const TIMESTAMP: OpCode = 0x42;
pub const NUMBER: OpCode = 0x43;
pub const DIFFICULTY: OpCode = 0x44;
pub const RANDOM: OpCode = 0x44; // Same as DIFFICULTY
pub const PREVRANDAO: OpCode = 0x44; // Same as DIFFICULTY
pub const GASLIMIT: OpCode = 0x45;
pub const CHAINID: OpCode = 0x46;
pub const SELFBALANCE: OpCode = 0x47;
pub const BASEFEE: OpCode = 0x48;
pub const BLOBHASH: OpCode = 0x49;

// 0x50 range - 'storage' and execution.
pub const POP: OpCode = 0x50;
pub const MLOAD: OpCode = 0x51;
pub const MSTORE: OpCode = 0x52;
pub const MSTORE8: OpCode = 0x53;
pub const SLOAD: OpCode = 0x54;
pub const SSTORE: OpCode = 0x55;
pub const JUMP: OpCode = 0x56;
pub const JUMPI: OpCode = 0x57;
pub const PC: OpCode = 0x58;
pub const MSIZE: OpCode = 0x59;
pub const GAS: OpCode = 0x5a;
pub const JUMPDEST: OpCode = 0x5b;
pub const PUSH0: OpCode = 0x5f;

// 0x60 range - pushes.
pub const PUSH1: OpCode = 0x60;
pub const PUSH2: OpCode = 0x61;
pub const PUSH3: OpCode = 0x62;
pub const PUSH4: OpCode = 0x63;
pub const PUSH5: OpCode = 0x64;
pub const PUSH6: OpCode = 0x65;
pub const PUSH7: OpCode = 0x66;
pub const PUSH8: OpCode = 0x67;
pub const PUSH9: OpCode = 0x68;
pub const PUSH10: OpCode = 0x69;
pub const PUSH11: OpCode = 0x6A;
pub const PUSH12: OpCode = 0x6B;
pub const PUSH13: OpCode = 0x6C;
pub const PUSH14: OpCode = 0x6D;
pub const PUSH15: OpCode = 0x6E;
pub const PUSH16: OpCode = 0x6F;
pub const PUSH17: OpCode = 0x70;
pub const PUSH18: OpCode = 0x71;
pub const PUSH19: OpCode = 0x72;
pub const PUSH20: OpCode = 0x73;
pub const PUSH21: OpCode = 0x74;
pub const PUSH22: OpCode = 0x75;
pub const PUSH23: OpCode = 0x76;
pub const PUSH24: OpCode = 0x77;
pub const PUSH25: OpCode = 0x78;
pub const PUSH26: OpCode = 0x79;
pub const PUSH27: OpCode = 0x7A;
pub const PUSH28: OpCode = 0x7B;
pub const PUSH29: OpCode = 0x7C;
pub const PUSH30: OpCode = 0x7D;
pub const PUSH31: OpCode = 0x7E;
pub const PUSH32: OpCode = 0x7F;

// 0x80 range - dups.
pub const DUP1: OpCode = 0x80;
pub const DUP2: OpCode = 0x81;
pub const DUP3: OpCode = 0x82;
pub const DUP4: OpCode = 0x83;
pub const DUP5: OpCode = 0x84;
pub const DUP6: OpCode = 0x85;
pub const DUP7: OpCode = 0x86;
pub const DUP8: OpCode = 0x87;
pub const DUP9: OpCode = 0x88;
pub const DUP10: OpCode = 0x89;
pub const DUP11: OpCode = 0x8A;
pub const DUP12: OpCode = 0x8B;
pub const DUP13: OpCode = 0x8C;
pub const DUP14: OpCode = 0x8D;
pub const DUP15: OpCode = 0x8E;
pub const DUP16: OpCode = 0x8F;

// 0x90 range - swaps.
pub const SWAP1: OpCode = 0x90;
pub const SWAP2: OpCode = 0x91;
pub const SWAP3: OpCode = 0x92;
pub const SWAP4: OpCode = 0x93;
pub const SWAP5: OpCode = 0x94;
pub const SWAP6: OpCode = 0x95;
pub const SWAP7: OpCode = 0x96;
pub const SWAP8: OpCode = 0x97;
pub const SWAP9: OpCode = 0x98;
pub const SWAP10: OpCode = 0x99;
pub const SWAP11: OpCode = 0x9A;
pub const SWAP12: OpCode = 0x9B;
pub const SWAP13: OpCode = 0x9C;
pub const SWAP14: OpCode = 0x9D;
pub const SWAP15: OpCode = 0x9E;
pub const SWAP16: OpCode = 0x9F;

// 0xa0 range - logging ops.
pub const LOG0: OpCode = 0xa0;
pub const LOG1: OpCode = 0xa1;
pub const LOG2: OpCode = 0xa2;
pub const LOG3: OpCode = 0xa3;
pub const LOG4: OpCode = 0xa4;

// 0xb0 range.
pub const TLOAD: OpCode = 0xb3;
pub const TSTORE: OpCode = 0xb4;

// 0xf0 range - closures.
pub const CREATE: OpCode = 0xf0;
pub const CALL: OpCode = 0xf1;
pub const CALLCODE: OpCode = 0xf2;
pub const RETURN: OpCode = 0xf3;
pub const DELEGATECALL: OpCode = 0xf4;
pub const CREATE2: OpCode = 0xf5;

pub const STATICCALL: OpCode = 0xfa;
pub const REVERT: OpCode = 0xfd;
pub const INVALID: OpCode = 0xfe;
pub const SELFDESTRUCT: OpCode = 0xff;
