#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32){
    /*
    for idx in low..=high{
        *total += idx;
        //println!("{}", idx);
    }
    */

    let n: i32 = high-low;
    *total = (n*(n+1))/2;

    
}

fn main(){
    let low = 0;
    let high = 100;
    let mut total = 0;
    sum(&mut total, low, high);

    println!("Total: {}", total);
 
    
}