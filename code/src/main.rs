struct TodoItem {
    task: String,
    completed: bool
}

fn show_data(todo_list: &Vec<TodoItem>){
    println!("Task list:");
    for i in 0..todo_list.len(){
        if todo_list[i].completed {
            println!("({i}) {} [DONE]", todo_list[i].task);
        } else {
            println!("({i}) {}", todo_list[i].task);
        }
    }
}

fn insert_todo(todo_list: &mut Vec<TodoItem>){
    loop {
        let mut line = String::new();
        println!("Enter your new task (write \"stop\" to return):");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim().to_string();

        if line == "stop" {
            break;
        }

        let todo_item = TodoItem {task: line, completed: false};
        todo_list.push(todo_item);
    }
}

fn set_as_done(todo_list: &mut Vec<TodoItem>) {
    let mut task_id = String::new();
    println!("Write the ID of the task you want to mark as done:");
    let _ = std::io::stdin().read_line(&mut task_id).unwrap();
    let task_id = task_id.to_lowercase().trim().to_string();

    let task_id : u32 = match task_id.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Not valid number");
            return;
        }
    };

    if task_id >= todo_list.len() as u32 {
        println!("The number doesn't correspond to an id");
        return;
    }

    todo_list[task_id as usize].completed = true;
    println!("Task ({task_id}) DONE");
}


fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    loop {
        let mut line = String::new();
        println!("========================");
        println!("What do you wan to do? :");
        println!("(1) Show the list");
        println!("(2) Add new tasks");
        println!("(3) Mark data as done");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "1" => show_data(&todo_list),
            "2" => insert_todo(&mut todo_list),
            "3" => set_as_done(&mut todo_list),
            _ => println!("Not valid option")
        }
    }
}
