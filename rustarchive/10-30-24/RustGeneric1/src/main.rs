// < > acts a placeholder variable for generic values
// these variables can be any valid value, even structures
fn largest<T>(el1:T, el2:T) -> T{
    el1 // just returns 1st element, cannot preform binary operations?
}

fn main(){
    //let res = largest(5,10);
    let res = largest(1, 5);
    println!("{:?}", res);
    let res = largest('z', 'a');
    println!("{:?}", res);
    let res = largest(1.11, 20.3);
    println!("{:?}", res);
    let res = largest("Hello".to_string(), "abhabdhajd".to_string());
    println!("{:?}", res);
    
}