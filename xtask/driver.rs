use std::env::args;

mod compile;
mod diff;
mod run;

fn main() {
    let task = args().nth(1);
    match task.as_deref() {
        Some("run") => run::main(),
        Some("compile") => compile::main(),
        Some("diff") => diff::main(),
        Some(unknown) => println!("Unknown task: {}", unknown),
        None => println!("No task specified"),
    }
}
