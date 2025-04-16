use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: <program> <nmap-arguments>");
        std::process::exit(1);
    }

    let name = format!("nmap-osccan.{}", args.join(" "));
    let tmp = "/tmp/.n";
    let out = format!("../out/{}", name);

    println!("Running nmap...");
    let nmap_status = Command::new("nmap")
        .arg("-oX")
        .arg(tmp)
        .arg("-T4")
        .arg("-A")
        .args(&args)
        .status()?;

    if !nmap_status.success() {
        eprintln!("nmap command failed.");
        std::process::exit(1);
    }

    println!("Validating XML output...");
    let validate_status = Command::new("xmllint")
        .arg("--noout")
        .arg(tmp)
        .status()?;

    if !validate_status.success() {
        eprintln!("XML validation failed.");
        std::process::exit(1);
    }

    println!("Converting output with xq..");
    let xq_status = Command::new("xq")
        .arg("--input-format")
        .arg("xml")
        .arg(tmp)
        .output()?;

    if !xq_status.status.success() {
        eprintln!(
            "xq command failed: {}",
            String::from_utf8_lossy(&xq_status.stderr)
        );
        std::process::exit(1);
    }

    fs::write(&out, xq_status.stdout)?;
    println!("Write {}", out);

    Ok(())
}
