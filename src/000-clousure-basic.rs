fn main() {
    let add_one = |x| x + 1;
    let a = add_one(5);
    println!("a = {}", a);

    let x = 7;
    let print_x = || {
        println!("the value of x is: {}", x);
    };

    print_x();

    let y2 = 5;

    fn add_one2(x: i32) -> i32 {
        // clousure only works with ||{} syntax
        x + 1
    }

    let f = add_one2;
    let e = f(5);
    println!("e = {}", e);

    // call immediately
    println!("{}", (|x, y| x + y)(1, 2));

    
}
