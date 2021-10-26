use clap_generate::generators::{Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate_to, generators::Bash};
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut app = build_cli();
    let path = generate_to(Bash, &mut app, "taskus", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Zsh, &mut app, "taskus", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Fish, &mut app, "taskus", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(PowerShell, &mut app, "taskus", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    let path = generate_to(Elvish, &mut app, "taskus", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    Ok(())
}
