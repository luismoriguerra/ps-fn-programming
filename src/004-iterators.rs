

pub fn iterators004() {
    let v = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    while index < v.len() {
        println!("from while {}", v[index]);
        index += 1;
    }

    v.iter().for_each(|i| println!("from interator {}", i));

    let sum_of_squares: i32 = v.iter().map(|&x| x * x).sum::<i32>();

    println!("sum of squares: {}", sum_of_squares);

    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() {
        println!("from for loop {}", i);
        // *i = 1; // error: cannot assign to `*i` which is behind a `&` reference
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in v.iter_mut() {
        println!("from for loop {}", i);
        *i = 1;
    }

    let v = vec![1, 2, 3, 4, 5];
    let v2: Vec<_> = v.into_iter().map(|x| x * 2).collect();

    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    println!("{:?}", v_iter.next()); // will print Some(1)
    println!("{:?}", v_iter.next()); // will print Some(2)
    println!("{:?}", v_iter.next()); // will print Some(3)
    println!("{:?}", v_iter.next()); // will print None

    let v_iter = v.iter();
    for val in v_iter {
        println!("{:?}", val);
    }
}
