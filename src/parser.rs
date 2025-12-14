// PURPOSE: Language detection and tree-sitter parsing wrapper
// - Detects language from file extensions (.rs → Rust, .py → Python, etc.)
// - Initializes correct tree-sitter parser for each language
// - Provides parse_file() method to parse source code into AST

use anyhow::{anyhow, Result};
use std::path::Path;
use tree_sitter::{Language, Parser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    C,
    Cpp,
}

impl SupportedLanguage {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "py" => Some(Self::Python),
            "js" | "jsx" => Some(Self::JavaScript),
            "ts" | "tsx" => Some(Self::TypeScript),
            "c" | "h" => Some(Self::C),
            "cpp" | "hpp" | "cc" | "cxx" | "hh" => Some(Self::Cpp),
            _ => None,
        }
    }

    /// Get the tree-sitter Language for this language
    pub fn tree_sitter_language(&self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::language(),
            Self::Python => tree_sitter_python::language(),
            Self::JavaScript => tree_sitter_javascript::language(),
            Self::TypeScript => tree_sitter_typescript::language_typescript(),
            Self::C => tree_sitter_c::language(),
            Self::Cpp => tree_sitter_cpp::language(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::C => "C",
            Self::Cpp => "C++",
        }
    }
}

pub struct CodeParser {
    parser: Parser,
}

impl CodeParser {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    /// Parse source code with the appropriate language parser
    pub fn parse(
        &mut self,
        source: &str,
        language: SupportedLanguage,
    ) -> Result<tree_sitter::Tree> {
        self.parser
            .set_language(&language.tree_sitter_language())
            .map_err(|e| anyhow!("Failed to set language: {}", e))?;

        self.parser
            .parse(source, None)
            .ok_or_else(|| anyhow!("Failed to parse source code"))
    }

    /// Detect language and parse file
    pub fn parse_file(&mut self, path: &Path, source: &str) -> Result<tree_sitter::Tree> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("No file extension"))?;

        let language = SupportedLanguage::from_extension(ext)
            .ok_or_else(|| anyhow!("Unsupported file extension: {}", ext))?;

        self.parse(source, language)
    }
}
