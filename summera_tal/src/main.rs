/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 * Edit: Filip Körlof <korlot@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input.lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();          // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.

    /* add code here ... */
    /*take out the first line and check how many numbers there are */
    let totalnumbers :i32 = lines[0].parse().unwrap();
    let halfnumbers :i32 = (totalnumbers +1)/2;

    let mut number_vector : Vec<i32> = vec![0;halfnumbers as usize];

    let mut numberlist :Vec<i32> = lines[1]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    numberlist.sort();
    numberlist.reverse();
    
    /*control if there is a lower number in the vector and switch with the number*/
    for &numbers in &numberlist {
        for control_number in &mut number_vector {
            if numbers > *control_number {
                *control_number = numbers;
                break;
            }
        }
    }

    let mut finalnumber: i32 = 0;

    for summation in number_vector {
        finalnumber += summation;
    }

    print!{"{}", finalnumber};

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}