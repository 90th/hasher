use std::{env, fs};
use crypto::digest::Digest;
use crypto::sha2::{Sha256, Sha512};
use crypto::md5::Md5;
use console::Term;
use std::io;
use rayon::prelude::*;

fn calculate_hash(file_content: &[u8], mut hasher: Box<dyn Digest + Send>, hash_type: String) -> String {
    hasher.input(file_content);
    let hash = hasher.result_str();
    format!("{}: {}", hash_type, hash)
}

fn main() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Hasher");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Please provide a file path as an argument."));
    }
    let file_path = &args[1];

    let file_content = fs::read(file_path)?;

    let hashes: Vec<String> = vec![
        ("SHA256", Box::new(Sha256::new()) as Box<dyn Digest + Send>),
        ("MD5", Box::new(Md5::new()) as Box<dyn Digest + Send>),
        ("SHA512", Box::new(Sha512::new()) as Box<dyn Digest + Send>),
    ].into_par_iter() // Use into_par_iter instead of par_iter
        .map(|(hash_type, hasher)| calculate_hash(&file_content, hasher, hash_type.to_string()))
        .collect();

    for hash in hashes {
        println!("{}", hash);
    }

    Ok(())
}
