use std::env;

use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    match args[0] {
        "build" => build_target(&args[1..]),
        "check" => check_target(),
        "clean" => clean_target(),
        "flash" => flash_target(&args[1..]),
        "doc" => doc_target(),
        "uml" => generate_uml_images(),
        _ => {
            println!("USAGE cargo xtask [build|check|clean|flash|doc|uml]");
            Ok(())
        }
    }
}

fn build_target(args: &[&str]) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo build {args...}").run()?;
    Ok(())
}

fn check_target() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir("crates/cross_compiled");
    cmd!(sh, "rustup run esp cargo check").run()?;
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
