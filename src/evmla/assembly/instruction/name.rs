//!
//! The EVM instruction name.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::assembly::instruction::Instruction;

///
/// The EVM instruction name.
///
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum Name {
    /// The eponymous EVM instruction.
    PUSH,
    /// Pushes a constant tag index.
    #[serde(rename = "PUSH [tag]")]
    PUSH_Tag,
    /// Pushes an unknown `data` value.
    #[serde(rename = "PUSH data")]
    PUSH_Data,
    /// Pushes a contract hash size.
    #[serde(rename = "PUSH #[$]")]
    PUSH_ContractHashSize,
    /// Pushes a contract hash.
    #[serde(rename = "PUSH [$]")]
    PUSH_ContractHash,

    /// The eponymous EVM instruction.
    PUSH1,
    /// The eponymous EVM instruction.
    PUSH2,
    /// The eponymous EVM instruction.
    PUSH3,
    /// The eponymous EVM instruction.
    PUSH4,
    /// The eponymous EVM instruction.
    PUSH5,
    /// The eponymous EVM instruction.
    PUSH6,
    /// The eponymous EVM instruction.
    PUSH7,
    /// The eponymous EVM instruction.
    PUSH8,
    /// The eponymous EVM instruction.
    PUSH9,
    /// The eponymous EVM instruction.
    PUSH10,
    /// The eponymous EVM instruction.
    PUSH11,
    /// The eponymous EVM instruction.
    PUSH12,
    /// The eponymous EVM instruction.
    PUSH13,
    /// The eponymous EVM instruction.
    PUSH14,
    /// The eponymous EVM instruction.
    PUSH15,
    /// The eponymous EVM instruction.
    PUSH16,
    /// The eponymous EVM instruction.
    PUSH17,
    /// The eponymous EVM instruction.
    PUSH18,
    /// The eponymous EVM instruction.
    PUSH19,
    /// The eponymous EVM instruction.
    PUSH20,
    /// The eponymous EVM instruction.
    PUSH21,
    /// The eponymous EVM instruction.
    PUSH22,
    /// The eponymous EVM instruction.
    PUSH23,
    /// The eponymous EVM instruction.
    PUSH24,
    /// The eponymous EVM instruction.
    PUSH25,
    /// The eponymous EVM instruction.
    PUSH26,
    /// The eponymous EVM instruction.
    PUSH27,
    /// The eponymous EVM instruction.
    PUSH28,
    /// The eponymous EVM instruction.
    PUSH29,
    /// The eponymous EVM instruction.
    PUSH30,
    /// The eponymous EVM instruction.
    PUSH31,
    /// The eponymous EVM instruction.
    PUSH32,

    /// The eponymous EVM instruction.
    DUP1,
    /// The eponymous EVM instruction.
    DUP2,
    /// The eponymous EVM instruction.
    DUP3,
    /// The eponymous EVM instruction.
    DUP4,
    /// The eponymous EVM instruction.
    DUP5,
    /// The eponymous EVM instruction.
    DUP6,
    /// The eponymous EVM instruction.
    DUP7,
    /// The eponymous EVM instruction.
    DUP8,
    /// The eponymous EVM instruction.
    DUP9,
    /// The eponymous EVM instruction.
    DUP10,
    /// The eponymous EVM instruction.
    DUP11,
    /// The eponymous EVM instruction.
    DUP12,
    /// The eponymous EVM instruction.
    DUP13,
    /// The eponymous EVM instruction.
    DUP14,
    /// The eponymous EVM instruction.
    DUP15,
    /// The eponymous EVM instruction.
    DUP16,

    /// The eponymous EVM instruction.
    SWAP1,
    /// The eponymous EVM instruction.
    SWAP2,
    /// The eponymous EVM instruction.
    SWAP3,
    /// The eponymous EVM instruction.
    SWAP4,
    /// The eponymous EVM instruction.
    SWAP5,
    /// The eponymous EVM instruction.
    SWAP6,
    /// The eponymous EVM instruction.
    SWAP7,
    /// The eponymous EVM instruction.
    SWAP8,
    /// The eponymous EVM instruction.
    SWAP9,
    /// The eponymous EVM instruction.
    SWAP10,
    /// The eponymous EVM instruction.
    SWAP11,
    /// The eponymous EVM instruction.
    SWAP12,
    /// The eponymous EVM instruction.
    SWAP13,
    /// The eponymous EVM instruction.
    SWAP14,
    /// The eponymous EVM instruction.
    SWAP15,
    /// The eponymous EVM instruction.
    SWAP16,

    /// The eponymous EVM instruction.
    POP,

    /// Sets the current basic code block.
    #[serde(rename = "tag")]
    Tag,
    /// The eponymous EVM instruction.
    JUMP,
    /// The eponymous EVM instruction.
    JUMPI,
    /// The eponymous EVM instruction.
    JUMPDEST,

    /// The eponymous EVM instruction.
    ADD,
    /// The eponymous EVM instruction.
    SUB,
    /// The eponymous EVM instruction.
    MUL,
    /// The eponymous EVM instruction.
    DIV,
    /// The eponymous EVM instruction.
    MOD,
    /// The eponymous EVM instruction.
    SDIV,
    /// The eponymous EVM instruction.
    SMOD,

    /// The eponymous EVM instruction.
    LT,
    /// The eponymous EVM instruction.
    GT,
    /// The eponymous EVM instruction.
    EQ,
    /// The eponymous EVM instruction.
    ISZERO,
    /// The eponymous EVM instruction.
    SLT,
    /// The eponymous EVM instruction.
    SGT,

    /// The eponymous EVM instruction.
    OR,
    /// The eponymous EVM instruction.
    XOR,
    /// The eponymous EVM instruction.
    NOT,
    /// The eponymous EVM instruction.
    AND,
    /// The eponymous EVM instruction.
    SHL,
    /// The eponymous EVM instruction.
    SHR,
    /// The eponymous EVM instruction.
    SAR,
    /// The eponymous EVM instruction.
    BYTE,

    /// The eponymous EVM instruction.
    ADDMOD,
    /// The eponymous EVM instruction.
    MULMOD,
    /// The eponymous EVM instruction.
    EXP,
    /// The eponymous EVM instruction.
    SIGNEXTEND,
    /// The eponymous EVM instruction.
    SHA3,
    /// The eponymous EVM instruction.
    KECCAK256,

    /// The eponymous EVM instruction.
    MLOAD,
    /// The eponymous EVM instruction.
    MSTORE,
    /// The eponymous EVM instruction.
    MSTORE8,

    /// The eponymous EVM instruction.
    SLOAD,
    /// The eponymous EVM instruction.
    SSTORE,
    /// The eponymous EVM instruction.
    PUSHIMMUTABLE,
    /// The eponymous EVM instruction.
    ASSIGNIMMUTABLE,

    /// The eponymous EVM instruction.
    CALLDATALOAD,
    /// The eponymous EVM instruction.
    CALLDATASIZE,
    /// The eponymous EVM instruction.
    CALLDATACOPY,
    /// The eponymous EVM instruction.
    CODESIZE,
    /// The eponymous EVM instruction.
    CODECOPY,
    /// The eponymous EVM instruction.
    PUSHSIZE,
    /// The eponymous EVM instruction.
    EXTCODESIZE,
    /// The eponymous EVM instruction.
    EXTCODEHASH,
    /// The eponymous EVM instruction.
    RETURNDATASIZE,
    /// The eponymous EVM instruction.
    RETURNDATACOPY,

    /// The eponymous EVM instruction.
    RETURN,
    /// The eponymous EVM instruction.
    REVERT,
    /// The eponymous EVM instruction.
    STOP,
    /// The eponymous EVM instruction.
    INVALID,

    /// The eponymous EVM instruction.
    LOG0,
    /// The eponymous EVM instruction.
    LOG1,
    /// The eponymous EVM instruction.
    LOG2,
    /// The eponymous EVM instruction.
    LOG3,
    /// The eponymous EVM instruction.
    LOG4,

    /// The eponymous EVM instruction.
    CALL,
    /// The eponymous EVM instruction.
    STATICCALL,
    /// The eponymous EVM instruction.
    DELEGATECALL,

    /// The eponymous EVM instruction.
    CREATE,
    /// The eponymous EVM instruction.
    CREATE2,

    /// The eponymous zkEVM instruction.
    #[serde(rename = "$ZK_CREATE")]
    ZK_CREATE,
    /// The eponymous zkEVM instruction.
    #[serde(rename = "$ZK_CREATE2")]
    ZK_CREATE2,

    /// The eponymous EVM instruction.
    ADDRESS,
    /// The eponymous EVM instruction.
    CALLER,

    /// The eponymous EVM instruction.
    CALLVALUE,
    /// The eponymous EVM instruction.
    GAS,
    /// The eponymous EVM instruction.
    BALANCE,
    /// The eponymous EVM instruction.
    SELFBALANCE,

    /// The eponymous EVM instruction.
    PUSHLIB,
    /// The eponymous EVM instruction.
    PUSHDEPLOYADDRESS,

    /// The eponymous EVM instruction.
    GASLIMIT,
    /// The eponymous EVM instruction.
    GASPRICE,
    /// The eponymous EVM instruction.
    ORIGIN,
    /// The eponymous EVM instruction.
    CHAINID,
    /// The eponymous EVM instruction.
    TIMESTAMP,
    /// The eponymous EVM instruction.
    NUMBER,
    /// The eponymous EVM instruction.
    BLOCKHASH,
    /// The eponymous EVM instruction.
    DIFFICULTY,
    /// The eponymous EVM instruction.
    PREVRANDAO,
    /// The eponymous EVM instruction.
    COINBASE,
    /// The eponymous EVM instruction.
    BASEFEE,
    /// The eponymous EVM instruction.
    MSIZE,

    /// The eponymous EVM instruction.
    CALLCODE,
    /// The eponymous EVM instruction.
    PC,
    /// The eponymous EVM instruction.
    EXTCODECOPY,
    /// The eponymous EVM instruction.
    SELFDESTRUCT,
}

impl From<Name> for Instruction {
    fn from(name: Name) -> Self {
        Self { name, value: None }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag => write!(f, "Tag ",),
            _ => write!(
                f,
                "{:16}",
                serde_json::to_string(self)
                    .expect("Always valid")
                    .trim_matches('\"')
            ),
        }
    }
}
