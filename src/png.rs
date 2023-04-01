use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

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

pub struct PngCheckResult {
    signature_ok: bool,
    iend_count: u32,
}

impl PngCheckResult {
    pub fn signature_ok(&self) -> bool {
        self.signature_ok
    }

    pub fn iend_count(&self) -> u32 {
        self.iend_count
    }
}

pub fn check_file(path: &Path) -> io::Result<PngCheckResult> {
    let mut file = File::open(path)?;

    let mut bytes: Vec<u8> = vec!();
    file.read_to_end(&mut bytes)?;

    Ok(PngCheckResult {
        signature_ok: check_png_signature(&bytes),
        iend_count: iend_count(&bytes),
    })
}
