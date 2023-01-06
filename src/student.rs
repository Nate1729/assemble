use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Student {
    pub name: String,
    avoid: Vec<String>
}

impl Student {
    pub fn can_work_with(&self, other_student: &Self) -> bool {
        if other_student.avoid.contains(&self.name) {
            return false
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
