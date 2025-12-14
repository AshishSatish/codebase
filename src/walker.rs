// PURPOSE: Directory traversal and file filtering
// - Uses walkdir to recursively scan directories
// - Skips hidden files/folders (starts with .)
// - Filters by file extension (only .rs, .py, .js, .c, .cpp, etc.)

use crate::parser::SupportedLanguage;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileWalker {
    path: PathBuf,
    extensions: Option<Vec<String>>,
}

impl FileWalker {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            extensions: None,
        }
    }

    /// Filter by specific extensions (e.g., ["rs", "py"])
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }

    /// Walk directory and return all matching files
    pub fn walk(&self) -> impl Iterator<Item = PathBuf> {
        let extensions = self.extensions.clone();

        WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|e| {
                // Skip hidden files and directories
                !e.file_name()
                    .to_str()
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .filter(move |path| {
                // Check if file has supported extension
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    // If specific extensions requested, filter by them
                    if let Some(ref exts) = extensions {
                        return exts.iter().any(|e| e == ext);
                    }
                    // Otherwise, check if it's a supported language
                    SupportedLanguage::from_extension(ext).is_some()
                } else {
                    false
                }
            })
    }
}
