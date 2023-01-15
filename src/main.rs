use std::process::exit;
use std::{env, fs};

mod parser;
use parser::{parse_args, print_help_screen};
mod student;
use student::{find_valid_arrangement, Student};

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
