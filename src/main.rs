use std::process::exit;
use std::{env, fs};

use itertools::Itertools;

mod parser;
use parser::{parse_args, print_help_screen};
mod student;
use student::Student;

fn validate_group(group: &[&Student]) -> bool {
    for pair in group.iter().combinations(2) {
        if !pair[0].can_work_with(pair[1]) {
            return false;
        }
        if !pair[1].can_work_with(pair[0]) {
            return false;
        }
    }
    true
}

fn validate_arrangement(students: &Vec<&Student>, group_size: usize) -> bool {
    let n_groups = students.len() / group_size;

    for offset in 0..n_groups {
        let group = &students[offset * group_size..(offset + 1) * group_size];
        if !validate_group(group) {
            return false;
        }
    }
    true
}

fn find_valid_arrangement(students: &[Student], group_size: usize) -> Option<Vec<&Student>> {
    for arrangement in students.into_iter().permutations(students.len()) {
        if validate_arrangement(&arrangement, group_size) {
            return Some(arrangement);
        }
    }

    return None;
}

fn main() {
    // Retrieve file path
    let file_path = match parse_args(env::args()) {
        Ok(s) => s,
        Err(s) => {
            eprintln!("{}", s);
            print_help_screen();
            exit(1);
        }
    };

    let data_in = fs::read_to_string(file_path).unwrap();
    let mut students: Vec<Student> = serde_json::from_str(&data_in).unwrap();
    students.sort();
    let student_cnt = students.len();
    let group_size = 4;

    // Find solution
    match find_valid_arrangement(&students, group_size) {
        None => println!("I couldn't find a solution!"),
        Some(a) => {
            let n_groups = student_cnt / 4;

            for offset in 0..n_groups {
                println!("=== Group {} ===", offset + 1);
                println!(
                    "{}\n{}\n{}\n{}",
                    a[offset * 4].name,
                    a[offset * 4 + 1].name,
                    a[offset * 4 + 2].name,
                    a[offset * 4 + 3].name
                )
            }
        }
    }
}
