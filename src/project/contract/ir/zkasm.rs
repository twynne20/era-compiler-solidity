//!
//! The contract zkEVM assembly source code.
//!

///
/// The contract zkEVM assembly source code.
///
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct ZKASM {
    /// The zkEVM assembly file path.
    pub path: String,
    /// The zkEVM assembly source code.
    pub source: String,
}

impl ZKASM {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(path: String, source: String) -> Self {
        Self { path, source }
    }
}
