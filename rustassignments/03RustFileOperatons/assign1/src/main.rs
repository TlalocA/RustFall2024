use std::io::{self, Read, Write};
use std::fs::File;

struct Car {
    name: String,
    year: u32,
    color: String,
}

impl Car {
    fn from_file(path: &str) -> Car {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let year = lines.next().unwrap().parse().unwrap();
        let color = lines.next().unwrap().to_string();

        Car { name, year, color }
    }
}

fn writing_from_console() {
    let mut file = File::create("user_info.txt").unwrap();
    let mut buffer = String::new();

    print!("What car do you drive? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What year is the car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    // convert int to string to append to file
    let year:u32 = buffer.trim().parse().unwrap();
    buffer.clear();

    print!("What color is the car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    // write to file
    writeln!(file, "{}", name).unwrap();
    writeln!(file, "{}", year).unwrap();
    writeln!(file, "{}", color).unwrap();

    println!();
}

fn reading_from_file() {
    let car = Car::from_file("user_info.txt");
    println!("Car information:");
    println!("----------");
    println!("Car Model: {}", car.name);
    println!("Year: {}", car.year);
    println!("Color: {}", car.color);
}


fn main() {
    writing_from_console();
    reading_from_file();
}