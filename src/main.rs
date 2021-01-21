#[macro_use]
extern crate clap;
use clap::Arg;
use colored::Colorize;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
mod task;

static VERBOSE: AtomicBool = AtomicBool::new(false);

fn main() {
    // leaving main clear for arg parsing
    let mut app = clap_app!(arts =>
        (version: "1.0")
        (author: "Fergus Molloy")
        (about: "Utility for doing arts calcualtions")
        (@arg INPUT: +required "Input tasks (see template)")
        (@arg response_time: -r --response_time "Calculate schedualbility using response time")
        (@arg verbose: -v --verbose "Enable extra printing")
        (@arg implicit: -i --implicit "Deadline is implicit")
        (@arg tasks: -t --tasks "List all tasks and their properties")
    );

    app = app.arg(
        Arg::with_name("utilization")
            .takes_value(true)
            .min_values(0)
            .short("u")
            .help("Calculate schedualbility using utilization"),
    );
    let matches = app.get_matches();

    VERBOSE.fetch_or(matches.is_present("verbose"), Ordering::SeqCst);

    let file = matches.value_of("INPUT").unwrap();
    let mut tasks: Vec<task::Task> = Vec::new();
    get_inp(&mut tasks, file, matches.is_present("implicit"));

    if matches.is_present("tasks") {
        // print all tasks so user can check details
        for i in tasks.iter() {
            println!("{}\n", i);
        }
    }

    if matches.is_present("response_time") {
        println!("\n[response time----------------------------------------------------------]");
        tasks.sort_unstable_by(|a, b| b.cmp(a));
        let cloned = tasks.clone();
        let referance = &mut tasks;
        for x in referance.into_iter() {
            x.R = response(x, &cloned) as u32;
        }
        let _: Vec<()> = tasks
            .iter()
            .map(|x| {
                let s;
                if x.R > x.D {
                    s = format!(
                        "Task {} has response time {} and deadline {}",
                        x.name, x.R, x.D
                    )
                    .red();
                } else {
                    s = format!(
                        "Task {} has response time {} and deadline {}",
                        x.name, x.R, x.D
                    )
                    .cyan()
                }
                println!("{}", s);
            })
            .collect();
        if tasks.iter().any(|x| x.R > x.D) {
            println!("{}", "schedulable with response time? false".red().bold());
        } else {
            println!("{}", "schedulable with response time? true".green().bold());
        }
    }

    if matches.is_present("utilization") {
        println!("\n[utilisations-----------------------------------------------------------]");
        // print if schedulable
        let families = match matches.value_of("utilization") {
            Some(v) => Some(v.parse::<f64>().unwrap()),
            None => None,
        };

        if families.is_none() {
            log(format!("using default families (number of tasks)"));
        } else {
            log(format!("using {} familes", families.unwrap()));
        }

        let (util, max_util) = util(&tasks, families);
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
    }
}

fn get_inp(tasks: &mut Vec<task::Task>, file: &str, implicit: bool) {
    // read in tasks
    let inp = fs::read_to_string(file).expect("unable to read file");

    // for each task
    for x in inp.lines().skip(1) {
        // split by , since csv
        let mut iter = x.split(',');
        // items should be in this order since using template
        // need to do this outside of assignment so i can calculate u
        let name = iter.next().unwrap().trim().to_string();
        let t = iter.next().unwrap().parse::<u32>().unwrap();
        let d;
        if implicit {
            d = t;
            iter.next();
        } else {
            d = iter.next().unwrap().parse::<u32>().unwrap();
        }
        let c = iter.next().unwrap().parse().unwrap();
        let p = iter.next().unwrap().parse().unwrap();
        let u = c as f64 / t as f64;
        let r = 0;
        // create task and push to vec
        tasks.push(task::Task {
            name,
            T: t,
            D: d,
            C: c,
            P: p,
            U: u,
            R: r,
        });
    }
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
    log(format!(
        "Task {} with deadline {}\n{}",
        task.name, task.D, task.C
    ));
    while r != last_r {
        last_r = r;
        r = task.C as f64
            + higher_p
                .iter()
                .map(|j| {
                    log(format!("+ ⌈{}/{}⌉ * {}", r, j.T, j.C));
                    (r as f64 / j.T as f64).ceil() * j.C as f64
                })
                .sum::<f64>();
    }
    log(format!("{}", format!(" = {}", r).cyan()));
    if r > task.D as f64 {
        log(format!("{}", format!("{} > {}\n", r, task.D).red()));
    } else {
        log(format!("{}", format!("{} <= {}\n", r, task.D).green()));
    }
    r
}

fn log(s: String) {
    if VERBOSE.load(Ordering::SeqCst) {
        println!("{}", s);
    }
}
