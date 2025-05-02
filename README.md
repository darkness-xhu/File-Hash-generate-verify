# File-Hash Generate and Verify

## ABOUT TOOL

File-Hash Generate and Verify is a Rust-based command-line tool designed to generate and verify file hashes. It allows users to create a cryptographic hash (e.g., SHA-256) of a file and later verify the integrity of the file by comparing its hash against a previously generated one. This tool is useful for file verification, ensuring the integrity of files, or comparing file contents.

## FEATURES

- **Generate** a hash for any given file.
- **Verify** the integrity of a file by comparing its current hash with a previously saved one.
- Supports multiple platforms: Linux, Windows, and Mac.

## AVAILABLE ON

- Linux
- Windows
- Mac

## REQUIREMENTS

- **Internet**: Required for cloning the repository and any necessary updates.
- **Storage**: 8.46 MB of disk space for the application.

## INSTALLATION

### Prerequisites:
Make sure you have **Rust** installed on your machine. If not, you can install it from [Rust's official website](https://www.rust-lang.org/tools/install).

### Steps to install:

1. Clone the repository:
   
```
https://github.com/darkness-xhu/File-Hash-generate-verify.git
```
2. Navigate into the project directory:
```
cd File-Hash-generate-verify
```

3. Build and run the tool:
```
cargo run
```


## USAGE

### 1. **Generate File Hash**:
```bash
cargo run -- generate --file /path/to/file
```
### 2. **Verify File Hash**:
```bash
argo run -- verify --file /path/to/file --hash <expected_hash> -e <expected file type> 

```
This will compare the file's current hash with the provided hash and inform you if the file is intact or has been altered.


## CONNECT WITH US :
[![Instagram](https://img.shields.io/badge/INSTAGRAM-FOLLOW-red?style=for-the-badge&logo=instagram)](https://www.instagram.com/darkness.xhu)


## WARNING

**This tool is only for educational purposes.**  
If you use this tool for any purposes other than education, we will not be held responsible for any consequences.

## LICENSE

This project is open-source and available under the [MIT License](LICENSE).
