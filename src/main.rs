// PURPOSE: CLI orchestration
// - Uses clap to parse arguments (query, --path, --ext flags)
// - Coordinates: walker → parser → query execution → formatted output
// - Displays results with colored output (file paths in blue, code in white)

mod parser;
mod query;
mod semantic;
mod walker;

use anyhow::Result;
use clap::Parser as ClapParser;
use colored::Colorize;
use parser::{CodeParser, SupportedLanguage};
use query::QueryEngine;
use semantic::{FunctionMetadata, SemanticSearchEngine};
use std::fs;
use std::path::PathBuf;
use walker::FileWalker;

#[derive(ClapParser)]
#[command(name = "codebase")]
#[command(about = "Semantic code search tool for offline use", long_about = None)]
struct Cli {
    /// Natural language query (e.g., "find functions", "show structs")
    query: String,

    /// Path to search (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Filter by specific file extensions (e.g., "rs,py,js")
    #[arg(short, long)]
    ext: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Number of context lines to show before and after each match
    #[arg(short = 'C', long, default_value = "0")]
    context: usize,

    /// Enable implementation-based search (semantic search by functionality)
    #[arg(short = 'i', long)]
    implementation: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Parse extensions if provided
    let extensions = cli.ext.map(|e| {
        e.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>()
    });

    // Initialize components
    let mut code_parser = CodeParser::new();
    let query_engine = QueryEngine::new();
    let semantic_engine = SemanticSearchEngine::new();

    // Set up file walker
    let walker = if let Some(exts) = extensions {
        FileWalker::new(cli.path).with_extensions(exts)
    } else {
        FileWalker::new(cli.path)
    };

    let mut total_matches = 0;
    let mut files_searched = 0;

    // Walk through files
    for file_path in walker.walk() {
        files_searched += 1;

        // Read file contents
        let source = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Detect language
        let language = match file_path
            .extension()
            .and_then(|e| e.to_str())
            .and_then(SupportedLanguage::from_extension)
        {
            Some(lang) => lang,
            None => continue,
        };

        // Parse file
        let tree = match code_parser.parse_file(&file_path, &source) {
            Ok(t) => t,
            Err(e) => {
                if cli.verbose {
                    eprintln!("Failed to parse {}: {}", file_path.display(), e);
                }
                continue;
            }
        };

        // Choose search mode: implementation-based or pattern-based
        if cli.implementation {
            // Semantic/implementation-based search
            // Extract all functions from the file
            let patterns = query_engine.find_patterns("find functions", language);

            if patterns.is_empty() {
                continue;
            }

            let mut functions = Vec::new();
            for pattern in patterns {
                if let Ok(matches) = query_engine.execute_query(pattern, &tree, &source) {
                    for match_ in matches {
                        let name = semantic_engine.extract_function_name(&match_.text);
                        functions.push(FunctionMetadata {
                            name,
                            full_text: match_.text.clone(),
                            line: match_.line,
                            column: match_.column,
                        });
                    }
                }
            }

            // Search and rank by semantic relevance
            let semantic_matches = semantic_engine.search(functions, &cli.query);

            if !semantic_matches.is_empty() {
                // Print file path (blue)
                println!("\n{}", file_path.display().to_string().blue().bold());

                for semantic_match in semantic_matches {
                    total_matches += 1;

                    // Print with relevance score
                    println!("\n  {}:{} {} {}",
                        semantic_match.line,
                        semantic_match.column,
                        "→".blue().bold(),
                        format!("[score: {:.1}]", semantic_match.score).yellow()
                    );

                    // Print first line of function
                    let first_line = semantic_match.text.lines().next().unwrap_or(&semantic_match.text);
                    println!("  {}", first_line.trim().white().bold());

                    // Print match reasons
                    if cli.verbose && !semantic_match.match_reasons.is_empty() {
                        println!("  {} {}",
                            "Reasons:".green(),
                            semantic_match.match_reasons.join(", ")
                        );
                    }
                }
            }
        } else {
            // Pattern-based search (original functionality)
            let patterns = query_engine.find_patterns(&cli.query, language);

            if patterns.is_empty() {
                continue;
            }

            // Execute queries
            for pattern in patterns {
                let matches = match query_engine.execute_query(pattern, &tree, &source) {
                    Ok(m) => m,
                    Err(e) => {
                        if cli.verbose {
                            eprintln!("Query failed for {}: {}", file_path.display(), e);
                        }
                        continue;
                    }
                };

                if !matches.is_empty() {
                    // Print file path (blue)
                    println!("\n{}", file_path.display().to_string().blue().bold());

                    for match_ in matches {
                        total_matches += 1;

                        if cli.context > 0 {
                            // Show context lines
                            let source_lines: Vec<&str> = source.lines().collect();
                            let match_line_idx = match_.line.saturating_sub(1);
                            let start_line = match_line_idx.saturating_sub(cli.context);
                            let end_line = (match_line_idx + cli.context + 1).min(source_lines.len());

                            println!("\n  {}:{}", match_.line, match_.column);
                            for (idx, line) in source_lines[start_line..end_line].iter().enumerate() {
                                let line_num = start_line + idx + 1;
                                if line_num == match_.line {
                                    // Highlight the match line
                                    println!("  {} {} {}", line_num, "→".blue().bold(), line.white().bold());
                                } else {
                                    println!("  {} {} {}", line_num, " ", line);
                                }
                            }
                        } else {
                            // Print line:col
                            print!("  {}:{} → ", match_.line, match_.column);

                            // Print code snippet (white, first line only for readability)
                            let first_line = match_.text.lines().next().unwrap_or(&match_.text);
                            println!("{}", first_line.trim().white());
                        }
                    }
                }
            }
        }
    }

    // Print summary
    println!(
        "\n{} {} matches in {} files",
        "Found".green().bold(),
        total_matches,
        files_searched
    );

    Ok(())
}
