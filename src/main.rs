use sqlite::{Connection, State, Result, Value, Error, Statement};

struct Task {
    id: u32,
    title: String,
    description: String,
    priority: String,
    due_date: String,
    status: String,
}

fn main() {
    let mut conn = Connection::open("tasks.db").unwrap();
    conn.execute(
        "create table if not exists tasks (id integer primary key autoincrement, title text, description text, date text, priority text, status text);").unwrap();

    let mut tasks: Vec<Task> = Vec::new();
    let mut id: u32 = 0;

    recover_items(&mut tasks, &mut conn);
    if tasks.len() > 0 {
        id = tasks[tasks.len()-1].id;
    }

    loop {
        show_menu();
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => create_task(&mut tasks, &mut id, &mut conn),
            2 => show_tasks(&tasks),
            3 => edit_task(&mut tasks, &mut conn),
            4 => erase_task(&mut tasks, &mut conn),
            5 => statistics(&tasks),
            6 => break,
            _ => println!(""),
        };

        tasks.clear();

        recover_items(&mut tasks, &mut conn);
        if tasks.len() > 0 {
            id = tasks[tasks.len()-1].id;
        }

    }
}

fn show_menu() {
    println!();
    println!("Task Manger");
    println!();
    println!("1. Create a task");
    println!("2. Show existing tasks");
    println!("3. Edit a task");
    println!("4. Erase a task");
    println!("5. Statistics");
    println!("6. Exit");
    println!();
}

fn create_task(tasks: &mut Vec<Task>, id: &mut u32, conn: &mut Connection) {
    println!("Title: ");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("Failed to read line");
    println!("Description: ");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Failed to read line");
    println!("Priority: ");
    let mut priority = String::new();
    std::io::stdin().read_line(&mut priority).expect("Failed to read line");
    println!("Due date: ");
    let mut due_date = String::new();
    std::io::stdin().read_line(&mut due_date).expect("Failed to read line");
    let status = String::from("TO DO");

    let query = "insert into tasks (title, description, date, priority, status) VALUES (?, ?, ?, ?, ?)";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, title.trim().into()),
                (2, description.trim().into()),
                (3, priority.trim().into()),
                (4, due_date.trim().into()),
                (5, status.trim().into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
    
    println!("Task created successfully");
}

fn show_all_tasks(tasks: &Vec<Task>) {
    println!();
    println!("All tasks: ");
    for i in 0..tasks.len() {
        println!("Title: {}", tasks[i].title);
        println!("Description: {}", tasks[i].description);
        println!("Priority: {}", tasks[i].priority);
        println!("Due date: {}", tasks[i].due_date);
        println!("Status: {}", tasks[i].status);
        println!();
    }
}

fn show_todo_tasks(tasks: &Vec<Task>) {
    println!();
    println!("TO DO tasks: ");
    for i in 0..tasks.len() {
        if tasks[i].status == "TO DO" {
            println!("Title: {}", tasks[i].title);
            println!("Description: {}", tasks[i].description);
            println!("Priority: {}", tasks[i].priority);
            println!("Due date: {}", tasks[i].due_date);
            println!("Status: {}", tasks[i].status);
            println!();
        }
    }
}

fn show_tasks(tasks: &Vec<Task>) {
    println!();
    println!("1. All tasks");
    println!("2. TO DO tasks");
    println!();
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    match option {
        1 => show_all_tasks(&tasks),
        2 => show_todo_tasks(&tasks),
        _ => println!("Invalid option"),
    }
}

