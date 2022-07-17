use crate::task::Task;
use colored::Colorize;

#[allow(non_snake_case)]
fn LL_utilissation(tasks: &Vec<Task>, n: usize) -> (f64, f64) {
    let max_util: f64 = n as f64 * ((2 as f64).powf(1.0 / n as f64) - 1.0);
    (tasks.iter().map(|x| x.U).sum(), max_util)
}

fn calc_families(task_set: &Vec<Task>) -> usize {
    task_set.len()
}

pub fn test_utilisation(task_set: &Vec<Task>, families: bool) {
    let n = if families {
        calc_families(&task_set)
    } else {
        task_set.len()
    };

    let (util, max_util) = LL_utilissation(&task_set, n);
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
