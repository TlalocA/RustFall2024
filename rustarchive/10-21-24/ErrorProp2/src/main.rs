use std::fs::File;
//use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
}

fn main() {
    let username = read_username_from_file_3ver().unwrap();
    println!("{:?}", username);
}
