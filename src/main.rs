mod png;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::png::check_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }
    let path_argument = Path::new(&args[1]);

    let mut paths: Vec<PathBuf> = vec!();

    if path_argument.is_dir() {
        println!("Scanning directory");

        for entry_result in WalkDir::new(path_argument) {
            match entry_result {
                Ok(entry) if entry.file_type().is_file() => {
                    let path = entry.path();
                    let extension = match path.extension() {
                        None => { continue; }
                        Some(extension) => { extension }
                    };

                    if extension.to_str() == Some("png") {
                        paths.push(PathBuf::from(path));
                    }
                }
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Skipping directory entry because of {}", err)
                }
            }
        }
    } else if path_argument.is_file() {
        paths.push(path_argument.to_path_buf());
    }

    for path in paths {
        let result = match check_file(&path) {
            Ok(result) => { result }
            Err(err) => {
                eprintln!("{}: Error: {}", path.display(), err);
                continue;
            }
        };

        if !result.signature_ok() {
            //println!("{}: Malformed png, no signature", path.display());
            continue;
        }

        match result.iend_count() {
            0 => { println!("{}: Malformed png, no end", path.display()); }
            1 => {
                //println!("{}: Ok", path.display());
            }
            _ => { println!("{}: Bad crop! {} png ends detected", path.display(), result.iend_count()); }
        }
    }
}
