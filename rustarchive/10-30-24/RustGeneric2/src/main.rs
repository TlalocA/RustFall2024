#[derive(Debug)]
struct Person<T, U>{
    name:U,
    age:T,
}

impl<T, U> Person<T, U> {
    fn new(name:U, age:T) -> Person<T, U> {
        Person {
            age:age,
            name:name,
            
        }
    }
    
    fn add(&mut self, extra:T){
        self.age = extra;
    }
}

fn main(){
    let mut p = Person::new("eight", 888);
    p.add(8);
    
    println!("{:?}", p);
}