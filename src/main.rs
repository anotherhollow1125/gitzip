use quicli::prelude::*;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Command {
    #[structopt(short = "u", long = "unzip")]
    /// unzip gitzip file
    unzip: bool,
}

fn main() -> CliResult {
    let args = Command::from_args();

    if !args.unzip {
        gitzip()?;
    } else {
        gitunzip()?;
    }

    Ok(())
}

fn gitzip() -> CliResult {
    let git_str = "./.git";
    let git_path = Path::new(git_str);
    let gitignore_path = Path::new("./.gitignore");

    if !git_path.exists() {
        return Err(failure::err_msg("There are no git repository").into());
    }

    let curdir = env::current_dir()?;
    let curdir_name = curdir.file_name().unwrap().to_string_lossy();
    let dest = format!("{}_git.zip", curdir_name);

    if !gitignore_path.exists() {
        let _ = fs::File::create(gitignore_path)?;
    }

    let mut already_written = false;
    {
        let f = fs::File::open(gitignore_path)?;
        for line in io::BufReader::new(f).lines() {
            let line = line?;
            if line.contains(&dest) {
                already_written = true;
            }
        }
    }

    if !already_written {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(gitignore_path)?;
        writeln!(f, "{}", dest)?;
    }

    match env::consts::OS {
        "windows" => {
            std::process::Command::new("powershell.exe")
                .args(["-c", "Compress-Archive"])
                .args(["-Path", git_str])
                .args(["-DestinationPath", &dest])
                .arg("-Force")
                .spawn()?;
        }
        "linux" => {
            std::process::Command::new("zip")
                .arg("-r")
                .arg(&dest)
                .arg(git_str)
                .spawn()?;
        }
        _ => (),
    }

    Ok(())
}

fn gitunzip() -> CliResult {
    let target = format!("{}_git.zip", env::current_dir()?.to_string_lossy());

    if !Path::new(&target).exists() {
        return Err(failure::err_msg("target not found.").into());
    }

    match env::consts::OS {
        "windows" => {
            println!("windows");
            std::process::Command::new("powershell.exe")
                .args(["-c", "Expand-Archive"])
                .args(["-Path", &target])
                .args(["-DestinationPath", "."])
                .arg("-Force")
                .spawn()?;
        }
        "linux" => {
            println!("linux");
            std::process::Command::new("unzip").arg(&target).spawn()?;
        }
        _ => (),
    }

    Ok(())
}
