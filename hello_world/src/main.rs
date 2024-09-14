fn main() {


    let s1 = String::from("Hello");
    let s2 = s1; // This moves ownership, s1 is no longer valid
    
    // println!("s1 is: {}, s2 is: {}", s1, s2); // This would cause a compile error
}