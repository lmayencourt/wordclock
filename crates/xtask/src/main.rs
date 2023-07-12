use std::{env, io::Write};
use std::fs::File;

use anyhow::anyhow;
use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    if args.len() == 0 {
        usage();
        return Err(anyhow!("No argument provided"));
    }

    match args[0] {
        "build" => build_target(&args[1..]),
        "check" => check_target(&args[1..]),
        "clean" => clean_target(),
        "flash" => flash_target(&args[1..]),
        "doc" => doc_target(),
        "uml" => generate_uml_images(),
        "generate_ota" => generate_ota_image(&args[1..]),
        _ => {
            usage();
            Ok(())
        }
    }
}

fn usage() {
    println!("USAGE cargo xtask [build|check|clean|flash|doc|uml|generate_ota]");
}

fn build_target(args: &[&str]) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo build {args...}").run()?;
    Ok(())
}

fn check_target(args: &[&str]) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo check {args...}").run()?;
    Ok(())
}

fn clean_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo clean").run()?;
    Ok(())
}

fn flash_target(args: &[&str]) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "espflash /dev/tty.usbserial-0001 target/xtensa-esp32-espidf/debug/cross_compiled --flash-freq 80M --flash-size 4MB --flash-mode DIO --speed 921600 --partition-table esp32_ota_partitions.csv {args...}").run()?;

    Ok(())
}

fn doc_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo doc --open").run()?;
    Ok(())
}

fn generate_uml_images() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("doc/uml");

    let mut output_dir = sh.current_dir();
    output_dir.push("exported");
    cmd!(sh, "rm -rf exported").run()?;
    cmd!(sh, "plantuml -png **.puml -o {output_dir}").run()?;

    Ok(())
}

fn generate_ota_image(args: &[&str]) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    let git_version = cmd!(sh, "git describe").read()?;
    println!("Releasing firmware: {:?}", git_version);

    let build_type: &str;
    match &args[..] {
        ["release"] => build_type = "release",
        ["debug"] => build_type = "debug",
        _ => {
            return Err(anyhow!(
                "Unsupported argument {:?}, must be [release|debug]",
                args
            ))
        }
    }
    cmd!(sh, "espflash save-image ESP32 --flash-size 2MB crates/cross_compiled/target/xtensa-esp32-espidf/{build_type}/cross_compiled firmware-ota.bin").run()?;
    let mut version_file = File::create("version.txt")?;
    version_file.write_all(git_version.as_bytes())?;

    Ok(())
}
