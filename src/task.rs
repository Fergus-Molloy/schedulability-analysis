#![allow(dead_code)]
use crate::resource::{CriticalSection, Resource};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Task {
    pub name: String,
    pub T: f64,
    pub D: f64,
    pub C: f64,
    pub P: u32,
    pub U: f64,
    pub critical_sections: Option<Vec<CriticalSection>>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let critical_sections = self.critical_sections.clone();
        match critical_sections {
            Some(cr) => write!(
                f,
                "Task {}\nT: {t}\nD: {d}\nC: {c}\nP: {p}\nU: {u}\nCritical Sections:\n {cr:?}",
                self.name,
                t = self.T,
                d = self.D,
                c = self.C,
                p = self.P,
                u = self.U,
                cr = cr,
            ),
            None => write!(
                f,
                "Task {}\nT: {t}\nD: {d}\nC: {c}\nP: {p}\nU: {u}",
                self.name,
                t = self.T,
                c = self.C,
                d = self.D,
                p = self.P,
                u = self.U,
            ),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.P.cmp(&other.P)
    }
}

pub fn get_resources(path: &str) -> Vec<CriticalSection> {
    let v = Vec::new();
    v
}

#[allow(non_snake_case)]
pub fn LL_utilissation(tasks: &Vec<Task>, families: Option<u32>) -> (f64, f64) {
    // number of families may or may not have been provided
    let n = match families {
        Some(v) => v as f64,
        None => tasks.len() as f64,
    };

    let max_util: f64 = n * ((2 as f64).powf(1.0 / n) - 1.0);
    (tasks.iter().map(|x| x.U).sum(), max_util)
}

/// Finds the response time of the given task
///
/// # Arguments
///
/// `task` is the task you want to analyse
///
/// `task_set` is the set off all tasks in the system
pub fn response_time(task: &Task, tasks: &Vec<Task>) -> f64 {
    //get higher priority tasks
    let hp: Vec<Task> = tasks
        .iter()
        .filter(|x| x.P > task.P)
        .map(|x| x.clone())
        .collect();

    let mut r = task.C;
    let mut last_r = -1.0;
    while r != last_r {
        last_r = r;
        r = task.C + hp.iter().map(|j| (r / j.T).ceil() * j.C).sum::<f64>();
    }
    r
}
