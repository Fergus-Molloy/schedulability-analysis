#[macro_use]
extern crate log;
extern crate env_logger;

use colored::Colorize;
use env_logger::Builder;
use log::{debug, error, info, trace, Record};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
mod resource;
mod task;

#[derive(Debug, StructOpt)]
#[structopt(name = "Schedualability Analysis")]
struct Opt {
    /// CSV file containing the tasks
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,

    /// Enable extra printing
    #[structopt(short, long)]
    verbose: bool,
    /// Enable debug printing
    #[structopt(short, long)]
    debug: bool,

    /// Perfrom response time analysis on the task set
    #[structopt(short, long)]
    response_time: bool,

    /// Perform L&L utilisation testing
    /// Optionally provide the number of families to use
    #[structopt(short, long)]
    utilisation: Option<Option<u32>>,
}

fn main() {
    //let tasks = get_inp("tasks", None);
    //for x in tasks.iter() {
    //    let r = task::response_time(&x, &tasks);
    //    println!("{}\nR: {}\n", x, r);
    //}
    command_line();
}

fn command_line() {
    // Get command line inputs
    let opt = Opt::from_args();

    // Set up logger
    let mut b = Builder::from_default_env();
    if opt.debug {
        // Use a fancy formatter
        let formatter = |buf: &mut env_logger::fmt::Formatter, record: &Record| {
            let mut style = buf.default_level_style(record.level());
            style.set_bold(true);
            writeln!(buf, "{}:{}", style.value(record.level()), record.args())
        };
        b.format(formatter).filter(None, log::LevelFilter::Debug);
    } else if opt.verbose {
        // Don't want to print level info as i will be using it for working
        b.format(|buf, record| writeln!(buf, "{}", record.args()))
            .filter(None, log::LevelFilter::Info);
    }
    b.init();

    // Create task set
    let mut task_set = get_inp(&opt.input, None);

    if opt.debug {
        for task in task_set.iter() {
            debug!("\n{}\n", task);
        }
    }

    match opt.utilisation {
        Some(n) => {
            let (util, max_util) = task::LL_utilissation(&task_set, n);
            if util <= max_util {
                let string = format!("Utilisation test passes: {:.3} <= {:.3}", util, max_util)
                    .green()
                    .bold();
                println!("{}", string);
            } else {
                let string = format!("Utilisation test fails: {:.3} >= {:.3}", util, max_util)
                    .red()
                    .bold();
                println!("{}", string);
            }
        }
        None => (),
    }

    if opt.response_time {
        let _: Vec<()> = task_set
            .iter()
            .map(|task| {
                let r = task::response_time(task, &task_set);
                if r <= task.D {
                    let string = format!(
                        "Response time for task {} passes with RT of {} (deadline is {})",
                        task.name, r, task.D
                    )
                    .blue();
                    println!("{}", string);
                } else {
                    let string = format!(
                        "Response time for task {} fails with RT of {} (deadline is {})",
                        task.name, r, task.D
                    )
                    .red()
                    .bold();
                    println!("{}", string);
                }
            })
            .collect();
    }
}

fn get_inp(file: &PathBuf, resources: Option<&str>) -> Vec<task::Task> {
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
            Some(path) => Some(task::get_resources(path)),
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
