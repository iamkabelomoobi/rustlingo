use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file: {}", path.display()))
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create output directory: {}", parent.display())
            })?;
        }
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write output file: {}", path.display()))
}

pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

pub fn get_file_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)
        .with_context(|| format!("Failed to get file metadata: {}", path.display()))?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_read_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        let content = read_file(&file_path).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("output.txt");

        write_file(&file_path, "Test content").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Test content");
    }

    #[test]
    fn test_write_file_with_nested_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nested").join("dirs").join("file.txt");

        write_file(&file_path, "Nested content").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Nested content");
    }

    #[test]
    fn test_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("exists.txt");

        assert!(!file_exists(&file_path));

        fs::File::create(&file_path).unwrap();
        assert!(file_exists(&file_path));
    }

    #[test]
    fn test_get_file_size() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("size_test.txt");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"12345").unwrap();

        let size = get_file_size(&file_path).unwrap();
        assert_eq!(size, 5);
    }
}
