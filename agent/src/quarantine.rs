use std::fs;
use std::path::{Path, PathBuf};

/// Gets the quarantine directory path
fn get_quarantine_dir() -> PathBuf {
    std::env::temp_dir().join("simware_quarantine")
}

/// Securely moves a blocked file to the quarantine vault.
pub fn quarantine_file(original_path: &str) -> std::io::Result<()> {
    let source_path = Path::new(original_path);
    
    if !source_path.exists() {
        return Ok(()); // File may have already been deleted or moved
    }

    let quarantine_dir = get_quarantine_dir();
    
    // Ensure quarantine directory exists
    if !quarantine_dir.exists() {
        fs::create_dir_all(&quarantine_dir)?;
    }

    let file_name = source_path.file_name().unwrap_or_default();
    
    // Create a unique destination path to prevent overwrites
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
        
    let dest_name = format!("{}_{}", timestamp, file_name.to_string_lossy());
    let dest_path = quarantine_dir.join(dest_name);

    // Move the file (effectively deleting it from the original location)
    fs::rename(source_path, &dest_path)?;
    
    // Strip execution permissions from the quarantined file
    let mut perms = fs::metadata(&dest_path)?.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(0o600); // Read/Write only for owner, no execution
    }
    fs::set_permissions(&dest_path, perms)?;

    println!("[Quarantine] Successfully moved {} to {}", original_path, dest_path.display());
    Ok(())
}
