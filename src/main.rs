use std::io::Error;
use std::io::Write;
use std::sync::atomic::AtomicI32;

struct Todo {
    id: i32,
    todo: String,
    completed: bool,
}

impl Todo {
    fn display(&self) -> String {
        format!(
            "\nID: {}\nTodo: {}\nIs Completed: {}\n",
            self.id,
            self.todo.trim(),
            self.completed
        )
    }
}

static NEXT_ID: AtomicI32 = AtomicI32::new(1);

fn main() {
    println!("Welcome to the Todo App!");

    let mut todo_list: Vec<Todo> = Vec::new();
    let mut user_choice: String = String::new();

    loop {
        println!("[1] Create a todo");
        println!("[2] View all todos");
        println!("[3] Update a todo");
        println!("[4] Delete a todo");
        println!("[5] Exit");
        println!("=====================");
        print!("Choose an option: ");
        std::io::stdout().flush().expect("Failed to flush");

        user_choice.clear();

        std::io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        match user_choice.trim() {
            "1" => {
                todo_list.push(create_todo());
            }
            "2" => {
                for todo in &todo_list {
                    println!("{}", todo.display());
                }
            }
            "3" => {
                let todo_id =
                    read_user_input("Enter todo id to update: ").expect("Failed to read user input");

                let result = todo_list
                    .iter()
                    .position(|todo| todo.id == todo_id.parse::<i32>().unwrap());

                match result {
                    Some(index) => {
                        let updated_todo: Todo = update_todo(&todo_list[index]);
                        todo_list[index] = updated_todo;
                    }
                    None => println!("Todo with ID {} not found!", todo_id),
                }
            }
            "4" => {
                let todo_id =
                    read_user_input("Enter todo id to delete: ").expect("Failed to read user input");

                let result = todo_list
                    .iter()
                    .position(|todo| todo.id == todo_id.parse::<i32>().unwrap());

                match result {
                    Some(index) => {
                        todo_list.remove(index);
                    }
                    None => println!("Todo with ID {} not found!", todo_id),
                }

            }
            "5" => {
                break;
            }
            _ => {
                println!("Invalid option!\n");
            }
        }
    }
}

fn create_todo() -> Todo {
    let id: i32 = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let todo: String = read_user_input("Enter todo: ").expect("Failed to read user input");

    Todo {
        id,
        todo,
        completed: false,
    }
}

fn update_todo(todo: &Todo) -> Todo {
    let updated_todo = read_user_input("Update todo: ").expect("Failed to read user input");
    let updated_status =
        read_user_input("Is completed [y/n]: ").expect("Failed to read user input");

    Todo {
        id: todo.id,
        todo: updated_todo,
        completed: match updated_status.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => todo.completed,
        },
    }
}

fn read_user_input(prompt: &str) -> Result<String, Error> {
    print!("{}", prompt);
    std::io::stdout().flush()?;

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
