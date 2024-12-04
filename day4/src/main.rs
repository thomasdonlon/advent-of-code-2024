use std::env;
use std::fs;

fn part1(contents: &Vec<Vec<char>>) {

    // it feels like there's some way to use string matching and counting to do this, 
    //    rather than writing out every direction, 
    //    but the diagonals make it really hard to do that. This seems easiest.

    let mut n_matches = 0;

    let mut i = 0;
    while i < contents.len() {

        let mut j = 0;
        while j < contents[i].len() {

            // count forwards
            if j + 3 < contents[i].len() {
                if (contents[i][j] == 'X') & 
                   (contents[i][j+1] == 'M') & 
                   (contents[i][j+2] == 'A') & 
                   (contents[i][j+3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count backwards
            if j > 2 {
                if (contents[i][j] == 'X') & 
                   (contents[i][j-1] == 'M') & 
                   (contents[i][j-2] == 'A') & 
                   (contents[i][j-3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count downwards
            if i + 3 < contents.len() {
                if (contents[i][j] == 'X') & 
                   (contents[i+1][j] == 'M') & 
                   (contents[i+2][j] == 'A') & 
                   (contents[i+3][j] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count upwards
            if i > 2 {
                if (contents[i][j] == 'X') & 
                   (contents[i-1][j] == 'M') & 
                   (contents[i-2][j] == 'A') & 
                   (contents[i-3][j] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count diagonal towards bottom right
            if (j + 3 < contents[i].len()) &
               (i + 3 < contents.len()) {
                if (contents[i][j] == 'X') & 
                   (contents[i+1][j+1] == 'M') & 
                   (contents[i+2][j+2] == 'A') & 
                   (contents[i+3][j+3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count diagonal towards bottom left
            if (j > 2) &
               (i + 3 < contents.len()) {
                if (contents[i][j] == 'X') & 
                   (contents[i+1][j-1] == 'M') & 
                   (contents[i+2][j-2] == 'A') & 
                   (contents[i+3][j-3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count diagonal towards top right
            if (j + 3 < contents[i].len()) &
               (i > 2) {
                if (contents[i][j] == 'X') & 
                   (contents[i-1][j+1] == 'M') & 
                   (contents[i-2][j+2] == 'A') & 
                   (contents[i-3][j+3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }

            // count diagonal towards top left
            if (j > 2) &
               (i > 2) {
                if (contents[i][j] == 'X') & 
                   (contents[i-1][j-1] == 'M') & 
                   (contents[i-2][j-2] == 'A') & 
                   (contents[i-3][j-3] == 'S') {
                    n_matches = n_matches + 1;
                   }
            }
            j = j + 1;
        }
        i = i + 1;
    }

    println!("part 1:");
    println!("number of matches: {n_matches}");
}

fn part2(contents: &Vec<Vec<char>>) {

    let mut n_matches = 0;

    let mut i = 0;
    while i < contents.len() {

        let mut j = 0;
        while j < contents[i].len() {

            // there are 4 options: 
            //    1) both left to right
            //    2) top backwards
            //    3) bottom backwards
            //    4) both backwards

            if (j + 2 < contents[i].len()) & 
               (i + 2 < contents.len()) { // only need to check once this time

                // could only check A's (have to be center tile) if I wanted to make this faster

                // check if top is there, forwards or backwards
                let mut top = false;

                if (contents[i][j] == 'M') & 
                   (contents[i+1][j+1] == 'A') & 
                   (contents[i+2][j+2] == 'S') {
                    top = true;
                   } else if (contents[i][j] == 'S') & 
                   (contents[i+1][j+1] == 'A') & 
                   (contents[i+2][j+2] == 'M') {
                    top = true;
                   }

                // check if bottom is there, forwards or backwards
                let mut bottom = false;

                if (contents[i][j+2] == 'M') & 
                   (contents[i+1][j+1] == 'A') & 
                   (contents[i+2][j] == 'S') {
                    bottom = true;
                   } else if (contents[i][j+2] == 'S') & 
                   (contents[i+1][j+1] == 'A') & 
                   (contents[i+2][j] == 'M') {
                    bottom = true;
                   }

                if top & bottom {
                    n_matches = n_matches + 1;

                }
            }
            j = j + 1;
        }
        i = i + 1;
    }

    println!("part 2:");
    println!("number of matches: {n_matches}");
}

fn main() {
    // read in the file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // make a 2D vector out of contents
    let contents: Vec<Vec<char>> = fs::read_to_string(file_path)
        .unwrap()
        .trim() //removes trailing whitespace, otherwise crash when it hits an empty line
        .split("\n")
        .map(|s| s.chars().collect()) //turn each string into a vector of chars
        .collect();  //turn into vector of strings rather than block of text, splitting on newlines

    // actual calculations
    part1(&contents);
    part2(&contents);
}
