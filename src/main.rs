use std::fs;

use itertools::Itertools;

mod student;
use student::Student;

fn validate_group(group: &[Student]) -> bool {
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

fn validate_student_list(students: &Vec<Student>, group_size: usize) -> bool {
    let n_groups = students.len() / group_size;

    for offset in 0..n_groups {
        let group = &students[offset * group_size..(offset + 1) * group_size];
        if !validate_group(group) {
            return false;
        }
    }
    true
}

fn main() {
    let data_in = fs::read_to_string("students.json").unwrap();
    let mut students: Vec<Student> = serde_json::from_str(&data_in).unwrap();
    students.sort();
    let student_cnt = students.len();

    for arrangement in students.into_iter().permutations(student_cnt) {
        if validate_student_list(&arrangement, 4) {
            let n_groups = student_cnt / 4;

            for offset in 0..n_groups {
                println!("=== Group {} ===", offset + 1);
                println!(
                    "{}\n{}\n{}\n{}",
                    arrangement[offset * 4].name,
                    arrangement[offset * 4 + 1].name,
                    arrangement[offset * 4 + 2].name,
                    arrangement[offset * 4 + 3].name
                )
            }
            break;
        }
    }
}
