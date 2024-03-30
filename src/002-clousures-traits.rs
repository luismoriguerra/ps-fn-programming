fn call_with_one<F>(func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(1)
}

fn do_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn consume_with_relish<F>(func: F)
where
    F: FnOnce() -> String,
{
    println!("Consumed: {}", func());
    println!("Delicious!");
}

fn main() {
    // trait Fn
    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);

    // trait FnMut
    let mut x = 5;
    let mut increment_x = || x += 1;
    increment_x();
    increment_x();
    println!("x = {}", x);

    let mut x2: usize = 1;
    {
        let add_two_to_x = || x2 += 2;
        do_twice(add_two_to_x);
    }

    assert_eq!(x2, 5);

    // trait FnOnce
    let xs = "hello".to_string();
    let consume_x = move || {
        let _bytes = xs.into_bytes();
    };
    consume_x();
    // consume_x(); // error: use of moved value: `consume_x`

    let xs2 = String::from("hello");
    let consume_and_return_xs2 = move || xs2;
    consume_with_relish(consume_and_return_xs2);

    // consume_and_relish can no longer be invoked at this point
}
