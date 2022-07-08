use std::env::args;

struct Todo {
    name: String,
    state: bool,
}
impl Todo {
    fn new(name: &str) -> Todo {
        Todo {
            name: String::from(name),
            state: false,
        }
    }
}

fn main() {
    for (pos, arg) in args().enumerate() {
        if pos == 0 {
            continue;
        }
        if arg == "-h" || arg == "--help"{
            println!("");
            break;
        }
        }
}
