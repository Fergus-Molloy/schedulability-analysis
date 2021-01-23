use colored::Colorize;
use std::fs;
mod resource;
mod task;

fn main() {
    let tasks = get_inp("tasks", None);
    for x in tasks.iter() {
        let r = response_time(&x, &tasks);
        println!("{}\nR: {}\n", x, r);
    }
}

fn get_inp(file: &str, resources: Option<&str>) -> Vec<task::Task> {
    // read in tasks
    let inp = fs::read_to_string(file).expect("unable to read file");

    let mut task_set = Vec::new();

    //skip title line
    for x in inp.lines().skip(1) {
        // split by , since csv
        let mut iter = x.split(',');

        // items are in this order since users are using  a template
        let name = iter.next().unwrap().trim().to_string();

        let t: f64 = iter.next().unwrap().parse().unwrap();

        let d_iter = iter.next().unwrap().trim();
        let d = match d_iter.is_empty() {
            true => t,
            false => d_iter.parse().unwrap(),
        };

        let c = iter.next().unwrap().parse().unwrap();

        let p_iter = iter.next().unwrap().trim();
        let p = match p_iter.is_empty() {
            true => 0,
            false => p_iter.parse().unwrap(),
        };

        let u = c / t;

        //TODO: handle resources
        let critical_sections = match resources {
            None => None,
            Some(path) => Some(get_resources(path)),
        };

        // create task and push to task_set
        task_set.push(task::Task {
            name,
            T: t,
            D: d,
            C: c,
            P: p,
            U: u,
            critical_sections,
        });
    }
    if task_set[0].P == 0 {
        // deadline monotonic ordering is the same as rate monotonic ordering for implicit tasks
        // as the deadline for implicit tasks is their period
        deadline_monotonic_ordering(&mut task_set);
    }
    task_set
}

fn get_resources(path: &str) -> Vec<resource::CriticalSection> {
    let v = Vec::new();
    v
}

fn deadline_monotonic_ordering(task_set: &mut Vec<task::Task>) {
    task_set.sort_by(|task_a, task_b| task_a.D.partial_cmp(&task_b.D).unwrap());
    let mut p = task_set.len() as u32;
    for x in 0..(task_set.len()) {
        task_set[x].P = p;
        p -= 1;
    }
}

fn rate_monotonic_ordering(task_set: &mut Vec<task::Task>) {
    task_set.sort_by(|task_a, task_b| task_a.T.partial_cmp(&task_b.T).unwrap());
    let mut p = task_set.len() as u32;
    for x in 0..(task_set.len()) {
        task_set[x].P = p;
        p -= 1;
    }
}

#[allow(non_snake_case)]
fn LL_utilissation(tasks: &Vec<task::Task>, families: Option<f64>) -> (f64, f64) {
    // number of families may or may not have been provided
    let n = match families {
        Some(v) => v,
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
fn response_time(task: &task::Task, tasks: &Vec<task::Task>) -> f64 {
    //get higher priority tasks
    let hp: Vec<task::Task> = tasks
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
