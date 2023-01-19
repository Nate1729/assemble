use std::cmp::Ordering;

use itertools::Itertools;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Student {
    pub name: String,
    avoid: Vec<String>,
}

impl Student {
    pub fn can_work_with(&self, other_student: &Self) -> bool {
        if other_student.avoid.contains(&self.name) {
            return false;
        }
        true
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.avoid.len() == other.avoid.len()
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.avoid.len().cmp(&other.avoid.len()))
    }
}
impl Eq for Student {}
impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.avoid.len().cmp(&other.avoid.len())
    }
}

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

pub fn find_valid_arrangement(students: &[Student], group_size: usize) -> Option<Vec<&Student>> {
    for arrangement in students.into_iter().permutations(students.len()) {
        if validate_arrangement(&arrangement, group_size) {
            return Some(arrangement);
        }
    }

    return None;
}
