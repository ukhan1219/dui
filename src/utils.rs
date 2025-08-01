// Utility functions for Docker CLI operations

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn validate_container_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Container name cannot be empty".to_string());
    }
    
    if name.len() > 63 {
        return Err("Container name cannot be longer than 63 characters".to_string());
    }
    
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        return Err("Container name can only contain alphanumeric characters, hyphens, underscores, and dots".to_string());
    }
    
    Ok(())
}

pub fn validate_image_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Image name cannot be empty".to_string());
    }
    
    // Basic validation for image names
    if name.contains(' ') {
        return Err("Image name cannot contain spaces".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 8), "hello...");
    }

    #[test]
    fn test_validate_container_name() {
        assert!(validate_container_name("my-container").is_ok());
        assert!(validate_container_name("my_container").is_ok());
        assert!(validate_container_name("my.container").is_ok());
        assert!(validate_container_name("").is_err());
        assert!(validate_container_name("my container").is_err());
    }

    #[test]
    fn test_validate_image_name() {
        assert!(validate_image_name("nginx:latest").is_ok());
        assert!(validate_image_name("my-app").is_ok());
        assert!(validate_image_name("").is_err());
        assert!(validate_image_name("my app").is_err());
    }
}
