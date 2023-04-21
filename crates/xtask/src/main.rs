use std::{env, path::PathBuf};

use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    match &args[..] {
        ["build"] => build_target(),
        ["check"] => check_target(),
        ["flash"] => flash_target(),
        _ => {
            println!("USAGE cargo xtask [build|check]");
            Ok(())
        }
    }
}

fn build_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo build").run()?;
    Ok(())
}

fn check_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo check").run()?;
    Ok(())
}

fn flash_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "espflash /dev/tty.usbserial-0001 target/xtensa-esp32-espidf/debug/cross_compiled --flash-freq 80M --flash-size 4MB --flash-mode DIO --speed 921600").run()?;
    Ok(())
}
