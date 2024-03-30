// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 3 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn customIter005() {
    let counter = Counter::new();
    for i in counter.take(10) {
        println!("{}", i);
    }

    let counter_sum = Counter::new();
    println!("Sum: {}", counter_sum.sum::<u32>());

    let mut counter_next = Counter::new();
    println!("{:?}", counter_next.next().unwrap());
    println!("{:?}", counter_next.next().unwrap());

    let counter = Counter::new();
    let counter_iter = counter.into_iter();
    for num in counter_iter {
        println!("{}", num);
    }
}
