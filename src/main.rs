use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: <program> <nmap-arguments>");
        std::process::exit(1);
    }

    // Create output directory if it doesn't exist
    let out_dir = Path::new("out");
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
        println!("Created output directory: {:?}", out_dir);
    }

    let name = format!("nmap-osccan.{}", args.join(" "));
    let tmp = format!("{}/.nmap_output.xml", env::temp_dir().to_str().unwrap());
    let out = format!("out/{}", name);

    println!("Running nmap...");
    let nmap_output = Command::new("nmap")
        .arg("-oX")
        .arg(&tmp)
        .arg("-T4")
        .arg("-A")
        .arg("-Pn")
        .args(&args)
        .output()?;

    // Check if nmap command failed
    if !nmap_output.status.success() {
        eprintln!("nmap command failed: {}", String::from_utf8_lossy(&nmap_output.stderr));
        std::process::exit(1);
    }

    // Print nmap output for debugging
    println!("Nmap output: {}", String::from_utf8_lossy(&nmap_output.stdout));

    // Verify the temporary file was created
    if !Path::new(&tmp).exists() {
        eprintln!("Error: Temporary file not created at {}", tmp);
        eprintln!("This might be because no hosts were found in the scan.");
        std::process::exit(1);
    }

    println!("Converting output with jq..");
    let jq_status = Command::new("jq")
        .arg("-R")
        .arg("-s")
        .arg(".")
        .arg(&tmp)
        .output()?;

    if !jq_status.status.success() {
        eprintln!(
            "jq command failed: {}",
            String::from_utf8_lossy(&jq_status.stderr)
        );
        std::process::exit(1);
    }

    fs::write(&out, jq_status.stdout)?;
    println!("Write {}", out);

    // Clean up temporary file
    if let Err(e) = fs::remove_file(&tmp) {
        eprintln!("Warning: Failed to remove temporary file {}: {}", tmp, e);
    }

    Ok(())
}
