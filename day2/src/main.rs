use std::env;
use std::fs;

fn inc_or_dec(avec: &Vec::<i32>) -> bool {

    if avec.is_sorted_by(|a, b| a < b) {
        true
    } else if avec.is_sorted_by(|a, b| a > b) {
        true
    } else {
        false
    }
}

fn within_2(avec: &Vec::<i32>) -> bool {
    let mut diff = Vec::new();

    let mut i = 0;
    while i < avec.len() - 1 {
        diff.push((avec[i + 1] - avec[i]).abs());
        i = i + 1;
    }
    
    diff.sort();
    if diff.last() > Some(&3) {
        false
    } else {
        true
    }
}

fn part1(contents: &String) {

    let mut n_safe = 0;

    for line in contents.lines() {
        let aline: Vec<i32> = line // this is pretty rough to just process text
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let inc_dec = inc_or_dec(&aline);
        let w2 = within_2(&aline);

        // println!("inc or dec: {inc_dec}, within 2: {w2}"); // debugging

        if inc_dec && w2 {
            n_safe = n_safe + 1;
        }
    }

    println!("part 1");
    println!("number of safe levels: {n_safe}\n");
}

fn part2(contents: &String) {

    let mut n_safe = 0;

    for line in contents.lines() {
        let aline: Vec<i32> = line // this is pretty rough to just process text
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        // try the initial case
        let inc_dec = inc_or_dec(&aline);
        let w2 = within_2(&aline);

        if inc_dec && w2 {
            n_safe = n_safe + 1;
        } else { // try removing the levels one-at-a-time
            // there is almost assuredly an intelligent way to do this by checking individual elements
            //    but we are okay with being slow and stupid 
            let mut i = 0;

            while i < aline.len() {
                let mut tmp_aline = aline.to_vec(); // copy the vector
                tmp_aline.remove(i);

                let inc_dec = inc_or_dec(&tmp_aline);
                let w2 = within_2(&tmp_aline);

                if inc_dec && w2 {
                    n_safe = n_safe + 1;
                    break;
                }

                i = i + 1
            }
        }
    }

    println!("part 1");
    println!("number of safe levels: {n_safe}\n");
}

fn main() {

    // read in the file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();

    // actual calculations
    part1(&contents);
    part2(&contents);

}