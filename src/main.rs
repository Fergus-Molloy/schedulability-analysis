#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Builder;
use log::{debug, error, info, trace, Record};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use scheduling::response::test_response_time;
use scheduling::task::{get_resources, Task};
use scheduling::utilisation::test_utilisation;

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

    /// Assume tasks have implicit deadlines
    #[structopt(long)]
    implicit: bool,

    /// Perfrom response time analysis on the task set
    #[structopt(short, long)]
    response_time: bool,

    /// Perform L&L utilisation testing
    /// Optionally provide the number of families to use
    #[structopt(short, long)]
    utilisation: bool,

    #[structopt(short, long)]
    families: bool,
}

fn main() {
    command_line();
}

fn init_logger(options: (bool, bool)) {
    let (verbose, debug) = options;
    // Set up logger
    let mut b = Builder::from_default_env();
    if debug {
        // Use a fancy formatter
        let formatter = |buf: &mut env_logger::fmt::Formatter, record: &Record| {
            let mut style = buf.default_level_style(record.level());
            style.set_bold(true);
            writeln!(buf, "{}: {}", style.value(record.level()), record.args())
        };
        b.format(formatter).filter(None, log::LevelFilter::Trace);
    } else if verbose {
        // Don't want to print level info as i will be using it for working
        b.format(|buf, record| writeln!(buf, "{}", record.args()))
            .filter(None, log::LevelFilter::Info);
    }
    b.init();
}

fn command_line() {
    // Get command line inputs
    let opt = Opt::from_args();

    init_logger((opt.verbose, opt.debug));

    // Create task set
    let task_set = get_inp(&opt.input, None, opt.implicit);

    if opt.debug {
        for task in task_set.iter() {
            debug!("\n{}\n", task);
        }
    }

    if opt.utilisation {
        test_utilisation(&task_set, opt.families);
    }

    if opt.response_time {
        test_response_time(&task_set);
    }
}

fn set_column_order(line: String) -> Vec<String> {
    let mut col_ordering = Vec::with_capacity(6);
    for heading in line.split(',') {
        match heading.trim() {
            "N" => col_ordering.push("N".into()),
            "T" => col_ordering.push("T".into()),
            "D" => col_ordering.push("D".into()),
            "C" => col_ordering.push("C".into()),
            "P" => col_ordering.push("P".into()),
            "CS" => col_ordering.push("CS".into()),
            _ => panic!("unrecognised column name {}", heading),
        }
    }
    debug!("col ordering: {:?}", col_ordering);
    col_ordering
}

fn get_inp(file: &PathBuf, resources: Option<&str>, implicit: bool) -> Vec<Task> {
    // read in tasks
    trace!("Reading input");
    let inp = fs::read_to_string(file).expect("unable to read file");
    let mut implicit = implicit;

    // use title line to check which parameters have been given and in what order
    // below is all the possible parameter options
    // let col_ordering = ["N", "T", "D", "C", "P", "CS"];
    let first: String = inp.lines().take(1).collect();
    let col_ordering = set_column_order(first);

    //skip title line since it has been processed
    let mut task_set: Vec<Task> = inp
        .lines()
        .skip(1)
        .map(|x| {
            // split by , since file is csv
            let mut iter = x.split(',');

            // initialise varibales to impossible values
            let mut name: &str = "";
            let mut t = 0.0;
            let mut d = 0.0;
            let mut c = 0.0;
            let mut p = 0;
            let mut critical_sections = None;

            // setup iterator varibales
            let mut col = col_ordering.iter();
            let mut next = || col.next();
            //while there is a column to process
            while let Some(value) = next() {
                match &(value)[..] {
                    "N" => {
                        name = iter.next().unwrap().trim();
                        trace!("name: {}", name);
                    }
                    "T" => {
                        t = match iter.next().unwrap().parse() {
                            Ok(val) => val,
                            Err(_) => panic!("Could not parse T (should be f64)"),
                        };
                        trace!("T: {}", t);
                    }
                    "D" => {
                        let d_iter = iter.next().unwrap().trim();
                        // allows user to have a D col with no value in
                        if !d_iter.is_empty() {
                            d = match iter.next().unwrap().parse() {
                                Ok(val) => val,
                                Err(_) => panic!("Could not parse D (should be f64)"),
                            };
                        } else if !implicit {
                            // if any D is not present all will be implicit
                            info!("D not given switching to implicit deadlines");
                            implicit = true;
                        }
                        trace!("D: {}", d);
                    }
                    "C" => {
                        c = match iter.next().unwrap().parse() {
                            Ok(val) => val,
                            Err(_) => panic!("Could not parse C (should be f64)"),
                        };
                        trace!("C: {}", c);
                    }
                    "P" => {
                        let p_iter = iter.next().unwrap().trim();
                        trace!("P: {}", p_iter);
                        if !p_iter.is_empty() {
                            p = match iter.next().unwrap().parse() {
                                Ok(val) => val,
                                Err(_) => panic!("Could not parse P (should be u32)"),
                            };
                        }
                    }
                    "CR" => {
                        let cr_iter = iter.next().unwrap();
                        critical_sections = if cr_iter.is_empty() {
                            None
                        } else {
                            Some(get_resources(cr_iter))
                        };
                        trace!("CR: {:?}", critical_sections);
                    }
                    _ => panic!("Something has gone horribly wrong, God is dead"),
                }
            }

            // Check varibales to ensure they are initialised properly
            // Note: it does not matter if P is not initialised as we will perform
            // deadline monotonic priority ordering if any task has a priority 0

            // deadline = period if implicit deadline of 0 is impossible so implicit mode is
            // implied
            let d = if d == 0.0 || implicit { t } else { d };

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

            //TODO: handle resources

            // create task and push to task_set
            Task {
                name: name.to_string(),
                T: t,
                D: d,
                C: c,
                P: p,
                U: c / t,
                critical_sections,
            }
        })
        .collect();

    // check to see if any task has a priority 0
    if task_set.iter().any(|task| task.P == 0) {
        // deadline monotonic ordering is the same as rate monotonic ordering for implicit tasks
        // as the deadline for implicit tasks is their period
        deadline_monotonic_ordering(&mut task_set);
    }

    task_set
}

/// Performs deadline monotonic priority ordering in place on the task set
/// the task set will be in priority order after this function has finished
///
/// Deadline monotonic priority ordering places tasks with a shorter deadline at a higher priority
/// than tasks with a longer deadline
fn deadline_monotonic_ordering(task_set: &mut Vec<Task>) {
    info!("Using deadline monotonic priority ordering");
    task_set.sort_by(|task_a, task_b| task_a.D.partial_cmp(&task_b.D).unwrap());
    let mut p = task_set.len() as u32;
    for x in 0..(task_set.len()) {
        task_set[x].P = p;
        p -= 1;
    }
}

/// Performs rate monotonic priority ordering in place on the task set.
/// The task set will be in priority order after this function is finished.
///
/// Rate monotonic priority ordering places tasks with a shorter period at a higher priority than
/// tasks with a longer period. If the task set has implicit deadlines this is the same as
/// deadline monotonic priority ordering
///
/// In general this should not be used as deadline monotonic priority ordering is at least as good if not
/// better for all task sets.
fn rate_monotonic_ordering(task_set: &mut Vec<Task>) {
    info!("Using rate monotonic ordering");
    task_set.sort_by(|task_a, task_b| task_a.T.partial_cmp(&task_b.T).unwrap());
    let mut p = task_set.len() as u32;
    for x in 0..(task_set.len()) {
        task_set[x].P = p;
        p -= 1;
    }
}
