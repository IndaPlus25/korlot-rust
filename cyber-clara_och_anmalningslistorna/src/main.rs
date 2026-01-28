/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 * Edit: Filip Körlof <korlot@kth.se>
 */

use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
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


    /*take out number of different names */

    let number_of_names: i32 = lines[0].parse().unwrap();
    
    /*Create mutable lists for names and surname */
    let mut full_names: HashSet<String> = HashSet::new();

    let mut full_name:String = lines[1].clone() + " " + &lines[1 + number_of_names as usize].clone();
    full_names.insert(full_name);

    let mut lines_marker: i32 = 2;

    /*code for iterate through names, and controlling both lists of spoken name and surname are unique */
    while lines_marker <= number_of_names {
        full_name = lines[lines_marker as usize].clone() + " " + &lines[lines_marker as usize + number_of_names as usize].clone();
        full_names.insert(full_name);
        lines_marker += 1;
    }

    let mut unique_names: i32 = 0;

    for unique in 0..full_names.len() {
        unique_names += 1;
    }

    println!("{}", unique_names);

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}