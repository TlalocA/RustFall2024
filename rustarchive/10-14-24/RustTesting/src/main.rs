fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mul(x:i32, y:i32) -> i32 {
    let mut total = 0;

    for i in 0..(y as usize){
        total = add(total, x);
    }
    total
}

fn expo(base:i32, power:i32) -> f32 {
    if power < 0{
        return 1.0/expo(base, -power);
    }

    if power == 0 {
        return 1.0;
    }

    let mut total = 1;
    for i in 0..(power as usize){
        total = mul(total, base);
    }

    total as f32
}

fn main() {
   println!("2 + 2 = {}", add(2, 2));
   println!("5 * 2 = {}", mul(5, 2));
   println!("5 ^ 2 = {}", expo(5, 2));
   println!("5 ^ -2 = {}", expo(5, -2));

   //println!("5 + 1/3 = {}", add(2, expo(3, -1)));

   /*
   let mut x: f32 = 5.0;
   let bad_val: f32 = 1.0/3.0;
   
   for _ in 0..10 {
        x = x+bad_val;
        println!("5 + 1/3 = {}", x);
        x = x-bad_val;
        println!("x - 1/3 = 5", x);

   }
    */
}

#[cfg(test)]

mod tests {
    use super::*;

    mod test_add_funct{
        use super::*;

        #[test]
        fn test_add() {
            // assert_eq! 
            assert_eq!(add(2, 2), 4);
            
        }

        #[test]
        fn test_mul() {
            assert_eq!(mul(2, 3), 6);
        }
    
        #[test]
        fn test_expo() {
            assert_eq!(expo(2, 3), 8.0);
            assert_eq!(expo(5, -2), 0.04);
            assert!(f32::abs(0.3333 - expo(3, -1)) < 0.005);
        }
    }

    mod test_add_multi {
        use super::*;

        #[test]
        fn test_add_multiple() {
            let test_cases = vec![
                (1, 1, 2),
                (0, 0, 0),
                (-1, 1, 0),
                (100, -50, 50)
            ];
            
            for (a, b, expected) in test_cases {
                assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
            }
        }
    }
    
}