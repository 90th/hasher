# hasher
This is a Rust program that calculates the SHA256, MD5, and SHA512 hashes of a given file. The program uses a thread pool to compute all hashes concurrently and outputs the result.

The program reads the file content, then uses a thread pool to calculate the SHA256, MD5, and SHA512 hashes. The results are then printed to the console.

# Dependencies

The following external crates are used in this program:

- `crypto`: Provides various cryptographic algorithms, including SHA256, MD5, and SHA512.
- `console`: Offers utility functions to interact with the terminal.
- `rayon`: A data parallelism library for Rust.

Add the following lines to your Cargo.toml file to include these dependencies:

```toml
[dependencies]
crypto = "0.2.36"
console = "0.15.5"
rayon = "1.5.1"
```

# Usage
## Method 1: Drag and Drop

Compile the Rust code into an executable by running `cargo build --release`. This will generate a `hasher.exe` file in the target/release directory.
Locate the `hasher.exe` file in the `target/release` directory.
Drag and drop the file you want to hash onto the `hasher.exe` file. The application will open up in a command prompt or PowerShell window, showing the calculated hashes for the dragged file.

## Method 2: Command Line

Compile the Rust code into an executable by running `cargo build --release`. This will generate a `hasher.exe` file in the `target/release` directory.
Open a command prompt (CMD) or PowerShell window.
Navigate to the directory containing the hasher.exe file using the cd command.
Run the hasher application with the file path as an argument, like this: `hasher.exe <file_location>`. Replace `<file_location>` with the path to the file you want to hash.

For example, to grab a hash of the file named `example.txt` located in the same directory as `hasher.exe`, you would run the following command:

      hasher.exe example.txt

The application will then calculate the SHA256, MD5, and SHA512 hashes of the specified file and display the results in the command prompt or PowerShell window.  
  
