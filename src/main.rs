use std::{
    env::args,
    fs::{self, File},
    io::{Read, Write},
};
const PATH: &str = ".todos";
struct Todo {
    id: u32,
    name: String,
}
impl Todo {
    fn new(name: &str, id: u32) -> Todo {
        Todo {
            id: id,
            name: String::from(name),
        }
    }
}
fn get_file() -> (Vec<Todo>, u32) {
    let mut todos: Vec<Todo> = Vec::new();
    let mut file: File;
    let mut buffer: String = String::new();
    let last_id: u32;
    if !std::path::Path::new(PATH).exists() {
        _ = File::create(PATH)
            .expect("Error while creating file")
            .write_all("\n".as_bytes())
            .expect("Error while writing to file");
    }
    file = File::open(PATH).expect("Error while opening file .todos");
    file.read_to_string(&mut buffer)
        .expect("Error while reading todo");
    for todo in buffer.split("\n") {
        if todo.trim().len() > 1 {
            println!("{}", todo);
            let splited = todo.split("-").collect::<Vec<&str>>();
            let id = splited[0]
                .trim()
                .parse::<u32>()
                .expect("Error while parsing");
            let name = splited[1];
            todos.push(Todo::new(name, id))
        }
    }

    match todos.last() {
        Some(todo) => last_id = todo.id,
        None => last_id = 0,
    };
    (todos, last_id)
}
fn save_to_file(todos: &Vec<Todo>) {
    let mut buffer = String::new();
    let mut first = true;
    for todo in todos {
        if first {
            buffer = format!("{} - {}", todo.id, todo.name);
            first = false;
            continue;
        }
        buffer = format!("{} \n{} - {}", buffer, todo.id, todo.name);
    }
    fs::write(PATH, buffer).expect("Error while saving the todos");
}
fn add_todo(todos: &mut Vec<Todo>, last_id: u32, name: &str) -> u32 {
    todos.push(Todo::new(name, last_id + 1));
    last_id + 1
}
fn list_todos(todos: &Vec<Todo>) {
    for todo in todos {
        println!("{} - {}", todo.id, todo.name)
    }
}
fn remove_todo(todos: &mut Vec<Todo>, id: u32) {
    let mut remove_pos: Option<usize> = None;
    for (pos, todo) in todos.iter().enumerate() {
        if todo.id == id {
            remove_pos = Some(pos);
            break;
        }
    }
    if remove_pos != None {
        todos.remove(remove_pos.unwrap());
    }
}
fn main() {
    let (mut todos, mut last_id) = get_file();
    let mut oper = String::new();
    for (pos, arg) in args().enumerate() {
        if pos == 0 {
            continue;
        }
        if arg == "-h" || arg == "--help" {
            println!("Basic commands: ");
            println!("\t -n {{name}} | Adds new todo");
            println!("\t -d {{id}} | Delete the todo");
            println!("\t -l | list the todos");
            break;
        }
        if oper == "-n" {
            last_id = add_todo(&mut todos, last_id, &arg);
        } else if oper == "-d" {
            let id = arg.trim().parse().expect("Error while parsing");
            remove_todo(&mut todos, id)
        } else if arg == "-l" {
            list_todos(&todos);
        }
        oper = arg;
    }

    save_to_file(&todos);
}
