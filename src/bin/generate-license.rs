use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

/// Helper application for generating the user manual license files.
#[derive(StructOpt, Debug)]
struct CliArgs {}

const USER_MANUAL_DIR: &str = "user-manual";

fn main() -> Result<(), Box<dyn Error>> {
    let _args = CliArgs::from_args();

    run_cargo_about("library-licenses.md.hbs")?;
    run_cargo_about("software-bom.md.hbs")?;

    println!("Finished generating licenses.");
    Ok(())
}

// Runs cargo-about using the provided template file/
fn run_cargo_about(template_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Generating licenses from {}", template_file);

    let current_dir = match std::env::var("CRATE_ROOT_DIR") {
        Ok(crate_root_dir) => Path::new(crate_root_dir.as_str()).to_path_buf(),
        Err(_) => std::env::current_dir()?,
    };
    let template_dir = current_dir.join(USER_MANUAL_DIR).join("template");
    let template_name = template_dir.join(template_file);
    let cargo_toml = current_dir.join("Cargo.toml");

    let output = Command::new("cargo")
        .args(&[
            "about",
            "-m",
            cargo_toml.to_str().unwrap(),
            "generate",
            template_name.to_str().unwrap(),
        ])
        .output()?;

    if output.stderr.is_empty() {
        // Write the generated file if there were no errors.
        let src_dir = Path::new(USER_MANUAL_DIR).join("src");
        let out_file = Path::new(template_file).file_stem().unwrap();
        let out_file_path = src_dir.join(out_file);
        let mut f = File::create(out_file_path)?;
        f.write_all(output.stdout.as_slice())?;
        Ok(())
    } else {
        eprintln!("{}", std::str::from_utf8(output.stderr.as_slice())?);
        panic!();
    }
}
