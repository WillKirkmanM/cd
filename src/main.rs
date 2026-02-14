use std::env;
use std::path::PathBuf;

/// Changes the current working directory.
/// 
/// Supports:
/// - Absolute paths: `/tmp`
/// - Relative paths: `../music`
/// - Home alias: `~` or `~/Documents`
/// - Empty input: defaults to HOME
fn cd(path: Option<&str>) -> std::io::Result<()> {
    // 1. Determine the Home directory (defaults to "/" if HOME not set)
    // In a real app, you might use the `dirs` crate for cross-platform safety.
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    // 2. Resolve the target path
    let target = match path {
        // Case A: "cd" (No argument) -> Go Home
        None => PathBuf::from(&home),
        
        Some(path_str) => {
            // Case B: "cd ~" -> Go Home
            if path_str == "~" {
                PathBuf::from(&home)
            } 
            // Case C: "cd ~/Documents" -> Replace ~ with Home path
            else if path_str.starts_with("~/") {
                let mut path_buf = PathBuf::from(&home);
                // Push the rest of the path (skipping "~/")
                path_buf.push(&path_str[2..]); 
                path_buf
            } 
            // Case D: Standard path (Relative or Absolute)
            else {
                PathBuf::from(path_str)
            }
        }
    };

    // 3. Attempt to change directory
    env::set_current_dir(&target)?;

    Ok(())
}

fn main() {
    // --- DEMO USAGE ---
    
    // Print starting directory
    println!("Start: {:?}", env::current_dir().unwrap());

    // 1. Change to parent
    if let Err(e) = cd(Some("..")) {
        eprintln!("Error: {}", e);
    }
    println!("After '..': {:?}", env::current_dir().unwrap());

    // 2. Change to Home (using ~ expansion)
    // Note: This relies on your actual HOME env var being set
    if let Err(e) = cd(Some("~")) {
        eprintln!("Error: {}", e);
    }
    println!("After '~': {:?}", env::current_dir().unwrap());
}