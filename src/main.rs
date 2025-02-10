use clap::{Arg, Command};
use std::process::{Command as ProcCommand, exit};
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let matches = Command::new("PatchNix")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Patches Linux binaries to work on NixOS")
        .arg(Arg::new("binary")
            .help("Path to the binary you want to patch")
            .required(true))
        .arg(Arg::new("lib-paths")
            .help("Path(s) to the NixOS libraries to use for patching")
            .required(true)
            .value_delimiter(','))
        .arg(Arg::new("output")
            .help("Path to save the patched binary")
            .required(true))
        .get_matches();

    let binary_path = matches.get_one::<String>("binary").expect("binary argument missing");
    let mut lib_paths: Vec<String> = matches.get_many::<String>("lib-paths").expect("lib-paths argument missing").cloned().collect();
    let output_path = matches.get_one::<String>("output").expect("output argument missing");

    // Add common system library paths
    let common_lib_paths = vec!["/lib", "/usr/lib", "/usr/local/lib"];
    for path in common_lib_paths {
        lib_paths.push(path.to_string());
    }

    // Step 1: Analyze the binary to find library dependencies
    let ldd_output = match ProcCommand::new("ldd")
        .arg(binary_path)
        .output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error running ldd: {}", e);
                exit(1);
            }
        };

    let ldd_output_str = String::from_utf8_lossy(&ldd_output.stdout);
    let mut missing_libraries = Vec::new();

    // Step 2: Identify missing libraries
    for line in ldd_output_str.lines() {
        if line.contains("not found") {
            if let Some(lib) = line.split_whitespace().next() {
                missing_libraries.push(lib.to_string());
            }
        }
    }

    if missing_libraries.is_empty() {
        println!("No missing libraries found. No patches needed.");
        return Ok(());
    }

    // Step 3: Attempt to patch the binary by replacing paths with the provided and common library paths
    let mut patched = false;
    for lib in &missing_libraries {
        let mut lib_found = false;
        for lib_path in &lib_paths {
            let new_path = format!("{}/{}", lib_path, lib);
            if Path::new(&new_path).exists() {
                println!("Patching {} to {}", lib, new_path);
                // Here you'd patch the binary with the new library path (simplified for this example)
                patched = true;
                lib_found = true;
                break;
            }
        }
        if !lib_found {
            eprintln!("Library {} not found in any provided or common paths", lib);
        }
    }

    if patched {
        // Here, we'd save the patched binary (simplified for this example)
        create_dir_all(Path::new(output_path).parent().unwrap())?;
        let mut patched_file = File::create(output_path)?;

        // Open the original binary
        let mut original_binary = File::open(binary_path)?;
        let mut buffer = Vec::new();

        // Read the original binary into a buffer and write to the patched file
        original_binary.read_to_end(&mut buffer)?;
        patched_file.write_all(&buffer)?;

        println!("Binary patched and saved to: {}", output_path);
    } else {
        println!("Failed to patch the binary. Some libraries were not found.");
    }

    Ok(())
}
