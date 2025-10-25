use std::error::Error;
use std::fs::{File, OpenOptions, read};
use std::io::{Read, Seek, SeekFrom, Write, stdin};
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    println!("enter path to file to be loaded:");

    let mut file_path = String::new();
    let _ = match stdin().read_line(&mut file_path) {
        Ok(read_count) => read_count,
        Err(err) => {
            println!("failed to read file path due to: {err}");
            return Err(Box::new(err));
        }
    };
    let file_path = file_path.trim();

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("failed to open file due to: {err}");
            return Err(Box::new(err));
        }
    };

    let mut contents = String::new();
    let _ = match file.read_to_string(&mut contents) {
        Ok(read_count) => read_count,
        Err(err) => {
            eprintln!("failed to read content of the file into memory due to: {err}");
            return Err(Box::new(err));
        }
    };
    println!("current content of: {file_path} is:\n{contents}");

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    let _ = match file.write_all(contents.as_bytes()) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("failed to write contents into the file due to: {err}");
            return Err(Box::new(err));
        }
    };
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("failed to run due to: {err}");
        std::process::exit(1);
    }
}
