#![allow(unused_imports)] 
#![allow(unused_variables)]

use std::{
    collections::HashMap, fs::{self, File, OpenOptions}, io::{BufRead, BufReader, ErrorKind, Read, Write}, os::unix::fs::MetadataExt, path::{self, Path, PathBuf}, process, time::SystemTime
};
use sha2::{Digest, Sha256};
use clap::{Arg, ArgAction, Command};
use chrono::{DateTime, Local};

#[allow(dead_code)]

fn format_system_time(time: std::io::Result<SystemTime>) -> String {
    match time {
        Ok(system_time) => {
            let datetime: DateTime<Local> = system_time.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "Unavailable".to_string(),
    }
}

fn hash_generator(path: &PathBuf) -> String {
    let fopen = File::open(&path);
    let fopen = match fopen {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Error: Permission denied while accessing the path. Please check your permissions or try running the program with elevated rights.");
                process::exit(1);
            },
            _ => {
                eprintln!("Error: failed to open file {}", err);
                process::exit(1);
            }
        }  
    };
    let mut hasher = Sha256::new();
    let mut reader = BufReader::new(fopen);
    let mut buffer = [0u8; 4096];


    loop {
        let bytes_read = reader.read(&mut buffer).expect("Error: failed to read buffer file content");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    hash

}

fn human_readable_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}


#[allow(dead_code)]

fn file_hash_metadata(directorypath: &str, file_extension: &Vec<String>, ouput_file: &str) {
    let dirpath = Path::new(directorypath);
    if dirpath.is_dir() {
        let entries = fs::read_dir(dirpath);
        let entries = match entries {
            Ok(entries) => entries,
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied while accessing the path. Please check your permissions or try running the program with elevated rights.");
                    process::exit(1);
                },
                _ => {
                    eprintln!("Error: Failed to read Directory {}", err);
                    process::exit(1);
                }
                
            }
        };
        for entry in entries {
            let entry = match entry {
                Ok(entry_path) => entry_path,
                Err(err) => {
                    eprintln!("Error: failed to read entries");
                    continue;
                }
            };
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension().and_then(|ex| ex.to_str()) {
                    if file_extension.contains(&extension.to_string()) {
                        let file_metadata: Result<fs::Metadata, std::io::Error> = file_path.metadata();
                        let file_metadata = match file_metadata {
                            Ok(metadata) => metadata,
                            Err(err) => {
                                eprintln!("Error: failed to read file Metadata {}", err);
                                continue;
                            }
                        };
                        let meta_hash_data = format!("{} | {} | {} | {}\n", file_path.display(), hash_generator(&file_path), human_readable_size(file_metadata.size()), format_system_time(file_metadata.modified()));
                        let open_file_rw = OpenOptions::new()
                        .append(true)
                        .write(true)
                        .open(ouput_file);
                        let mut file = match open_file_rw {
                            Ok(f) => f,
                            Err(err) => match err.kind() {
                                ErrorKind::PermissionDenied => {
                                    eprintln!("Error: Permission denied while accessing the path. Please check your permissions or try running the program with elevated rights.");
                                    process::exit(1);
                                },
                                ErrorKind::IsADirectory => {
                                    eprintln!("Error: Please enter valid file path not directory path.");
                                    process::exit(1)
                                },
                                ErrorKind::NotFound => match File::create(ouput_file) {
                                    Ok(nf) => nf,
                                    Err(er) => {
                                        eprintln!("Error: Failed to create file {}.", er);
                                        process::exit(1);
                                    }
                                },
                                _ => {
                                    eprintln!("Error: Failed to open file for write data {}", err);
                                    process::exit(1)
                                }
                            }
                        };
                        file.write_all(meta_hash_data.as_bytes()).expect("Error: Failed to write data in ")
                        
                        
                    }
                }
            }else if file_path.is_dir() {
                let path_to_str = &file_path.to_str();
                let path_to_str = match path_to_str {
                    Some(x) => x,
                    None => {
                        process::exit(1)
                    }
                };
                file_hash_metadata(path_to_str, file_extension, ouput_file);
            }
        }



    }else if !dirpath.is_dir() {
        eprintln!("Error: enter valid directory path.");
        process::exit(1);
    }
    
}

