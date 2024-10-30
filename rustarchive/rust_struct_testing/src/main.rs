mod student;
mod major;

fn main() {
    let s1 = student::Student::new("JOHN", "CS");
    println!("{:?}", s1);

}
