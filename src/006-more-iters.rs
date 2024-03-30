fn main() {
    let v = vec![1, 2, 3];
    let iter = v.iter();
    let even: Vec<_> = iter.filter(|x| *x % 2 == 0).collect();

    for item in even {
        println!("{}", item);
    }

    let v = vec![1, 2, 3];
    let iter = v.iter();
    // here map returns i32, so filter can be used without *x
    let even_squares: Vec<_> = iter.map(|x| x * x).filter(|x| x % 2 == 0).collect();

    for item in even_squares {
        println!("{}", item);
    }
}
