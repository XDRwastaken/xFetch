use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn get_distro_name() -> String {
    let file = File::open("/etc/os-release").await.expect("Can't find the file \"etc/os-release\"");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let pretty_name = String::new();

    while reader
        .read_line(&mut line)
        .await
        .expect("Failed to read line")
        > 0
    {
        if let Some(pretty_name) = line.strip_prefix("PRETTY_NAME=") {
            return pretty_name.trim().trim_matches('"').to_string();
        }
        line.clear();
    }
    pretty_name
}
