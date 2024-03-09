#![allow(unused_must_use)]
use std::{io::{self, Write, BufRead, BufWriter}, fs::File, thread::spawn};

pub mod packages;

macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        if !$value.is_empty() {
            if !$value.is_empty() {
                __writeln_backend($handle, $entry, $value);
            }
        }
    };
}

macro_rules! writeln_to_handle_if_not_empty_i16 {
    ($handle:expr, $entry:expr, $value:expr) => {
        if $value != 0 {
            __writeln_backend($handle, $entry, $value.to_string().as_str());
        }
    }
}

#[inline(always)]
fn __writeln_backend(handle: &mut BufWriter<&mut io::StdoutLock<'_>>, entry: &str, value: &str) {
    // format! is slow, so do it manually with a String
    // this code isn't concise but its faster than format! https://stackoverflow.com/questions/63690623/why-is-the-format-macro-slower-than-pushing-into-a-string-directly
    // this code can be changed back to format! if its faster than the current method if https://github.com/rust-lang/rust/issues/99012 is fixed

    let mut output = String::from("\x1B[0;35m   ");
    output.push_str(entry);
    output.push_str("\x1B[0m ~ ");
    output.push_str(value.to_string().as_str());
    output.push('\n');

    handle.write_all(output.as_bytes());
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}

fn main() -> io::Result<()> {
    // this code is all intentional: we are starting the tasks that tend to be the slowest first
    let packages_thread = spawn(|| {
        packages::get_num_packages()
    });

    let header_thread = spawn(|| {
        let usr = get_env_var!("USER");
        let mut result = String::from("\x1B[0;31m\x1B[1mx\x1B[0;36mFetch\x1B[0m - ");

        result.push_str(&usr);
        result.push('\n');

        result
    });

    let distro_thread = spawn(|| {
        let file = File::open("/etc/os-release").expect("Can't open /etc/os-release!");
        let mut reader = io::BufReader::new(file);
        let (mut line, mut pretty_name) = (String::new(), String::new());

        while reader.read_line(&mut line).expect("Failed to read line") > 0 {
            if line.starts_with("PRETTY_NAME=") {
                pretty_name = line.splitn(2, '=').nth(1).unwrap().to_string(); // if clippy tells you to change this code, probably don't, since this is faster in my tests.
                pretty_name = pretty_name.trim().trim_matches('"').to_string();
                break;
            }
            line.clear();
        }
        pretty_name
    });

    let uptime_thread = spawn(|| {
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

    let desktop_thread = spawn(|| {
        get_env_var!("XDG_SESSION_DESKTOP")
    });

    let shell_thread = spawn(|| {
        get_env_var!("SHELL")
    });

    // this code is all intentional: we are collecting the tasks that tend to be the slowest last
    let arch = std::env::consts::ARCH;
    let shell = shell_thread.join().unwrap();
    let desktop = desktop_thread.join().unwrap();
    let uptime = uptime_thread.join().unwrap();
    let distro = distro_thread.join().unwrap();
    let header = header_thread.join().unwrap();
    let pkg = packages_thread.join().unwrap();

    let mut handle = io::stdout().lock(); // lock stdout for slightly faster writing
    let mut writer = BufWriter::new(&mut handle); // buffer it for even faster writing
    // the actual printing
    writer.write_all(header.as_bytes());
    writeln_to_handle_if_not_empty!(&mut writer, "Shell", &shell);
    writeln_to_handle_if_not_empty_i16!(&mut writer, "PKGs", pkg);
    writeln_to_handle_if_not_empty!(&mut writer, "Arch", &arch);
    writeln_to_handle_if_not_empty!(&mut writer, "Uptime", &uptime);
    writeln_to_handle_if_not_empty!(&mut writer, "Distro", &distro);
    writeln_to_handle_if_not_empty!(&mut writer, "Desktop", &desktop);

    Ok(())
}
