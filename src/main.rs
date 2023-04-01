mod png;

use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

use crate::png::check_file;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(required = true, value_hint = clap::ValueHint::AnyPath)]
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut png_paths: Vec<PathBuf> = vec!();

    for path_argument in args.paths {
        if path_argument.is_dir() {
            println!("Scanning directory {} for png files", path_argument.display());

            for entry_result in WalkDir::new(path_argument) {
                match entry_result {
                    Ok(entry) if entry.file_type().is_file() => {
                        let path = entry.path();
                        let extension = match path.extension() {
                            None => { continue; }
                            Some(extension) => { extension }
                        };

                        if extension.to_str() == Some("png") {
                            png_paths.push(PathBuf::from(path));
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Skipping directory entry because of {}", err)
                    }
                }
            }
        } else if path_argument.is_file() {
            png_paths.push(path_argument.to_path_buf());
        } else {
            eprintln!("{} is not a valid path to a file or directory", path_argument.display());
            return;
        }
    }

    for png_path in png_paths {
        let result = match check_file(&png_path) {
            Ok(result) => { result }
            Err(err) => {
                eprintln!("{}: Error: {}", png_path.display(), err);
                continue;
            }
        };

        if !result.signature_ok() {
            //println!("{}: Malformed png, no signature", path.display());
            continue;
        }

        match result.iend_count() {
            0 => { println!("{}: Malformed png, no end", png_path.display()); }
            1 => {
                //println!("{}: Ok", path.display());
            }
            _ => { println!("{}: Bad crop! {} png ends detected", png_path.display(), result.iend_count()); }
        }
    }
}
