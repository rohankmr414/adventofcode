use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filepath = args.get(1).unwrap();
    let path = std::path::Path::new(filepath);
    let (mut n1, mut n2) = read_input(path);
    n1.sort();
    n2.sort();
    println!("Total distance: {:?}", find_distance(&n1, &n2));
    println!("Similarity score: {:?}", find_similarity_score(&n1, &n2));
}

fn read_input(path: &Path) -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut n1: Vec<i32> = vec![];
    let mut n2: Vec<i32> = vec![];
    for line in input.lines() {
        let k = line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
        n1.push(k[0]);
        n2.push(k[1]);
    }
    (n1, n2)
}

fn find_distance(n1: &Vec<i32>, n2: &Vec<i32>) -> i32 {
    let mut s = 0;
    for i in 0..n1.len() {
        let d = n2[i] - n1[i];
        s += d.abs();
    }
    s
}

fn find_similarity_score(n1: &Vec<i32>, n2: &Vec<i32>) -> i32 {
    let mut s = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in 0..n2.len() {
        map.entry(n2[i]).and_modify(|x| *x += 1).or_insert(1);
    }

    for i in 0..n1.len() {
        if map.contains_key(&n1[i]) {
            s += map[&n1[i]] * n1[i]
        }
    }
    s
}
