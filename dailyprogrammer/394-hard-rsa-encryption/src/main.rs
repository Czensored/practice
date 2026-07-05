// Ref: https://www.reddit.com/r/dailyprogrammer/comments/nzmvsj/20210614_challenge_394_difficult_rsa_encryption/
// This algorithm is not actually cryptographically secure, I did not use secure rng

use rsa_encryption::generate_keypair;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

const DEFAULT_KEY_NAME: &str = "ssh_assignment_rsa";

struct Cli {
    out_dir: PathBuf,
    name: String,
    comment: String,
}

fn main() {
    let cli = match parse_args() {
        Ok(cli) => cli,
        Err(message) => {
            eprintln!("{message}");
            process::exit(2);
        }
    };

    let keypair = generate_keypair();

    if let Err(err) = write_ssh_keypair(&keypair, &cli.out_dir, &cli.name, &cli.comment) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn parse_args() -> Result<Cli, String> {
    let mut out_dir = PathBuf::from(".");
    let mut name = DEFAULT_KEY_NAME.to_string();
    let mut comment = String::new();
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--out-dir requires a directory".to_string())?;
                out_dir = PathBuf::from(value);
            }
            "--name" => {
                name = args
                    .next()
                    .ok_or_else(|| "--name requires a filename".to_string())?;
            }
            "--comment" => {
                comment = args
                    .next()
                    .ok_or_else(|| "--comment requires a value".to_string())?;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: cargo run --release -- [--out-dir DIR] [--name FILE] [--comment TEXT]\n\
                     \n\
                     Writes FILE and FILE.pub as an OpenSSH RSA keypair.\n\
                     By default, DIR is the current directory and FILE is ssh_assignment_rsa."
                );
                process::exit(0);
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    Ok(Cli {
        out_dir,
        name,
        comment,
    })
}

fn write_ssh_keypair(
    keypair: &rsa_encryption::RsaKeyPair,
    out_dir: &Path,
    name: &str,
    comment: &str,
) -> Result<(), Box<dyn Error>> {
    if name.is_empty() || name.contains('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "key name must be a filename, not a path",
        )
        .into());
    }

    fs::create_dir_all(out_dir)?;

    let private_path = out_dir.join(name);
    let public_path = out_dir.join(format!("{name}.pub"));
    let private_key = keypair.to_openssh_private_key(comment)?;
    let public_key = keypair.to_openssh_public_key(comment);

    write_file_with_mode(&private_path, &private_key, 0o600)?;
    write_file_with_mode(&public_path, &public_key, 0o644)?;

    println!("Wrote private key: {}", private_path.display());
    println!("Wrote public key:  {}", public_path.display());
    println!("Install the .pub line on the server, then connect with:");
    println!("ssh -i {} user@host", private_path.display());

    Ok(())
}

#[cfg(unix)]
fn write_file_with_mode(path: &Path, contents: &str, mode: u32) -> io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(mode)
        .open(path)?;
    file.write_all(contents.as_bytes())?;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}

#[cfg(not(unix))]
fn write_file_with_mode(path: &Path, contents: &str, _mode: u32) -> io::Result<()> {
    fs::write(path, contents)
}
