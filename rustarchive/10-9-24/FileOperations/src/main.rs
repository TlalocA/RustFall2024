use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Write(String, String),
}

impl FileOperation{
    fn get_user_input() -> String{
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        buffer.to_string()
    }

    fn validate_file(filename:&String) -> bool{
        Path::new(filename).exists()
    }
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic

            if FileOperation::validate_file(&filename){
                println!("File '{}' already exists.", filename.trim());
                return;
            }
           
            fs::File::create(&filename).unwrap();
            println!("File '{}' created successfully.", filename.trim());
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic

            if !FileOperation::validate_file(&old_name){
                println!("File '{}' does not exist.", old_name.trim());
                return;
            }
        
            fs::rename(&old_name, &new_name).unwrap();
            println!("File renamed from '{}' to '{}' successfully.", old_name.trim(), new_name.trim());
        }
        FileOperation::Write(filename, buffer) => {
             // TODO: Implement file writing logic

             if !FileOperation::validate_file(&filename){
                println!("File '{}' does not exist.", filename.trim());
                return;
            }

            fs::write(&filename, &buffer).unwrap();
            println!("File '{}' successfully written to.", filename.trim());
        }
    }
}

fn main() {
    for _ in 0..2{
        println!("Choose an operation:");
        println!("1. Create a new file");
        println!("2. Rename an existing file");
        println!("3. Write to an existing file");

        let mut choice:String = FileOperation::get_user_input();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // TODO: Prompt for new filename and call perform_operation
                println!("Type the name of the file you want to create: ");
                let new_file: String = FileOperation::get_user_input();
                perform_operation(FileOperation::Create(new_file));
            }
            "2" => {
                // TODO: Prompt for old and new filenames and call perform_operation
                println!("Type the name of the file you want to rename: ");
                let old_file: String = FileOperation::get_user_input();
                println!("Type the new name of the file: ");
                let new_name: String = FileOperation::get_user_input();

                perform_operation(FileOperation::Rename(old_file,new_name));
            }
            "3" => {
                println!("Type the name of the file you want to write to: ");
                let filename: String = FileOperation::get_user_input();
                println!("Type what you want to write: ");
                let buffer: String = FileOperation::get_user_input();

                perform_operation(FileOperation::Write(filename,buffer));
            }
            _ => println!("Invalid choice"),
        }
    }
}