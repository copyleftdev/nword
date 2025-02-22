Here’s the **ultimate usage README** for your Rust project, `nword`. This README provides a comprehensive guide to installing, configuring, and using the project, as well as details about its functionality and dependencies.

---

# **nword**  
**A Rust-based tool for scanning, processing, and analyzing source code directories.**

`nword` is a command-line tool designed to scan directories, detect programming languages, and process source code files. It generates a JSON snapshot of the directory structure, including file metadata, language detection, and condensed content.

---

## **Features**
- **Directory Scanning**: Recursively scans directories for source code files.
- **Language Detection**: Detects the programming language of files based on their extensions.
- **File Filtering**: Excludes binary files, hidden files, and blacklisted directories/extensions.
- **Content Condensation**: Condenses file content by removing extra whitespace.
- **JSON Output**: Generates a JSON snapshot of the directory structure and file metadata.

---

## **Installation**

### **Prerequisites**
- Rust (version 1.60 or higher)
- Cargo (Rust's package manager)

### **Steps**
1. Clone the repository:
   ```bash
   git clone https://github.com/copyleftdev/nword.git
   cd nword
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   cargo install --path .
   ```

4. Verify installation:
   ```bash
   nword --version
   ```

---

## **Usage**

### **Basic Command**
Scan the current directory and generate a JSON snapshot:
```bash
nword
```

### **Specify a Directory**
Scan a specific directory:
```bash
nword --directory /path/to/your/directory
```

### **Output**
The tool generates a JSON file named `project_snapshot_<timestamp>.json` in the current directory. The file contains:
- **File Path**: The relative path of the file.
- **Language**: The detected programming language.
- **Content**: The condensed content of the file.

Example output:
```json
{
  "files": [
    {
      "path": "src/main.rs",
      "language": "Rust",
      "content": "mod cli; mod file_processor; mod filters; mod language_detection; mod text_processing; ..."
    },
    {
      "path": "src/filters.rs",
      "language": "Rust",
      "content": "use std::fs::File; use std::io::Read; use std::path::Component; ..."
    }
  ]
}
```

---

## **Configuration**

### **Blacklisted Extensions**
The tool excludes files with specific extensions (e.g., binaries, images). You can modify the `BLACKLIST_EXTENSIONS` array in `src/filters.rs` to customize the list.

Example:
```rust
const BLACKLIST_EXTENSIONS: [&str; 29] = [
    "exe", "dll", "so", "dylib", "bin", "jpg", "jpeg", "png", "gif", "bmp", "tiff", "ico",
    "mp3", "wav", "ogg", "flac", "mp4", "mov", "avi", "mkv", "zip", "tar", "gz", "rar", "7z",
    "bz2", "class", "o", "obj",
];
```

### **Blacklisted Directories**
The tool excludes specific directories (e.g., `node_modules`, `target`). You can modify the `BLACKLIST_DIRECTORIES` array in `src/filters.rs` to customize the list.

Example:
```rust
const BLACKLIST_DIRECTORIES: [&str; 7] = [
    "node_modules", "target", "vendor", "dist", "build", "out", ".git"
];
```

---

## **Dependencies**
The project uses the following Rust crates:
- **clap**: For command-line argument parsing.
- **walkdir**: For directory traversal.
- **rayon**: For parallel processing.
- **serde**: For JSON serialization.
- **chrono**: For timestamp generation.

See `Cargo.toml` for the full list of dependencies.

---

## **Development**

### **Building and Testing**
1. Build the project:
   ```bash
   cargo build
   ```

2. Run tests:
   ```bash
   cargo test
   ```

3. Run the project locally:
   ```bash
   cargo run -- --directory /path/to/test
   ```

### **Code Structure**
- **`src/main.rs`**: Entry point for the application.
- **`src/cli.rs`**: Handles command-line arguments.
- **`src/file_processor.rs`**: Processes directories and files.
- **`src/filters.rs`**: Filters files based on extensions, directories, and content.
- **`src/language_detection.rs`**: Detects programming languages based on file extensions.
- **`src/text_processing.rs`**: Condenses file content.

---

## **Contributing**
Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a detailed description of your changes.

---

## **License**
This project is licensed under the **MIT License**. See the `LICENSE` file for details.

---

## **Support**
For questions, issues, or feature requests, please open an issue on the [GitHub repository](https://github.com/copyleftdev/nword/issues).

---

Enjoy using `nword`! 🚀