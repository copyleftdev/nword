use std::fs::File;
use std::io::Read;
use std::path::Component;  // Added import for Component
use walkdir::DirEntry;

const BLACKLIST_EXTENSIONS: [&str; 29] = [
    "exe", "dll", "so", "dylib", "bin", "jpg", "jpeg", "png", "gif", "bmp", "tiff", "ico",
    "mp3", "wav", "ogg", "flac", "mp4", "mov", "avi", "mkv", "zip", "tar", "gz", "rar", "7z",
    "bz2", "class", "o", "obj",
];

const BLACKLIST_DIRECTORIES: [&str; 7] = [
    "node_modules", "target", "vendor", "dist", "build", "out", ".git"
];

pub fn should_exclude(entry: &DirEntry) -> bool {
    if is_hidden_file(entry) {
        return true;
    }

    if is_blacklisted_extension(entry) {
        return true;
    }

    if is_in_blacklisted_directory(entry) {
        return true;
    }

    if is_binary_by_header(entry) {
        return true;
    }

    if contains_exclusion_keywords(entry) {
        return true;
    }

    false
}

fn contains_exclusion_keywords(entry: &DirEntry) -> bool {
    let path = entry.path();

    match File::open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                let exclusion_keywords = ["TODO", "FIXME", "HACK", "REVIEW"];
                exclusion_keywords.iter().any(|&keyword| content.contains(keyword))
            } else {
                false
            }
        }
        Err(_) => false,  // Handle error case
    }
}

fn is_binary_by_header(entry: &DirEntry) -> bool {
    let mut file = match File::open(entry.path()) {
        Ok(f) => f,
        Err(_) => return false,  // If the file can't be opened, consider it not binary
    };

    let mut header = [0u8; 8];
    match file.read(&mut header) {
        Ok(bytes_read) if bytes_read >= 4 => {
            if header.starts_with(&[0x7F, b'E', b'L', b'F']) || header.starts_with(&[b'M', b'Z']) {
                return true;
            }

            let mach_o_signatures: [[u8; 4]; 4] = [
                [0xFE, 0xED, 0xFA, 0xCE],
                [0xCE, 0xFA, 0xED, 0xFE],
                [0xFE, 0xED, 0xFA, 0xCF],
                [0xCF, 0xFA, 0xED, 0xFE],
            ];

            if mach_o_signatures.contains(&header[0..4].try_into().unwrap()) || header.contains(&0) {
                return true;
            }

            false
        }
        _ => false,  // Handle error cases gracefully
    }
}

fn is_hidden_file(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map_or(false, |name| name.starts_with('.'))
}

fn is_blacklisted_extension(entry: &DirEntry) -> bool {
    entry.path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext| BLACKLIST_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
}

fn is_in_blacklisted_directory(entry: &DirEntry) -> bool {
    entry.path()
        .components()
        .any(|component| match component {
            Component::Normal(os_str) => {
                let comp_str = os_str.to_str().unwrap_or("").to_lowercase();
                BLACKLIST_DIRECTORIES.contains(&comp_str.as_str())
            }
            _ => false,
        })
}