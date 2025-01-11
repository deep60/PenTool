use std::{env, fs, path::Path, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprint!("Usage: <program> <nmap-arguments>");
        std::process::exit(1);
    }

    let name = format!("nmap-osscan.{}", args.join(" "));
    let tmp = "/tmp/.n";
    let out = format!("../out/{}", name);

    println!("Running nmap..");
    let nmap_status = Command::new("nmap")
        .arg("-oX")
        .arg(tmp)
        .arg("-T4")
        .arg("-A")
        .args(&args)
        .status()?;

    if !nmap_status.success() {
        eprint!("nmap command failed.");
        std::process::exit(1);
    }

    println!("Converting output with xq..");
    let xq_status = Command::new(xq).arg("--xml-file").arg(tmp).output()?;

    if !xq_status.status.success() {
        eprintln!(
            "xq cmmand failed: {}",
            String::from_utf8_lossy(&xq_status.stderr)
        );
        std::process::exit(1);
    }

    fs::write(&out, xq_status.stdout)?;
    println!("Wrote {}", out);

    Ok(())
}
