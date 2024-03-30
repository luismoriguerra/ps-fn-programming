use std::thread;

fn main() {
    let mut y = 5;
    // mutating
    let mut add_y = |x| {
        y += x;
        y
    };

    let a = add_y(5);
    println!("a = {}", a);
    println!("y = {}", y);


    let message = "hello from thread".to_string();

    // taking owner of message
    let thread = thread::spawn(move ||{
        println!("{}", message);
    });

    thread.join().unwrap();
}
