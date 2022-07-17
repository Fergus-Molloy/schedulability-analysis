use crate::task::Task;
use colored::Colorize;

/// Finds the response time of the given task
///
/// # Arguments
///
/// `task` is the task you want to analyse
///
/// `task_set` is the set off all tasks in the system
fn response_time(task: &Task, tasks: &Vec<Task>) -> f64 {
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

pub fn test_response_time(task_set: &Vec<Task>) {
    let _: Vec<()> = task_set
        .iter()
        .map(|task| {
            let r = response_time(task, &task_set);
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
