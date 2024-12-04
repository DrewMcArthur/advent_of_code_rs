use std::io::Read;

const EXPECTED: i32 = 2769675;

fn main() {
    let (mut l1, mut l2) = get_input();
    l1.sort();
    l2.sort();
    let mut total = 0;
    for i in 0..l1.len() {
        let distance = l1[i] - l2[i];
        total += distance.abs();
    }
    assert_eq!(total, EXPECTED);
    println!("total distance: {}", total);

    let similarty: i32 = l1
        .iter()
        .map(|x| x * l2.iter().filter(|y| *y == x).count() as i32)
        .sum();
    println!("Similarity: {}", similarty);
}

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    contents.split("\n").into_iter().for_each(|line| {
        let mut line = line.split("   ");
        let val = line.next().unwrap();
        let val = val.trim().parse().expect("couldn't parse val: {val}");
        l1.push(val);
        let val = line.next().unwrap();
        let val = val.trim().parse().expect("couldn't parse val: {val}");
        l2.push(val);
    });
    (l1, l2)
}