#[allow(dead_code)]
fn verify_hash(directorypath: &str, file_extension: &Vec<String>, hash_file: &str, verbose: bool) {
    let mut known_hashes = HashMap::new();

    let file = match File::open(hash_file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: Could not open hash file: {}", err);
            process::exit(1);
        }
    };

    for line in BufReader::new(file).lines() {
        let line = line.unwrap_or_default();
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() != 4 {
            eprintln!("Error: Invalid line in hash file: {}", line);
            process::exit(1);
        }

        let path = parts[0].to_string();
        let hash = parts[1].to_string();
        let size = parts[2].to_string();
        let modified = parts[3].to_string();

        known_hashes.insert(path.clone(), (hash, size, modified));
    }

    let mut seen_paths = Vec::new();

    fn walk(path: &Path, ext: &Vec<String>, known: &HashMap<String, (String, String, String)>, seen: &mut Vec<String>, verbose: bool) {
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                walk(&path, ext, known, seen, verbose);
            } else if path.is_file() {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    if !ext.contains(&extension.to_string()) {
                        continue;
                    }

                    let full_path = path.to_string_lossy().to_string();
                    seen.push(full_path.clone());

                    let new_hash = hash_generator(&path);

                    if let Some((old_hash, size, modified)) = known.get(&full_path) {
                        if &new_hash == old_hash {
                            if verbose {
                                println!("\n");
                                println!("----------------------------------------------------------------------\n");
                                println!("\n[OK] {}", path.display());
                            }
                        } else {
                            println!("\n");
                            println!("----------------------------------------------------------------------\n");
                            println!("\n[MODIFIED] {}: Hash changed", path.display());
                        }
                    } else {
                        println!("\n");
                        println!("----------------------------------------------------------------------\n");
                        println!("\n[NEW] {}: Not found in saved hash file", path.display());
                    }
                }
            }
        }
    }

    walk(Path::new(directorypath), file_extension, &known_hashes, &mut seen_paths, verbose);

    for stored_path in known_hashes.keys() {
        if !seen_paths.contains(stored_path) {
            let (hash, size, modified) = &known_hashes[stored_path];
            println!("\n");
            println!("----------------------------------------------------------------------\n");
            println!("\n[DELETED] {} | {} | {} | {}", stored_path, hash, size, modified);
        }
    }
}


fn main() {
    let user_args = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about("Create and verify file hashes recursively\n")
    .long_about("This tool lets you generate cryptographic hashes of files with specific extensions in a directory and later verify if any files have been modified, deleted, or newly added.\n")
    .subcommand_required(true)

    .subcommand(
        Command::new("mode")
        .subcommand_required(true)
        .long_about("Selects the operational mode: either 'generate' to create file hashes, or 'verify' to detect file changes.\n")
        .subcommand_required(true)

        .subcommand(
            Command::new("generate")
            .long_about("Scans the given directory for .txt files and generates their hashes. The result is saved to the specified output file.\n")

            .arg(
                Arg::new("path")
                .short('p')
                .long("path")
                .required(true)
                .value_name("DIR")
                .help("Path to the directory you want to scan and hash .txt files from.\n")
            )

            .arg(
                Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("File path to store generated hashes (default: hashes.txt).")
                .default_value("hashes.txt")
            )

            .arg(
                Arg::new("ext")
                .short('e')
                .long("ext")
                .value_name("EXTENSIONS")
                .required(true)
                .value_delimiter(',')
                .help("Specify the file extensions to include when generating hashes (e.g., txt,log,json)\n")
            )

        )

        .subcommand(
            Command::new("verify")
            .long_about("Compares current .txt file hashes in the directory against a previously saved hash file and reports any changes.\n")

            .arg(
                Arg::new("path")
                .short('p')
                .long("path")
                .required(true)
                .value_name("DIR")
                .help("Path to directory where current .txt files will be scanned.\n")
            )

            .arg(
                Arg::new("hashfile")
                .short('H')
                .long("hashfile")
                .required(true)
                .help("Path to the file that contains previously saved hashes.\n")
            )

            .arg(
                Arg::new("ext")
                .short('e')
                .long("ext")
                .value_name("EXTENSIONS")
                .help("Specify the file extensions to include when verifying file hashes (e.g., txt,log,json)\n")
                .value_delimiter(',')
                .required(true)
            )

            .arg(
                Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Print information for unchanged files too")
            )
        )

    )
    .get_matches();


    if let Some(mode_matches) = user_args.subcommand_matches("mode") {
        if let Some(generate_matches) = mode_matches.subcommand_matches("generate") {
            let path = generate_matches.get_one::<String>("path").unwrap();
            let output = generate_matches.get_one::<String>("output").unwrap();
            let extensions: Vec<String> = generate_matches
                .get_many::<String>("ext")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
    
            file_hash_metadata(&path, &extensions, &output);
        } else if let Some(verify_matches) = mode_matches.subcommand_matches("verify") {
            let path = verify_matches.get_one::<String>("path").unwrap();
            let hashfile = verify_matches.get_one::<String>("hashfile").unwrap();
            let extensions: Vec<String> = verify_matches
                .get_many::<String>("ext")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();
    
            let is_verbose = verify_matches.get_flag("verbose");
    
            verify_hash(&path, &extensions, &hashfile, is_verbose);
        }
    }
    
    

}