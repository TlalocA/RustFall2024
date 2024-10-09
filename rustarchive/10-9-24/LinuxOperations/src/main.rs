use std::process:Command;

fn main(){
    let new_file = "rust_linux.txt";
    let command_to_execute = format!("touch {}", new_file);
    println!("{}", command_to_execute);

    let output = Command::new(program: "sh").arg("-c").arg(command_to_execute).spawn();

}