#[path = "./task_a.rs"]
mod task_a;

#[path = "./task_b.rs"]
mod task_b;

pub fn main() {
    task_a::main();
    task_b::main();
}
