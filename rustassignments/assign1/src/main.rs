fn fahrenheit_to_celsius(f:f64) -> f64{
    //(f − 32) * 5/9 = c
    (f - 32.0) * (5.0/9.0)
   
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    //(c * 9/5) + 32 = f
   (c * (9.0/5.0)) + 32.0
   
}

fn main() {
    let mut f:f64 = 32.0;
    let mut c:f64 = 0.0;

    let mut count = 5;
    while count != 0{
        f += 1.0;
        c += 1.0;

        println!("{}F is {:.0}C", f, fahrenheit_to_celsius(f));
        println!("{}C is {:.0}F", c, celsius_to_fahrenheit(c));
        println!(" ");
        
        count -= 1;
    }

}
