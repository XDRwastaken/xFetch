#![allow(unused_must_use)]
use std::{io::{self, Write, BufRead}, fs::File};
use tokio::{task::spawn, join};

pub mod packages;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        if !$value.is_empty() {
            // let to_write = String::new();
            // to_write.push_str("38;2;205;0;205    ");
            // to_write.push_str($entry.purple());
            writeln!($handle, "\x1B[0;35m   {}\x1B[0m ~ {}", $entry, $value);
        }
    };
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let name_thread = spawn(async {
        get_env_var!("USER")
    });

    let distro_thread = spawn(async {
        let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
        let mut reader = io::BufReader::new(file);
        let (mut line, mut pretty_name) = (String::new(), String::new());

        while reader.read_line(&mut line).expect("Failed to read line") > 0 {
            if line.starts_with("PRETTY_NAME=") {
                pretty_name = line.splitn(2, '=').nth(1).unwrap().to_string();
                pretty_name = pretty_name.trim().trim_matches('"').to_string();
                break;
            }
            line.clear();
        }
        pretty_name
    });

    let desktop_thread = spawn(async {
        get_env_var!("XDG_SESSION_DESKTOP")
    });

    let shell_thread = spawn(async {
        get_env_var!("SHELL")
    });

    let packages_thread = spawn(async {
        packages::get_num_packages()
    });

    let uptime_thread = spawn(async {
        match uptime_lib::get() {
            Ok(uptime) => {
                let raw = uptime.as_secs();
                let (days, hrs, mins) = (raw / (60 * 60 * 24),
                                         raw/ (60 * 60) % 24,
                                         raw / 60 % 60);

                let mut formatted_uptime = String::new();

                if days > 0 {
                    formatted_uptime.push_str(&format!("{}d, ", days));
                }
                if hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{}h, ", hrs));
                }
                if mins > 0 || hrs > 0 || days > 0 {
                    formatted_uptime.push_str(&format!("{}m", mins));
                } else {
                    // system uptime is less than a minute. display seconds instead.
                    formatted_uptime.push_str(&format!("{}s", raw));
                }

                formatted_uptime
            }
            Err(_) => String::new(),
        }
    });

    // join! to await all `futures` types concurrently
    let (usr, distro, shell, desktop, pkg, uptime) = join!(
        name_thread,
        distro_thread,
        shell_thread,
        desktop_thread,
        packages_thread,
        uptime_thread
    );

    // and then .unwrap the results. pray that none of them contain an `Err` type & panic! the app
    // that'd be bad lol
    let usr = usr.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let uptime = uptime.unwrap();
    let arch = std::env::consts::ARCH;

    let mut handle = io::stdout().lock(); // lock stdout for slightly faster writing
    // the actual printing
    writeln!(handle, "{}{} - {}", "\x1B[0;31m\x1B[1mx", "\x1B[0;36mFetch\x1B[0m", usr).unwrap();
    writeln_to_handle_if_not_empty!(handle, "Shell", shell);
    if pkg != 0 { // odd one out; too lazy to properly implement this lol
        writeln!(handle, "   {} ~ {}, {}", "\x1B[0;35mPKGs\x1B[0m", pkg, arch).unwrap();
    } else {
        writeln_to_handle_if_not_empty!(handle, "Arch", arch);
    }
    writeln_to_handle_if_not_empty!(handle, "Uptime", uptime);
    writeln_to_handle_if_not_empty!(handle, "Distro", distro);
    writeln_to_handle_if_not_empty!(handle, "Desktop", desktop);

    drop(handle);
    Ok(())
}
