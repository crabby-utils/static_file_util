use base64ct::Encoding;
use std::fs;
use std::path::Path;

/// Utility function to process a file, generate a hash, and set an environment variable.
pub fn process_file(file_path: &str, env_var_name: &str) {
    // Create a Path object for the file
    let path = Path::new(file_path);

    // Read the file contents
    let content = fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    // Hash the content using Blake3
    let hash = blake3::hash(&content);

    // Extract the first 48 bits (6 bytes) from the hash
    let short_hash = &hash.as_bytes()[..6];

    // Encode the 48-bit hash into a base64 string using base64ct
    let base64_hash = base64ct::Base64UrlUnpadded::encode_string(short_hash);

    // Pass the base64-encoded hash as an environment variable
    println!("cargo:rustc-env={}={}", env_var_name, base64_hash);
}
