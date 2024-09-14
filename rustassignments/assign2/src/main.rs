fn is_even(n: i32) -> bool{
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let num_array = [8, 22, 5, 11, 9, 15, 2, 10, 25, 3];

    for n in 0..num_array.len(){
        if is_even(num_array[n]){
            println!("{} is Even", num_array[n]);
        } else{
            println!("{} is Odd", num_array[n]);
        }

        if num_array[n] % 3 == 0 && num_array[n] % 5 == 0{
            println!("FizzBuzz");
        } else if num_array[n] % 3 == 0{
            println!("Fizz");
        } else if num_array[n] % 5 == 0{
            println!("Buzz");
        }
    }

    let mut counter = num_array.len() - 1;
    let mut sum = 0;
    
    while counter != 0{
        sum += num_array[counter];
        counter -= 1;
    }

    println!("sum of all numbers: {}", sum);

    let mut max = 0;
    
    let result = loop{
        if num_array[counter] > max {
            max = num_array[counter];
        }
            
        counter += 1;

        if counter == num_array.len(){
            break max;
        }
    };

    println!("Max Number: {}", result);
}
