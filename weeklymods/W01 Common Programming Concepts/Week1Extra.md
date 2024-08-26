
fn main() {
    // Section # 1 Data(data type specifically) is a King

    // Variable declaration
    let a = 10; 
    let b: i32 = 20;
    let c = 30i32; 
    let d = 30_i32; // underscores are ignored
    let res = a + b + c + d;
    
    println!("(a + b + c + d) = {}", res);

    // => Now it will become fun.
    // Numbers are data types (same as objects -> first level citizens)

    let ten = 10f32;
    let twenty = 20i32;
    let neg_ten = -10i32;

    println!("20**2 = {}", twenty.pow(2));
    println!("square root of 10 = {}", ten.sqrt());
    println!("neg_ten to positive = {}", neg_ten.abs());


    let ten = 10i32;
    println!("10 in binary (base 2) = {:b}", ten);
    println!("10 in octal (base 8) = {:o}", ten);
    println!("10 in hex (base 16) = {:x}", ten);

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("3 = {}, 30 = {}, 300 = {}",three,thirty,three_hundred);
    
    // Data type regulates how much space will be taken inside of memory

    let i8_twenty = 20i8;
    let f32_twenty = 20f32;

    println!("20 in i8 = {:b}", i8_twenty);
    println!("20 in f32 = {:b}", f32_twenty.to_bits());


    // Regarding comparisons

    // to be able to compare two values they should belong to the same type
    // forget the idea about implicit promotions.

    let x: i32 = 10;
    let y: u16 = 100;

    if x < y as i32 {
        println!("Ten is less than one hundred");
    }

    // Important idea regarding operations with numbers.
    // Remember operations will be performed by the CPU not the compiler

    let x: f32 = 1.0/0.0;
    println!("x = {}",x);
    println!("is x a valid {}",x.is_finite());

    let x: f32 = (-42f32).sqrt();
    println!("x = {}",x);
    println!("is x a valid {}",x.is_finite());

    // Just a remainder for using references.

    let a = 42;
    let r = &a;
    let b = a + *r;

    println!(" a + a = {}",b);

    

    // Section #2 : For loops world is different )))

    Remainder: Array type

    // I assume that you are comfortable, things keep in mind (fixed length, same type, it's not a vector or list!!!)

    let cities = ["Edinburg", "MCAllen", "Austin"];
    // or

    let nums: [i32; 5] = [77,111,222,333,444];  // I must specify a type and size in advance

    // or 
    let zeros = [0; 100]; // I can specify like this [default value; number of elements]

    println!("My zeros {:?} ",zeros); // if you noticed :?, this is called printing in a debug node, arrays don't have string representation, so it's a convenient way to see the inside of array (you can try delete, we will get even error)
    // What is good in Rust, there is no silencing or ignoring.

    // Accessing, same as everywhere

    let my_home = cities[0];

    println!("My home is : {}",my_home);

    // 3 types of for loops
    let nums = [1,2,3,4,5];
    println!("{:?}",nums);
    // First type
    
    // for item in collection
    // for item in collection.into_iter()

    // Used up this collection, just use when you plan to iterate once

    

    for num in nums {
        println!("{}",num);
    }

    for num in nums.into_iter(){
        println!("{}",num);
    }
    println!("{:?}",nums);

    
    // Second type: Iterating through reference, (read only approach)

    for num in &nums { // & reference, you will get to the idea to use references
        println!("{}",num);
    }

    for num in nums.iter() {
        println!("{}",num);
    }

    // Third type: Iterating when you plan to change the value

    let mut nums = [1,2,3,4,5]; // Everything should be explicit

    for num in &mut nums {
        *num += 5;
        println!("{}",num);
    }

    println!("{:?}",nums);

    for num in nums.iter_mut() {
        *num += 5;
        println!("{}",num);
    }

    // Understanding those subtle differences like this, will make you a better programmer.

    // Another for loop or so called enumerate idea, when for need for indexes

    let goal = 5;
    let nums = [5,25,45,100,5];
    
    for (idx,num) in nums.iter().enumerate() {
        if *num == goal {
            println!("Our target is under index {}",idx);
        }
    } 

        // Section 3: Functions

    // Functions are a first-class citizens. 
    // fn is a keyword used to declare them

    // Convention is snake_name for a function (surprisingly even a compiler will enforce it from you)

    // Plain function

    fn message() {
        println!("Just function");
    }
    message();

    // Function with parameters

    fn message_with_param(favnum: i32){
        println!("My favorite number is: {}",favnum);
    }
    
    message_with_param(777);

    // Its pretty much the same as in C++

    // Statements and Expressions in Functions

    // Rust distinguished statements and expressions
    // -> Statements are instructions x = 5;
    // -> Expressions are evaluated   x = 5 + 10;
    // Subtle the difference though. Key idea that statements don't return result value, 
    // expression should, which basically means it has a return type!!!! What an idea.

    // In practical terms x = y = 6 will not work :)
    // And statements closed with ;, but expressions don't

    let x = 5;
    let y = {
        // Curly braces, different scope
        let x = 3;
        x + 1 // Expression, returns the value )) 
    };

    println!("X is: {}. Y is {}",x,y);

    // Well why bother to understand , you will see this a lot in a Rust. Especially if you want to learn from other peoples code.
    

    
    fn add_five(mynum: i32) -> i32 { // notice return type after -> (very common nowdays syntax)
        mynum + 5
    }
    
    println!("Calling add_five function, relying on expression return {}",add_five(100));

    // So here comes next idea.
    // Return statement is usually explictly used when you want to return from a function earlier, based on some condition
    
    // otherwise you can rely on your last expression in the function

    // Short example on Safety in-Place

    fn subtract_five (mynum: i32) -> u8 {
        mynum as u8 - 5 // mynum - 5 not correct
    }

    println!("Calling subtract_five function, relying on expression return {}",subtract_five(6)); // try to pass 4 ))

    // Practice problems. LeetCode. With this knowledge I believe you should be able to code next problems in Rust:

    // 509. Fibonacci Number
    // https://leetcode.com/problems/fibonacci-number/

    // 278. First Bad Version
    // https://leetcode.com/problems/first-bad-version/

    // Just checking waters, no need to lookup for solutions online, 
    // if you can't dont worry )) I will disciss in class shortly.


    
}

