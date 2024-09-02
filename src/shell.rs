use std::env;
use std::path::Path;

pub async fn get_current_shell() -> String {
    let shell = env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
    let path = Path::new(&shell);
    let name = path.file_name().unwrap_or_else(|| path.as_os_str()).to_string_lossy();
    name.to_string()
}
