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
    let task_set = get_inp(&opt.input, None);

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

    // use title line to check which parameters have been given and in what order
    // below is all the possible parameter options
    // let col_ordering = ["N", "T", "D", "C", "P", "CS"];
    let mut col_ordering = Vec::with_capacity(6);
    let first: String = inp.lines().take(1).collect();
    for heading in first.split(',') {
        match heading.trim() {
            "N" => col_ordering.push("N"),
            "T" => col_ordering.push("T"),
            "D" => col_ordering.push("D"),
            "C" => col_ordering.push("C"),
            "P" => col_ordering.push("P"),
            "CS" => col_ordering.push("CS"),
            _ => panic!("unrecognised column name {}", heading),
        }
    }

    //skip title line since it has been processed
    let mut task_set: Vec<task::Task> = inp
        .lines()
        .skip(1)
        .map(|x| {
            // split by , since file is csv
            let mut iter = x.split(',');

            // initialise varibales to impossible values
            let mut name: &str = "";
            let mut t: f64 = 0.0;
            let mut d = 0.0;
            let mut c = 0.0;
            let mut p = 0;
            let mut critical_sections = None;

            // setup iterator varibales
            let mut col = col_ordering.iter();
            let mut next = col.next();
            //while there is a column to process
            while next.is_some() {
                match next.unwrap() {
                    &"N" => {
                        name = iter.next().unwrap().trim();
                        debug!("name: {}", name);
                    }
                    &"T" => {
                        t = iter.next().unwrap().parse().unwrap();
                        debug!("T: {}", t);
                    }
                    &"D" => {
                        let d_iter = iter.next().unwrap().trim();
                        if d_iter.is_empty() {
                            if t == 0.0 {
                                // D column is present but no D is given (D is implicit) but T has not been processed yet
                                d = 0.0;
                            } else {
                                // D is not given but there is a T to use
                                d = t;
                            }
                        } else {
                            // D is given
                            d = d_iter.parse().unwrap();
                        }
                        debug!("D: {}", d);
                    }
                    &"C" => {
                        let inp = iter.next().unwrap();
                        c = inp.parse().unwrap();
                        debug!("C: {}", c);
                    }
                    &"P" => {
                        let p_iter = iter.next().unwrap().trim();
                        debug!("P: {}", p_iter);
                        if p_iter.is_empty() {
                            // P column is there but no value is given
                            p = 0;
                        } else {
                            p = p_iter.parse().unwrap();
                        }
                    }
                    &"CR" => {
                        let cr_iter = iter.next().unwrap();
                        critical_sections = if cr_iter.is_empty() {
                            None
                        } else {
                            Some(task::get_resources(cr_iter))
                        };
                    }
                    _ => panic!("something has gone horribly wrong, God is dead"),
                }
                next = col.next();
            }

            // Check varibales to ensure they are instantiated properly
            // Note: it does not matter if P is not instantiated as we will perform deadline monotonic
            // priority ordering if any task has a priority 0

            // It is critical that d is instantiated
            let d = if d == 0.0 { t } else { d };

            // If C or T is not given it is impossible to recover
            if c == 0.0 {
                panic!("A computation time must be given");
            }

            if t == 0.0 {
                panic!("A period must be given");
            }

            // Not having a name makes RTA very unclear about which tasks pass and which tasks fail
            if name.is_empty() {
                warn!("You should name your tasks so you can distinguish them");
            }

            let u = c / t;

            //TODO: handle resources

            // create task and push to task_set
            task::Task {
                name: name.to_string(),
                T: t,
                D: d,
                C: c,
                P: p,
                U: u,
                critical_sections,
            }
        })
        .collect();

    // check to see if any task has a priority 0
    if task_set.iter().any(|task| task.P == 0) {
        info!("Running deadline monotonic priority ordering");
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
