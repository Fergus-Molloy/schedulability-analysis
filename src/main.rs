use colored::Colorize;
use std::fs;
mod resource;
mod task;

fn main() {
    let tasks = get_inp("tasks", None);
    for x in tasks {
        print!("{}", x);
    }
}

fn get_inp(file: &str, resources: Option<&str>) -> Vec<task::Task> {
    // read in tasks
    let inp = fs::read_to_string(file).expect("unable to read file");
    let mut tasks = Vec::new();

    //skip title line
    for x in inp.lines().skip(1) {
        // split by , since csv
        let mut iter = x.split(',');

        // items should be in this order since users are using  a template
        let name = iter.next().unwrap().trim().to_string();

        let t = iter.next().unwrap().parse::<u32>().unwrap();

        let d_iter = iter.next().unwrap().trim();
        let d = match d_iter.is_empty() {
            true => None,
            false => Some(d_iter.parse().unwrap()),
        };

        let c = iter.next().unwrap().parse().unwrap();

        let p_iter = iter.next().unwrap().trim();
        let p = match p_iter.is_empty() {
            true => None,
            false => Some(p_iter.parse().unwrap()),
        };

        let u = c as f64 / t as f64;

        let critical_sections = match resources {
            None => None,
            Some(path) => Some(get_resources(path)),
        };
        // create task and push to vec
        tasks.push(task::Task {
            name,
            T: t,
            D: d,
            C: c,
            P: p,
            U: u,
            critical_sections,
        });
    }
    tasks
}

fn get_resources(path: &str) -> Vec<resource::CriticalSection> {
    let v = Vec::new();
    v
}

fn util(tasks: &Vec<task::Task>, families: Option<f64>) -> (f64, f64) {
    let n = match families {
        Some(v) => v,
        None => tasks.len() as f64,
    };
    let max_util = n * ((2 as f64).powf(1 as f64 / n) - 1 as f64);
    (tasks.iter().map(|x| x.U).sum(), max_util)
}

fn response(task: &task::Task, tasks: &Vec<task::Task>) -> f64 {
    //get higher priority tasks
    let higher_p: Vec<task::Task> = tasks
        .iter()
        .filter(|x| x.P > task.P)
        .map(|x| x.clone())
        .collect();

    let mut r = task.C as f64;
    let mut last_r = -1.0;
    while r != last_r {
        last_r = r;
        r = task.C as f64
            + higher_p
                .iter()
                .map(|j| (r as f64 / j.T as f64).ceil() * j.C as f64)
                .sum::<f64>();
    }
    r
}
