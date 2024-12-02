use std::env;
use std::fs;

fn part1(contents: &String) {

    //some additional parsing stuff 
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let l: Vec<String> = line.split_whitespace().map(str::to_string).collect();

        vec1.push(l[0].parse::<i32>().unwrap());
        vec2.push(l[1].parse::<i32>().unwrap());
    }

    // sort and compute sum of differences
    vec1.sort();
    vec2.sort();

    let mut sum = 0;

    let mut i = 0;

    while i < vec1.len() {
        sum = sum + (vec1[i] - vec2[i]).abs();
        i = i + 1;
    }

    println!("part 1");
    println!("sum: {sum}\n");
}

fn part2(contents: &String) {

    //some additional parsing stuff 
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let l: Vec<String> = line.split_whitespace().map(str::to_string).collect();

        vec1.push(l[0].parse::<i32>().unwrap());
        vec2.push(l[1].parse::<i32>().unwrap());
    }

    // compute similarity score
    let mut sim_score = 0;

    let mut i = 0;

    while i < vec1.len() {
        let n_occ = vec2.iter().filter(|&n| *n == vec1[i]).count() as i32;
        sim_score = sim_score + n_occ * vec1[i];
        i = i + 1;
    }

    println!("part 2");
    println!("similarity score: {sim_score}\n");
}

fn main() {

    // part 1

    // parsing
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();

    // actual calculations
    part1(&contents);
    part2(&contents);

}