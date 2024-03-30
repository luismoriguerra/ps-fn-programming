// this s func is a higher order function that returns a closure
fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let double = multiplier(2);
    let result = double(5);

    println!("The result is {}", result);
}
