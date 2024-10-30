fn classic_example_stack() {
    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Stack<T> {
            Stack { items: Vec::new() }
        }
        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }
    }

    let mut stack = Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("My stack holds {:?}", stack);
    stack.pop();
    println!("My stack holds {:?}", stack);
}

fn main(){
    classic_example_stack();
}