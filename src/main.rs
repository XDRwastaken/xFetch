#![allow(unused_must_use)]
//#![deny(unsafe_code)]

use std::{io::{self, Write, BufRead}, fs::File};
use tokio::{task::spawn, join};
use sysinfo_dot_h::try_collect;

pub mod packages;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        if !$value.is_empty() {
            writeln!($handle, "\x1B[0;35m   {}\x1B[0m ~ {}", $entry, $value);
        }
    };
}

macro_rules! writeln_to_handle_if_not_empty_i16 {
    ($handle:expr, $entry:expr, $value:expr) => {
        if $value.parse::<i16>().unwrap() != 0 {
            writeln!($handle, "\x1B[0;35m   {}\x1B[0m ~ {}", $entry, $value).unwrap();
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
        let info = try_collect().unwrap();
        
    let (days, hrs, mins) = (info.uptime as f64 / (60.0 * 60.0 * 24.0),
                             (info.uptime as f64 / (60.0 * 60.0)) % 24.0,
                             (info.uptime as f64 / 60.0) % 60.0);
    
        let formatted_uptime = format!("{}d, {}h, {}m", days, hrs, mins);
        formatted_uptime
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
    writeln_to_handle_if_not_empty_i16!(handle, "Uptime", uptime);
    writeln_to_handle_if_not_empty!(handle, "Distro", distro);
    writeln_to_handle_if_not_empty!(handle, "Desktop", desktop);

    drop(handle);
    Ok(())
}
