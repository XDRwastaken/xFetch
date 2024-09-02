use std::{fs, path::Path};

const PACMAN_DIR: &str = "/var/lib/pacman/local";

pub fn get_current_packages() -> i16 {
    fs::read_dir(Path::new(PACMAN_DIR))
        .map(|entries| entries.count())
        .unwrap_or_default()
        .try_into()
        .unwrap()
}
