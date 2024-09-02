#![allow(unused_must_use)]

use std::io::{self, Write};
use tokio::{join, task::spawn};

pub mod distro;
pub mod packages;
pub mod shell;
pub mod uptime;
#[macro_use]
pub mod macros;

#[tokio::main]
async fn main() -> io::Result<()> {
    let desktop_thread = spawn(async { get_env_var!("XDG_SESSION_DESKTOP") });
    let distro_thread = spawn(async { distro::get_distro_name().await });
    let name_thread = spawn(async { get_env_var!("USER") });
    let packages_thread = spawn(async { packages::get_current_packages() });
    let shell_thread = spawn(async { shell::get_current_shell().await });
    let uptime_thread = spawn(async { uptime::get_uptime().await });

    let (usr, distro, shell, desktop, pkg, uptime) = join!(
        name_thread,
        distro_thread,
        shell_thread,
        desktop_thread,
        packages_thread,
        uptime_thread
    );

    let usr = usr.unwrap();
    let distro = distro.unwrap();
    let shell = shell.unwrap();
    let desktop = desktop.unwrap();
    let pkg = pkg.unwrap();
    let uptime = uptime.unwrap();

    let mut handle = io::stdout().lock(); // Lock stdout for slightly faster writing

    writeln!(handle, "{}{} - {}", "\x1B[0;31m\x1B[1mx", "\x1B[0;36mFetch\x1B[0m", usr).unwrap();
    writeln_to_handle_if_not_empty!(handle, "Shell", shell);
    writeln!(handle, "   {} ~ {}", "\x1B[0;35mPKGs\x1B[0m", pkg).unwrap();
    writeln_to_handle_if_not_empty!(handle, "Uptime", uptime);
    writeln_to_handle_if_not_empty!(handle, "Distro", distro);
    writeln_to_handle_if_not_empty!(handle, "Desktop", desktop);

    drop(handle);
    Ok(())
}
