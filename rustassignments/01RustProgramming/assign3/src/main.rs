fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    } else if guess > secret{
        1
    } else{
        -1
    }
}

fn main() {
    // mut variables to simulate input
    let mut guess: i32 = 10;
    let mut secret: i32 = 16;

    if check_guess(guess, secret) == 0{
        println!("Guess was correct!");
    } else if check_guess(guess, secret) == 1{
        println!("Guess was too high!");
    } else if check_guess(guess, secret) == -1{
        println!("Guess was too low!");
    }
}
