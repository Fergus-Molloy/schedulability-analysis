use colored::Colorize;
use std::io::{self, Read};
mod task;

fn main() {
    // leaving main clear for arg parsing
    let mut tasks: Vec<task::Task> = Vec::new();
    get_inp(&mut tasks);
    // print all tasks so user can check details
    for i in tasks.iter() {
        println!("{}\n", i);
    }
    // print if schedulable
    let (util, max_util) = util(&tasks, Some(4.0));
    if util <= max_util {
        println!(
            "max util: {:.4}\ntotal util: {:.4}\n{}",
            max_util,
            util,
            "schedulable with utilization? true\n".green().bold()
        );
    } else {
        println!(
            "max util: {:.4}\ntotal util: {:.4}\n{}",
            max_util,
            util,
            "schedulable with utilization? false\n".red().bold()
        );
    }

    //check that it is schedulable
    let schedulable: Vec<&task::Task> = tasks.iter().filter(|x| x.r > x.t).collect();

    if schedulable.len() == 0 {
        println!(
            "{}",
            "schedulable with response time? true\n".green().bold()
        );
    } else {
        println!("{}", "schedulable with response time? false\n".red().bold());
        for x in schedulable.into_iter() {
            println!("{} fails", x);
        }
    }
}

fn get_inp(tasks: &mut Vec<task::Task>) {
    // read in tasks
    let mut inp = String::new();
    io::stdin()
        .read_to_string(&mut inp)
        .expect("unable to read line");

    // for each task
    for x in inp.lines().skip(1) {
        // split by , since csv
        let mut iter = x.split(',');
        // items should be in this order since using template
        // need to do this outside of assignment so i can calculate u
        let name = iter.next().unwrap().trim().to_string();
        let t = iter.next().unwrap().parse().unwrap();
        let c = iter.next().unwrap().parse().unwrap();
        let p = iter.next().unwrap().parse().unwrap();
        let u = c as f64 / t as f64;
        let r = 0;
        // create task and push to vec
        tasks.push(task::Task {
            name,
            t,
            c,
            p,
            u,
            r,
        });
    }
    let cloned = tasks.clone();
    for x in tasks.into_iter() {
        x.r = response(x, &cloned) as u32;
    }
}

fn util(tasks: &Vec<task::Task>, families: Option<f64>) -> (f64, f64) {
    let n = match families {
        Some(v) => v,
        None => tasks.len() as f64,
    };
    let max_util = n * ((2 as f64).powf(1 as f64 / n) - 1 as f64);
    (tasks.iter().map(|x| x.u).sum(), max_util)
}

fn response(task: &task::Task, tasks: &Vec<task::Task>) -> f64 {
    //get higher priority tasks
    let higher_p: Vec<task::Task> = tasks
        .iter()
        .filter(|x| x.p > task.p)
        .map(|x| x.clone())
        .collect();

    let mut r = 0.0;
    let mut last_r = -1.0;
    while r != last_r {
        last_r = r;
        r = task.c as f64
            + higher_p
                .iter()
                .map(|j| (r as f64 / j.t as f64).ceil() * j.c as f64)
                .sum::<f64>();
    }
    r
}
