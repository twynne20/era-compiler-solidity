//!
//! The `solc --standard-json` input.
//!

pub mod language;
pub mod settings;
pub mod source;

use std::collections::BTreeMap;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::metadata::Metadata as SolcStandardJsonInputSettingsMetadata;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;

use self::language::Language;
use self::settings::Settings;
use self::source::Source;

///
/// The `solc --standard-json` input.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The input language.
    pub language: Language,
    /// The input source code files hashmap.
    pub sources: BTreeMap<String, Source>,
    /// The compiler settings.
    pub settings: Settings,
}

impl Input {
    ///
    /// A shortcut constructor from stdin.
    ///
    pub fn try_from_stdin(solc_pipeline: SolcPipeline, via_ir: bool) -> anyhow::Result<Self> {
        let mut input: Self = serde_json::from_reader(std::io::BufReader::new(std::io::stdin()))?;
        input
            .settings
            .output_selection
            .get_or_insert_with(SolcStandardJsonInputSettingsSelection::default)
            .extend_with_required(solc_pipeline);
        input.settings.via_ir = if via_ir { Some(true) } else { None };
        Ok(input)
    }

    ///
    /// A shortcut constructor from paths.
    ///
    pub fn try_from_paths(
        language: Language,
        paths: &[PathBuf],
        library_map: Vec<String>,
        output_selection: SolcStandardJsonInputSettingsSelection,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
        metadata: Option<SolcStandardJsonInputSettingsMetadata>,
        via_ir: bool,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_from(path.as_path()).unwrap_or_else(|error| {
                    panic!("Source code file {path:?} reading error: {error}")
                });
                (path.to_string_lossy().to_string(), source)
            })
            .collect();

        let libraries = Settings::parse_libraries(library_map)?;

        Ok(Self {
            language,
            sources,
            settings: Settings::new(libraries, output_selection, optimizer, metadata, via_ir),
        })
    }

    ///
    /// A shortcut constructor from source code.
    ///
    /// Only for the integration test purposes.
    ///
    pub fn try_from_sources(
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        output_selection: SolcStandardJsonInputSettingsSelection,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
        metadata: Option<SolcStandardJsonInputSettingsMetadata>,
        via_ir: bool,
    ) -> anyhow::Result<Self> {
        let sources = sources
            .into_par_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();

        Ok(Self {
            language: Language::Solidity,
            sources,
            settings: Settings::new(libraries, output_selection, optimizer, metadata, via_ir),
        })
    }
}
