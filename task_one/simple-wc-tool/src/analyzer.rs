use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct FileStats {
    pub words: usize,
    pub lines: usize,
    pub chars: usize,
}

/// Analyzes a file and returns statistics about its content.
///
/// Note: Could be optimized using Rayon for large files parallel processing,
/// but it would be an overkill for a test task. Also, I haven't got enough time to analyze if it's worth it.
pub fn analyze_file(path: &Path) -> Result<FileStats> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    let mut stats = FileStats {
        words: 0,
        lines: 0,
        chars: 0,
    };

    let mut is_first_line = true;

    for line in reader.lines() {
        let line =
            line.with_context(|| format!("Failed to read line from file: {}", path.display()))?;

        if !is_first_line {
            stats.chars += 1;
        }
        is_first_line = false;

        stats.lines += 1;
        stats.words += count_words(&line);
        stats.chars += line.chars().count();
    }

    Ok(stats)
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{}", content)?;
        Ok(file)
    }

    #[test]
    fn test_empty_file() -> Result<()> {
        let file = create_temp_file("")?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 0,
                lines: 0,
                chars: 0,
            }
        );
        Ok(())
    }

    #[test]
    fn test_single_word() -> Result<()> {
        let file = create_temp_file("hello")?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 1,
                lines: 1,
                chars: 5,
            }
        );
        Ok(())
    }

    #[test]
    fn test_multiple_words_single_line() -> Result<()> {
        let file = create_temp_file("hello world rust")?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 3,
                lines: 1,
                chars: 16,
            }
        );
        Ok(())
    }

    #[test]
    fn test_multiple_lines() -> Result<()> {
        let content = "hello world\nrust is great\nthird line";
        let file = create_temp_file(content)?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 7,
                lines: 3,
                chars: 36,
            }
        );
        Ok(())
    }

    #[test]
    fn test_extra_whitespace() -> Result<()> {
        let content = "  hello   world  \n  rust  ";
        let file = create_temp_file(content)?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 3,
                lines: 2,
                chars: 26,
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonexistent_file() {
        let result = analyze_file(Path::new("nonexistent.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_newlines() -> Result<()> {
        let content = "hello\n\nworld\n\nrust";
        let file = create_temp_file(content)?;
        let stats = analyze_file(file.path())?;

        assert_eq!(
            stats,
            FileStats {
                words: 3,
                lines: 5,
                chars: 18,
            }
        );
        Ok(())
    }
}
