use std::{fs, time::Instant};

fn search_with_loops(content: &str) -> usize {
    let mut count = 0;
    let mut word = String::new();
    let mut index = 0;

    while index < content.len() {
        let ch = content.as_bytes()[index] as char;

        if ch.is_whitespace() {
            if word == "the" {
                count += 1;
            }
            word.clear();
        } else {
            word.push(ch);
        }

        index += ch.len_utf8();
    }

    if word == "the" {
        count += 1;
    }

    count
}

fn search_with_iterators(content: &str) -> usize {
    content
        .split_whitespace()
        .filter(|&word| word == "the")
        .count()
}

fn main() {
    let content = fs::read_to_string("peace.txt").expect("Error reading file");

    let start = Instant::now();
    let _ = search_with_loops(&content);
    let duration = start.elapsed();
    println!("Time taken with loops: {:?}", duration);

    let start = Instant::now();
    let _ = search_with_iterators(&content);
    let duration = start.elapsed();
    println!("Time taken with iterators: {:?}", duration);
}
