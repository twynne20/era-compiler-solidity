//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::assembly::Assembly;

use self::bytecode::Bytecode;

///
/// The `solc --standard-json` output contract EVM data.
///
/// It is replaced by zkEVM data after compiling.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// The contract EVM legacy assembly code.
    #[serde(rename = "legacyAssembly")]
    pub assembly: Option<Assembly>,
    /// The contract zkEVM assembly code.
    #[serde(rename = "assembly")]
    pub assembly_text: Option<String>,
    /// The contract bytecode.
    /// Is reset by that of zkEVM before yielding the compiled project artifacts.
    pub bytecode: Option<Bytecode>,
    /// The contract function signatures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,
}

impl EVM {
    ///
    /// Sets the zkEVM assembly and bytecode.
    ///
    pub fn modify(&mut self, assembly_text: String, bytecode: String) {
        self.assembly = None;
        self.assembly_text = Some(assembly_text);
        self.bytecode = Some(Bytecode::new(bytecode));
    }
}
