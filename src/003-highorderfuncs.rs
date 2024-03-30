#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let x = Some(5);
    let x = None;

    let y = x.unwrap_or_else(|| {
        let z = 10;
        z * 2
    });

    println!("{}", y);


    let mut list = [
        Rectangle { width: 30, height: 50 },
        Rectangle { width: 10, height: 20 },
        Rectangle { width: 40, height: 60 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:?}", list);
}
