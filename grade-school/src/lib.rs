use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School<'a> {
    grades: BTreeMap<u32, BTreeSet<&'a str>>
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {grades: BTreeMap::<u32, BTreeSet<&str>>::new()}
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let s = self.grades.entry(grade).or_insert(BTreeSet::<&str>::new());
        (*s).insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().map(|x| *x).collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|x| x.iter().map(|y| String::from(*y)).collect())
    }
}
