// Import needed modules
use std::{env, fs};
use std::io::{self, Write, Read};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::md5::Md5;

use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use console::Term;

fn main() -> io::Result<()> {
    // Initialize terminal
    let term = Term::stdout();
    term.set_title("Hasher");

    // Get command line arguments and check if a file path is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as an argument.");
    }
    let file_path = &args[1];

    // Read the file content
    let file_content = fs::read(file_path)?;

    // Initialize a shared hash map to store the hashes and their counts
    let hash_map: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    // Spawn a thread to calculate the SHA256 hash
    let sha256_content = file_content.clone();
    let sha256_hash_map = Arc::clone(&hash_map);
    let sha256_thread = thread::spawn(move || {
        let mut sha256 = Sha256::new();
        sha256.input(&sha256_content);
        let sha256_hash = sha256.result_str();
        let mut sha256_hash_map = sha256_hash_map.lock().unwrap();
        sha256_hash_map.entry(sha256_hash).and_modify(|count| *count += 1).or_insert(1);
    });

    // Spawn a thread to calculate the MD5 hash
    let md5_content = file_content.clone();
    let md5_hash_map = Arc::clone(&hash_map);
    let md5_thread = thread::spawn(move || {
        let mut md5 = Md5::new();
        md5.input(&md5_content);
        let md5_hash = md5.result_str();
        let mut md5_hash_map = md5_hash_map.lock().unwrap();
        md5_hash_map.entry(md5_hash).and_modify(|count| *count += 1).or_insert(1);
    });

    // Wait for both threads to finish
    sha256_thread.join().unwrap();
    md5_thread.join().unwrap();

    // Print the hashes and their counts
    let hash_map = hash_map.lock().unwrap();
    for (hash, count) in hash_map.iter() {
        let hash_type = match hash.len() {
            64 => "SHA256",
            32 => "MD5",
            _ => "Unknown"
        };
        println!("{}: {}: {}", hash_type, hash, count);
    }

    // Wait for user input to prevent the console window from closing
    print!("Press enter to exit...");
    io::stdout().flush()?;
    let _ = io::stdin().read(&mut [0u8])?;

    Ok(())
}
