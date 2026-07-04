use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileCategory {
    Executable,
    Installer,
    ShellScript,
    PowerShell,
    BatchFile,
    PythonScript,
    JavaArchive,
    OfficeMacro,
    Pdf,
    Archive,
    SharedLibrary,
    AppBundle,
    DiskImage,
    ApplicationPackage,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub file_name: String,
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: u64,
    pub category: FileCategory,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FileMetadata {
    pub fn from_path(path: PathBuf) -> Self {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        let category = classify_by_extension(extension.as_deref());
        Self {
            path: path.clone(),
            file_name,
            extension,
            mime_type: None,
            size_bytes: 0,
            category,
            created_at: None,
            modified_at: None,
        }
    }
}

pub fn classify_by_extension(ext: Option<&str>) -> FileCategory {
    match ext {
        Some("exe") | Some("com") | Some("scr") => FileCategory::Executable,
        Some("msi") | Some("msix") | Some("pkg") | Some("deb") | Some("rpm") => {
            FileCategory::Installer
        }
        Some("sh") | Some("bash") | Some("zsh") => FileCategory::ShellScript,
        Some("ps1") | Some("psm1") => FileCategory::PowerShell,
        Some("bat") | Some("cmd") => FileCategory::BatchFile,
        Some("py") | Some("pyc") | Some("pyw") => FileCategory::PythonScript,
        Some("jar") | Some("war") | Some("ear") => FileCategory::JavaArchive,
        Some("doc") | Some("docm") | Some("xls") | Some("xlsm") | Some("ppt")
        | Some("pptm") => FileCategory::OfficeMacro,
        Some("pdf") => FileCategory::Pdf,
        Some("zip") | Some("rar") | Some("7z") | Some("tar") | Some("gz") => FileCategory::Archive,
        Some("so") | Some("dll") | Some("dylib") => FileCategory::SharedLibrary,
        Some("app") => FileCategory::AppBundle,
        Some("dmg") | Some("iso") | Some("img") => FileCategory::DiskImage,
        Some("apk") | Some("ipa") => FileCategory::ApplicationPackage,
        _ => FileCategory::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_powershell() {
        assert_eq!(
            classify_by_extension(Some("ps1")),
            FileCategory::PowerShell
        );
    }
}
