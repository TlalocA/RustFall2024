use std::process::Command;

// not exactly working?

fn executing_os_commands_linux() {
    let output = Command::new(program: "python3")
        .arg("my_script.py")
        .output()
        .expect("Failed to execute command");

    println!("{:?}", output);
    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    executing_os_commands_linux();
    
}