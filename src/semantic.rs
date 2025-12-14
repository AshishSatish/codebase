// PURPOSE: Semantic/implementation-based code search
// - Extracts function metadata (name, comments, body)
// - Performs keyword matching and scoring
// - Ranks results by relevance to user query

use std::collections::HashSet;

/// Represents a function with its metadata for semantic search
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub full_text: String,
    pub line: usize,
    pub column: usize,
}

/// Result of semantic search with relevance score
#[derive(Debug, Clone)]
pub struct SemanticMatch {
    pub line: usize,
    pub column: usize,
    pub text: String,
    pub score: f32,
    pub match_reasons: Vec<String>,
}

/// Semantic search engine for implementation-based queries
pub struct SemanticSearchEngine;

impl SemanticSearchEngine {
    pub fn new() -> Self {
        Self
    }

    /// Extract keywords from a natural language query
    /// Example: "function that checks if number is prime" -> ["check", "number", "prime"]
    pub fn extract_keywords(&self, query: &str) -> Vec<String> {
        // Common stop words to ignore
        let stop_words: HashSet<&str> = [
            "a", "an", "the", "that", "which", "who", "is", "are", "was", "were",
            "be", "been", "being", "have", "has", "had", "do", "does", "did",
            "will", "would", "should", "could", "may", "might", "must",
            "function", "functions", "method", "methods", "class", "classes",
            "if", "of", "to", "for", "in", "on", "at", "by", "with",
        ]
        .iter()
        .cloned()
        .collect();

        query
            .to_lowercase()
            .split_whitespace()
            .filter(|word| !stop_words.contains(word))
            .filter(|word| word.len() > 2) // Ignore very short words
            .map(|word| word.to_string())
            .collect()
    }

    /// Score a function based on how well it matches the query keywords
    pub fn score_function(&self, func: &FunctionMetadata, keywords: &[String]) -> (f32, Vec<String>) {
        let mut score = 0.0;
        let mut match_reasons = Vec::new();

        let func_lower = func.full_text.to_lowercase();
        let name_lower = func.name.to_lowercase();

        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            // Check function name (highest weight)
            if name_lower.contains(&keyword_lower) {
                score += 10.0;
                match_reasons.push(format!("'{}' in function name", keyword));
            }

            // Check full text (lower weight)
            if func_lower.contains(&keyword_lower) {
                score += 2.0;
                if !match_reasons.iter().any(|r| r.contains(keyword)) {
                    match_reasons.push(format!("'{}' in function body", keyword));
                }
            }

            // Bonus for exact name match
            if name_lower == keyword_lower {
                score += 15.0;
            }
        }

        // Bonus for matching multiple keywords
        if match_reasons.len() > 1 {
            score += match_reasons.len() as f32 * 1.5;
        }

        (score, match_reasons)
    }

    /// Search and rank functions based on semantic relevance
    pub fn search(
        &self,
        functions: Vec<FunctionMetadata>,
        query: &str,
    ) -> Vec<SemanticMatch> {
        let keywords = self.extract_keywords(query);

        if keywords.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<SemanticMatch> = functions
            .iter()
            .filter_map(|func| {
                let (score, match_reasons) = self.score_function(func, &keywords);

                // Only return matches with score > 0
                if score > 0.0 {
                    Some(SemanticMatch {
                        line: func.line,
                        column: func.column,
                        text: func.full_text.clone(),
                        score,
                        match_reasons,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (highest first)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }

    /// Extract function name from tree-sitter node text
    /// This is a simple heuristic parser
    pub fn extract_function_name(&self, text: &str) -> String {
        // Try to find function name after "fn", "def", "function", etc.
        let text = text.trim();

        // Rust: fn name(...
        if let Some(idx) = text.find("fn ") {
            let after_fn = &text[idx + 3..];
            if let Some(paren) = after_fn.find('(') {
                return after_fn[..paren].trim().to_string();
            }
        }

        // Python: def name(...
        if let Some(idx) = text.find("def ") {
            let after_def = &text[idx + 4..];
            if let Some(paren) = after_def.find('(') {
                return after_def[..paren].trim().to_string();
            }
        }

        // JavaScript/TypeScript: function name(...
        if let Some(idx) = text.find("function ") {
            let after_fn = &text[idx + 9..];
            if let Some(paren) = after_fn.find('(') {
                return after_fn[..paren].trim().to_string();
            }
        }

        // C/C++: type name(...
        if let Some(paren) = text.find('(') {
            let before_paren = &text[..paren];
            if let Some(last_space) = before_paren.rfind(|c: char| c.is_whitespace() || c == '*') {
                return before_paren[last_space + 1..].trim().to_string();
            }
        }

        // Fallback: use first line
        text.lines()
            .next()
            .unwrap_or("unknown")
            .chars()
            .take(50)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let engine = SemanticSearchEngine::new();
        let keywords = engine.extract_keywords("function that checks if number is prime");
        assert!(keywords.contains(&"checks".to_string()));
        assert!(keywords.contains(&"number".to_string()));
        assert!(keywords.contains(&"prime".to_string()));
        assert!(!keywords.contains(&"function".to_string())); // stop word
    }

    #[test]
    fn test_score_function() {
        let engine = SemanticSearchEngine::new();
        let func = FunctionMetadata {
            name: "isPrime".to_string(),
            full_text: "fn isPrime(n: u64) -> bool { ... }".to_string(),
            line: 10,
            column: 1,
        };

        let keywords = vec!["prime".to_string(), "check".to_string()];
        let (score, reasons) = engine.score_function(&func, &keywords);

        assert!(score > 0.0);
        assert!(!reasons.is_empty());
    }
}
