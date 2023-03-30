use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const PNG_IEND: [u8; 12] = [0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

fn check_png_signature(bytes: &[u8]) -> bool {
    bytes.starts_with(&PNG_SIGNATURE)
}

fn iend_count(bytes: &[u8]) -> u32 {
    let mut count = 0;
    for w in bytes.windows(PNG_IEND.len()) {
        if w == PNG_IEND {
            count += 1;
        }
    }
    count
}

struct PngCheckResult {
    signature_ok: bool,
    iend_count: u32,
}

fn check_file(path: &Path) -> io::Result<PngCheckResult> {
    let mut file = File::open(path)?;

    let mut bytes: Vec<u8> = vec!();
    file.read_to_end(&mut bytes)?;

    Ok(PngCheckResult {
        signature_ok: check_png_signature(&bytes),
        iend_count: iend_count(&bytes),
    })
}

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

        if !result.signature_ok {
            //println!("{}: Malformed png, no signature", path.display());
            continue;
        }

        match result.iend_count {
            0 => { println!("{}: Malformed png, no end", path.display()); }
            1 => {
                //println!("{}: Ok", path.display());
            }
            _ => { println!("{}: Bad crop! {} png ends detected", path.display(), result.iend_count); }
        }
    }
}