fn edit_task(tasks: &mut Vec<Task>, conn: &mut Connection) {
    println!("Which task do you want to edit?");
    for i in 0..tasks.len() {
        println!("{}. {}", i+1, tasks[i].title);
    }
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: usize = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    println!();
    println!("1. Title");
    println!("2. Description");
    println!("3. Priority");
    println!("4. Due date");
    println!("5. Status");
    println!();
    let mut option2 = String::new();
    std::io::stdin().read_line(&mut option2).expect("Failed to read line");
    let option2: u32 = match option2.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let mut new_value = String::new();
    if option2 != 5 {
        println!("New value: ");
        std::io::stdin().read_line(&mut new_value).expect("Failed to read line");
    } else {
        new_value = "DONE".to_string();
    
    };

    let local_id: i64 = tasks[option-1].id as i64;

    match option2 {
        1 => edit_title(tasks, option, new_value, local_id, conn),
        2 => edit_description(tasks, option, new_value, local_id, conn),
        3 => edit_priority(tasks, option, new_value, local_id, conn),
        4 => edit_date(tasks, option, new_value, local_id, conn),
        5 => edit_status(tasks, option, new_value, local_id, conn),
        _ => println!("Invalid option"),
    };
    println!("Task updated successfully.");
}

fn edit_title(tasks: &mut Vec<Task>, option: usize, new_value: String, id: i64, conn: &mut Connection) {

    let query = "update tasks set title = ? where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, new_value.trim().into()),
                (2, id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
}

fn edit_description(tasks: &mut Vec<Task>, option: usize, new_value: String, id: i64, conn: &mut Connection) {

    let query = "update tasks set description = ? where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, new_value.trim().into()),
                (2, id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
}

fn edit_priority(tasks: &mut Vec<Task>, option: usize, new_value: String, id: i64, conn: &mut Connection) {

    let query = "update tasks set priority = ? where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, new_value.trim().into()),
                (2, id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
}

fn edit_date(tasks: &mut Vec<Task>, option: usize, new_value: String, id: i64, conn: &mut Connection) {

    let query = "update tasks set date = ? where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, new_value.trim().into()),
                (2, id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
}

fn edit_status(tasks: &mut Vec<Task>, option: usize, new_value: String, id: i64, conn: &mut Connection) {

    let query = "update tasks set status = ? where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, new_value.trim().into()),
                (2, id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };
}

fn erase_task(tasks: &mut Vec<Task>, conn: &mut Connection) {
    println!("Which task do you want to erase?");
    for i in 0..tasks.len() {
        println!("{}. {}", i+1, tasks[i].title);
    }
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: usize = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let local_id: i64 = tasks[option-1].id as i64;

    let query = "delete from tasks where id = ?";
    let mut statement = match conn.prepare(query) {
        Ok(mut statement) => {
            statement.bind::<&[(_, Value)]>(&[
                (1, local_id.into()),
            ][..]).expect("Failed to create");

           statement.next().expect("Failed to create");
        },
        Err(error) => {
            println!("Error preparing statement: {:?}", error);
        }
    };

    println!("Task erased successfully.");
}

fn statistics(tasks: &Vec<Task>) {
    println!("Statistics:");
    println!();
    println!("Total tasks: {}", tasks.len());
    let mut todo_tasks = 0;
    let mut done_tasks = 0;
    for i in 0..tasks.len() {
        if tasks[i].status == "TO DO" {
            todo_tasks += 1;
        } else {
            done_tasks += 1;
        }
    }
    println!("TO DO tasks: {}", todo_tasks);
    println!("DONE tasks: {}", done_tasks);
    println!();
    println!("Percentage of tasks completed: {}%", done_tasks*100/tasks.len());
    println!();
}

fn recover_items(tasks: &mut Vec<Task>, conn: &mut Connection) {
    let mut stmt = conn.prepare("select * from tasks").unwrap();

    while let State::Row = stmt.next().unwrap() {

        let id: i64 = stmt.read(0).unwrap();
        let title: String = stmt.read(1).unwrap();
        let description: String = stmt.read(2).unwrap();
        let priority: String = stmt.read(3).unwrap();
        let due_date: String = stmt.read(4).unwrap();
        let status: String = stmt.read(5).unwrap();

        let task = Task {
            id: id as u32,
            title: title,
            description: description,
            priority: priority,
            due_date: due_date,
            status: status,
        };

        tasks.push(task);
    }
}