use std::env;
use std::fs;
use regex::Regex;

fn part1(contents: &String) {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut sum = 0;
    for (_, [val1, val2]) in re.captures_iter(contents).map(|c| c.extract()) {
        let val1_i32 = val1.parse::<i32>().unwrap();
        let val2_i32 = val2.parse::<i32>().unwrap();
        sum = sum + val1_i32 * val2_i32;
    }

    println!("part 1:");
    println!("sum: {sum}");
}

fn part2(contents: &String) {

    // delete all characters between don't()'s and do()'s
    // also need to delete all characters between a trailing don't() and EOF -- this can be done by adding a "do()" to EOF
    let contents = contents.replace("\n", ""); //remove the line breaks
    let re = Regex::new(r"(don't\(\).*?do\(\))").unwrap();
    let contents_app = contents.to_owned() + r"do()";
    let after = re.replace_all(&contents_app, "").to_string();

    // then do the regex from part 1 on the result
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut sum = 0;
    for (_, [val1, val2]) in re.captures_iter(&after).map(|c| c.extract()) {
        let val1_i32 = val1.parse::<i32>().unwrap();
        let val2_i32 = val2.parse::<i32>().unwrap();
        sum = sum + val1_i32 * val2_i32;
    }

    println!("part 2:");
    println!("sum: {sum}");
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
