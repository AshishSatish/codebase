// PURPOSE: Query pattern matching engine
// - Defines QueryPattern struct (name, tree-sitter query string, keywords)
// - Maps natural language ("find functions") to tree-sitter queries
// - Stores patterns for functions, structs, imports across all 5 languages

use crate::parser::SupportedLanguage;
use anyhow::{anyhow, Result};
use tree_sitter::{Query, QueryCursor};

pub struct QueryPattern {
    pub name: &'static str,
    pub query_str: &'static str,
    pub keywords: &'static [&'static str],
    pub language: SupportedLanguage,
}

impl QueryPattern {
    /// Check if user query matches this pattern's keywords
    pub fn matches(&self, user_query: &str) -> bool {
        let query_lower = user_query.to_lowercase();
        self.keywords
            .iter()
            .any(|keyword| query_lower.contains(keyword))
    }
}

pub struct QueryEngine {
    patterns: Vec<QueryPattern>,
}

impl QueryEngine {
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    /// Define default patterns for common searches
    fn default_patterns() -> Vec<QueryPattern> {
        vec![
            // Rust function patterns
            QueryPattern {
                name: "Rust Functions",
                query_str: "(function_item) @function",
                keywords: &["function", "fn", "func", "method"],
                language: SupportedLanguage::Rust,
            },
            // Rust struct patterns
            QueryPattern {
                name: "Rust Structs",
                query_str: "(struct_item) @struct",
                keywords: &["struct", "structure"],
                language: SupportedLanguage::Rust,
            },
            // Rust enums
            QueryPattern {
                name: "Rust Enums",
                query_str: "(enum_item) @enum",
                keywords: &["enum"],
                language: SupportedLanguage::Rust,
            },
            // Rust traits
            QueryPattern {
                name: "Rust Traits",
                query_str: "(trait_item) @trait",
                keywords: &["trait", "interface"],
                language: SupportedLanguage::Rust,
            },
            // Rust impl blocks
            QueryPattern {
                name: "Rust Implementations",
                query_str: "(impl_item) @impl",
                keywords: &["impl", "implementation"],
                language: SupportedLanguage::Rust,
            },
            // Rust constants
            QueryPattern {
                name: "Rust Constants",
                query_str: "(const_item) @const",
                keywords: &["const", "constant"],
                language: SupportedLanguage::Rust,
            },
            // Rust imports
            QueryPattern {
                name: "Rust Imports",
                query_str: "(use_declaration) @import",
                keywords: &["import", "use", "dependency"],
                language: SupportedLanguage::Rust,
            },
            // Python function patterns
            QueryPattern {
                name: "Python Functions",
                query_str: "(function_definition) @function",
                keywords: &["function", "fn", "func", "def", "method"],
                language: SupportedLanguage::Python,
            },
            // Python class patterns
            QueryPattern {
                name: "Python Classes",
                query_str: "(class_definition) @class",
                keywords: &["class", "type"],
                language: SupportedLanguage::Python,
            },
            // Python imports
            QueryPattern {
                name: "Python Imports",
                query_str: "(import_statement) @import",
                keywords: &["import", "use", "dependency"],
                language: SupportedLanguage::Python,
            },
            // Python decorators
            QueryPattern {
                name: "Python Decorators",
                query_str: "(decorator) @decorator",
                keywords: &["decorator"],
                language: SupportedLanguage::Python,
            },
            // JavaScript function patterns
            QueryPattern {
                name: "JavaScript Functions",
                query_str: "(function_declaration) @function",
                keywords: &["function", "fn", "func", "method"],
                language: SupportedLanguage::JavaScript,
            },
            // JavaScript class patterns
            QueryPattern {
                name: "JavaScript Classes",
                query_str: "(class_declaration) @class",
                keywords: &["class", "type"],
                language: SupportedLanguage::JavaScript,
            },
            // JavaScript arrow functions
            QueryPattern {
                name: "JavaScript Arrow Functions",
                query_str: "(arrow_function) @function",
                keywords: &["arrow", "function", "fn", "func"],
                language: SupportedLanguage::JavaScript,
            },
            // JavaScript variable declarations
            QueryPattern {
                name: "JavaScript Variables",
                query_str: "(variable_declaration) @variable",
                keywords: &["variable", "var", "let", "const"],
                language: SupportedLanguage::JavaScript,
            },
            // JavaScript imports
            QueryPattern {
                name: "JavaScript Imports",
                query_str: "(import_statement) @import",
                keywords: &["import", "use", "dependency"],
                language: SupportedLanguage::JavaScript,
            },
            // JavaScript exports
            QueryPattern {
                name: "JavaScript Exports",
                query_str: "(export_statement) @export",
                keywords: &["export"],
                language: SupportedLanguage::JavaScript,
            },
            // TypeScript function patterns
            QueryPattern {
                name: "TypeScript Functions",
                query_str: "(function_declaration) @function",
                keywords: &["function", "fn", "func", "method"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript arrow functions
            QueryPattern {
                name: "TypeScript Arrow Functions",
                query_str: "(arrow_function) @function",
                keywords: &["arrow", "function", "fn", "func"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript class patterns
            QueryPattern {
                name: "TypeScript Classes",
                query_str: "(class_declaration) @class",
                keywords: &["class", "type"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript interfaces
            QueryPattern {
                name: "TypeScript Interfaces",
                query_str: "(interface_declaration) @interface",
                keywords: &["interface"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript type aliases
            QueryPattern {
                name: "TypeScript Type Aliases",
                query_str: "(type_alias_declaration) @type",
                keywords: &["type", "alias"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript enums
            QueryPattern {
                name: "TypeScript Enums",
                query_str: "(enum_declaration) @enum",
                keywords: &["enum"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript imports
            QueryPattern {
                name: "TypeScript Imports",
                query_str: "(import_statement) @import",
                keywords: &["import", "use", "dependency"],
                language: SupportedLanguage::TypeScript,
            },
            // TypeScript exports
            QueryPattern {
                name: "TypeScript Exports",
                query_str: "(export_statement) @export",
                keywords: &["export"],
                language: SupportedLanguage::TypeScript,
            },
            // C function patterns
            QueryPattern {
                name: "C Functions",
                query_str: "(function_definition) @function",
                keywords: &["function", "fn", "func", "method"],
                language: SupportedLanguage::C,
            },
            // C struct patterns
            QueryPattern {
                name: "C Structs",
                query_str: "(struct_specifier) @struct",
                keywords: &["struct", "structure"],
                language: SupportedLanguage::C,
            },
            // C enums
            QueryPattern {
                name: "C Enums",
                query_str: "(enum_specifier) @enum",
                keywords: &["enum"],
                language: SupportedLanguage::C,
            },
            // C typedefs
            QueryPattern {
                name: "C Typedefs",
                query_str: "(type_definition) @typedef",
                keywords: &["typedef", "type"],
                language: SupportedLanguage::C,
            },
            // C++ function patterns
            QueryPattern {
                name: "C++ Functions",
                query_str: "(function_definition) @function",
                keywords: &["function", "fn", "func", "method"],
                language: SupportedLanguage::Cpp,
            },
            // C++ class patterns
            QueryPattern {
                name: "C++ Classes",
                query_str: "(class_specifier) @class",
                keywords: &["class", "type"],
                language: SupportedLanguage::Cpp,
            },
            // C++ struct patterns
            QueryPattern {
                name: "C++ Structs",
                query_str: "(struct_specifier) @struct",
                keywords: &["struct", "structure"],
                language: SupportedLanguage::Cpp,
            },
            // C++ enums
            QueryPattern {
                name: "C++ Enums",
                query_str: "(enum_specifier) @enum",
                keywords: &["enum"],
                language: SupportedLanguage::Cpp,
            },
            // C++ namespaces
            QueryPattern {
                name: "C++ Namespaces",
                query_str: "(namespace_definition) @namespace",
                keywords: &["namespace"],
                language: SupportedLanguage::Cpp,
            },
            // C++ templates
            QueryPattern {
                name: "C++ Templates",
                query_str: "(template_declaration) @template",
                keywords: &["template"],
                language: SupportedLanguage::Cpp,
            },
        ]
    }

    /// Find patterns matching user query for a specific language
    pub fn find_patterns(
        &self,
        user_query: &str,
        language: SupportedLanguage,
    ) -> Vec<&QueryPattern> {
        self.patterns
            .iter()
            .filter(|p| p.language == language && p.matches(user_query))
            .collect()
    }

    /// Execute a pattern query on a tree
    pub fn execute_query(
        &self,
        pattern: &QueryPattern,
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> Result<Vec<QueryMatch>> {
        let language = pattern.language.tree_sitter_language();
        let query = Query::new(&language, pattern.query_str)
            .map_err(|e| anyhow!("Failed to create query: {}", e))?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

        let mut results = Vec::new();
        for match_ in matches {
            for capture in match_.captures {
                let node = capture.node;
                let start = node.start_position();
                let text = node.utf8_text(source.as_bytes())?;

                results.push(QueryMatch {
                    line: start.row + 1,
                    column: start.column + 1,
                    text: text.to_string(),
                });
            }
        }

        Ok(results)
    }
}

#[derive(Debug)]
pub struct QueryMatch {
    pub line: usize,
    pub column: usize,
    pub text: String,
}
