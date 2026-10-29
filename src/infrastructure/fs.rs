
use std::fs;
use std::path::Path;
use crate::domain::errors::VerifyError;

/// Reads raw XDR data from a file path.
/// 
/// Returns a `VerifyError::Io` if the file cannot be found, read, or permissions are denied.
/// Returns `VerifyError::MissingEvidence` if the file is empty.
pub fn read_xdr_file<P: AsRef<Path>>(path: P) -> Result<String, VerifyError> {
    let content = fs::read_to_string(&path)
        .map_err(VerifyError::Io)?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(VerifyError::MissingEvidence(format!(
            "File is empty: {}",
            path.as_ref().display()
        )));
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "AAAAAgAAA...").unwrap();
        
        let result = read_xdr_file(file.path()).unwrap();
        assert_eq!(result, "AAAAAgAAA...");
    }

    #[test]
    fn test_read_empty_file_returns_missing_evidence() {
        let file = NamedTempFile::new().unwrap();
        
        let err = read_xdr_file(file.path()).unwrap_err();
        assert!(matches!(err, VerifyError::MissingEvidence(_)));
    }

    #[test]
    fn test_read_nonexistent_file_returns_io_error() {
        let err = read_xdr_file("does_not_exist.xdr").unwrap_err();
        assert!(matches!(err, VerifyError::Io(_)));
    }
}