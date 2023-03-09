use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Opcode {
        match byte {
            0x00 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::MUL,
            0x03 => Opcode::SUB,
            0x04 => Opcode::DIV,
            0x05 => Opcode::SDIV,
            0x06 => Opcode::MOD,
            0x07 => Opcode::SMOD,
            0x08 => Opcode::ADDMOD,
            0x09 => Opcode::MULMOD,
            0x0A => Opcode::EXP,
            0x0B => Opcode::SIGNEXTEND,
            0x10 => Opcode::LT,
            0x11 => Opcode::GT,
            0x12 => Opcode::SLT,
            0x13 => Opcode::SGT,
            0x14 => Opcode::EQ,
            0x15 => Opcode::ISZERO,
            0x16 => Opcode::AND,
            0x17 => Opcode::OR,
            0x18 => Opcode::XOR,
            0x19 => Opcode::NOT,
            0x1a => Opcode::BYTE,
            0x1b => Opcode::SHL,
            0x1c => Opcode::SHR,
            0x1d => Opcode::SAR,
            0x20 => Opcode::SHA3,
            0x30 => Opcode::ADDRESS,
            0x31 => Opcode::BALANCE,
            0x32 => Opcode::ORIGIN,
            0x33 => Opcode::CALLER,
            0x34 => Opcode::CALLVALUE,
            0x35 => Opcode::CALLDATALOAD,
            0x36 => Opcode::CALLDATASIZE,
            0x37 => Opcode::CALLDATACOPY,
            0x38 => Opcode::CODESIZE,
            0x39 => Opcode::CODECOPY,
            0x3a => Opcode::GASPRICE,
            0x3b => Opcode::EXTCODESIZE,
            0x3c => Opcode::EXTCODECOPY,
            0x3d => Opcode::RETURNDATASIZE,
            0x3e => Opcode::RETURNDATACOPY,
            0x3f => Opcode::EXTCODEHASH,
            0x40 => Opcode::BLOCKHASH,
            0x41 => Opcode::COINBASE,
            0x42 => Opcode::TIMESTAMP,
            0x43 => Opcode::NUMBER,
            0x44 => Opcode::DIFFICULTY,
            0x45 => Opcode::GASLIMIT,
            0x46 => Opcode::CHAINID,
            0x47 => Opcode::SELFBALANCE,
            0x48 => Opcode::BASEFEE,
            0x50 => Opcode::POP,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::SSTORE,
            0x56 => Opcode::JUMP,
            0x57 => Opcode::JUMPI,
            0x58 => Opcode::PC,
            0x59 => Opcode::MSIZE,
            0x5a => Opcode::GAS,
            0x5b => Opcode::JUMPDEST,
            0x60 => Opcode::PUSH1,
            0x61 => Opcode::PUSH2,
            0x62 => Opcode::PUSH3,
            0x63 => Opcode::PUSH4,
            0x64 => Opcode::PUSH5,
            0x65 => Opcode::PUSH6,
            0x66 => Opcode::PUSH7,
            0x67 => Opcode::PUSH8,
            0x68 => Opcode::PUSH9,
            0x69 => Opcode::PUSH10,
            0x6a => Opcode::PUSH11,
            0x6b => Opcode::PUSH12,
            0x6c => Opcode::PUSH13,
            0x6d => Opcode::PUSH14,
            0x6e => Opcode::PUSH15,
            0x6f => Opcode::PUSH16,
            0x70 => Opcode::PUSH17,
            0x71 => Opcode::PUSH18,
            0x72 => Opcode::PUSH19,
            0x73 => Opcode::PUSH20,
            0x74 => Opcode::PUSH21,
            0x75 => Opcode::PUSH22,
            0x76 => Opcode::PUSH23,
            0x77 => Opcode::PUSH24,
            0x78 => Opcode::PUSH25,
            0x79 => Opcode::PUSH26,
            0x7a => Opcode::PUSH27,
            0x7b => Opcode::PUSH28,
            0x7c => Opcode::PUSH29,
            0x7d => Opcode::PUSH30,
            0x7e => Opcode::PUSH31,
            0x7f => Opcode::PUSH32,
            0x80 => Opcode::DUP1,
            0x81 => Opcode::DUP2,
            0x82 => Opcode::DUP3,
            0x83 => Opcode::DUP4,
            0x84 => Opcode::DUP5,
            0x85 => Opcode::DUP6,
            0x86 => Opcode::DUP7,
            0x87 => Opcode::DUP8,
            0x88 => Opcode::DUP9,
            0x89 => Opcode::DUP10,
            0x8a => Opcode::DUP11,
            0x8b => Opcode::DUP12,
            0x8c => Opcode::DUP13,
            0x8d => Opcode::DUP14,
            0x8e => Opcode::DUP15,
            0x8f => Opcode::DUP16,
            0x90 => Opcode::SWAP1,
            0x91 => Opcode::SWAP2,
            0x92 => Opcode::SWAP3,
            0x93 => Opcode::SWAP4,
            0x94 => Opcode::SWAP5,
            0x95 => Opcode::SWAP6,
            0x96 => Opcode::SWAP7,
            0x97 => Opcode::SWAP8,
            0x98 => Opcode::SWAP9,
            0x99 => Opcode::SWAP10,
            0x9a => Opcode::SWAP11,
            0x9b => Opcode::SWAP12,
            0x9c => Opcode::SWAP13,
            0x9d => Opcode::SWAP14,
            0x9e => Opcode::SWAP15,
            0x9f => Opcode::SWAP16,
            0xa0 => Opcode::LOG0,
            0xa1 => Opcode::LOG1,
            0xa2 => Opcode::LOG2,
            0xa3 => Opcode::LOG3,
            0xa4 => Opcode::LOG4,
            0xf0 => Opcode::CREATE,
            0xf1 => Opcode::CALL,
            0xf2 => Opcode::CALLCODE,
            0xf3 => Opcode::RETURN,
            0xf4 => Opcode::DELEGATECALL,
            0xf5 => Opcode::CREATE2,
            0xfa => Opcode::STATICCALL,
            0xfd => Opcode::REVERT,
            0xff => Opcode::SELFDESTRUCT,
            _ => Opcode::INVALID,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Operation {
    pub opcode: Opcode,
    pub input: Vec<u8>,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.input.len() > 0 {
            write!(
                f,
                "{:?} {}",
                self.opcode,
                "0x".to_owned() + &hex::encode(&self.input)
            )
        } else {
            write!(f, "{:?}", self.opcode,)
        }
    }
}

impl Operation {
    pub fn new(opcode: Opcode) -> Self {
        Operation {
            opcode,
            input: Vec::new(),
        }
    }

    pub fn with_bytes(self, num_bytes: u8, bytes: &mut VecDeque<u8>) -> Self {
        if num_bytes == 0 {
            return self;
        }
        Operation {
            opcode: self.opcode,
            input: bytes.drain(0..num_bytes as usize).collect::<Vec<u8>>(),
        }
    }
}