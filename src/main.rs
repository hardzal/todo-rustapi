use std::io;

struct Todo {
    name: String,
    done: bool,
}

impl Todo {
    fn new(name: String) -> Todo {
        Todo { name, done: false }
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    println!("=== Simple Todolist Application ===");

    loop {
        println!("\nMenu: ");
        println!("1. Add Todo");
        println!("2. Show Todos");
        println!("3. Take Action");
        println!("4. Search todo");
        println!("5. Delete Todo");
        println!("6. Delete all todo");
        println!("7. Exit");
        println!("Choose Menu (1-4): ");

        let menu = read_input();

        match menu.trim() {
            "1" => {
                println!("Enter todo title: ");
                let title = read_input();
                add_todo(&mut todos, title);
            }

            "2" => {
                println!("=== Todos ===");
                show_todo(&todos);
            }

            "3" => {
                println!("=== Take Action ===");
                show_todo(&todos);
                println!("Enter todo index to take action: ");
                let index = read_input();

                if let Ok(index) = index.trim().parse::<usize>() {
                    take_action(&mut todos, index);
                } else {
                    println!("Invalid index. Please enter a valid number.");
                }
            }

            "4" => {
                println!("Search todo: ");
            }

            "5" => {
                println!("Delete todo: ");
            }

            "6" => {
                println!("are you sure to delete all todo (y/n): ");
            }

            "7" => {
                println!("See you later!");
                break;
            }

            _ => {
                println!("Invalid menu choice. Please choose a number between 1 and 4.");
            }
        }
    }
}

fn add_todo(list: &mut Vec<Todo>, name: String) {
    let new_todo = Todo::new(name.trim().to_string());
    list.push(new_todo);
    println!("Todo {} added", name);
}

fn show_todo(list: &Vec<Todo>) {
    if list.is_empty() {
        println!("No todos found");
    } else {
        for (index, todo) in list.iter().enumerate() {
            let status = if todo.done { "[X]" } else { "[ ]" };
            println!("{} {} - {}", index + 1, status, todo.name);
        }
    }
}

fn take_action(list: &mut Vec<Todo>, index: usize) {
    println!("Data todo saat ini adalah: {}", list.len());

    if index > 0 && index <= list.len() {
        if list[index - 1].done {
            println!("Todo {} already done", index);
            return;
        }

        list[index - 1].done = true;
        println!("Todo {} marked as done", index);
    } else {
        println!("Todo {} not found", index);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
