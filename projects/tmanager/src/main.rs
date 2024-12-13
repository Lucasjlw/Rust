use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::stdin;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    priority: Priority,
    due_date: NaiveDate,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

pub fn main() {
    println!("Welcome to Task Manager!");
    println!("------------------------");

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Delete task");
        println!("5. Save tasks");
        println!("6. Load tasks");
        println!("7. Exit");

        let input = get_menu_input();
        match input {
            1 => handle_add_task(&mut tasks),
            2 => handle_list_tasks(&tasks),
            3 => handle_complete_task(&mut tasks),
            4 => handle_delete_task(&mut tasks),
            5 => handle_save_tasks(&tasks),
            6 => handle_load_tasks(&mut tasks),
            _ => return,
        }
    }
}

fn get_menu_input() -> u8 {
    let input = read_input().parse().expect("Should be an integer");

    input
}

fn handle_add_task(tasks: &mut Vec<Task>) {
    println!();
    println!("What is the task title?");
    let title = read_input();

    println!();
    println!("What is the task description?");
    let description = read_input();

    println!();
    println!("What is the task priority?");
    let priority: Priority;
    match read_input().parse().expect("Should be an integer") {
        1 => priority = Priority::High,
        2 => priority = Priority::Medium,
        3 => priority = Priority::Low,
        _ => priority = Priority::Low,
    }

    println!();
    println!("When is the task due?");
    let due_date: NaiveDate = read_input().parse().expect("Should be a date");

    let new_task = Task {
        id: tasks.len().try_into().unwrap(),
        title,
        description,
        priority,
        due_date,
        completed: false,
    };

    tasks.push(new_task);
}

fn handle_list_tasks(tasks: &Vec<Task>) {
    println!();
    for task in tasks.into_iter() {
        println!(
            "{} {} {} {}",
            task.id, task.title, task.due_date, task.completed
        );
    }
}

fn handle_complete_task(tasks: &mut Vec<Task>) {
    let task_id: u32 = read_input().parse().expect("Should be an integer");

    for task in tasks.into_iter() {
        if (task.id == task_id) {
            task.completed = true;
            return;
        }
    }
}

fn handle_delete_task(tasks: &mut Vec<Task>) {
    let task_id: u32 = read_input().parse().expect("Should be an integer");

    for i in 0..tasks.len() {
        if (tasks.get(i).unwrap().id == task_id) {
            tasks.remove(i);
            return;
        }
    }
}

fn handle_save_tasks(tasks: &Vec<Task>) {
    let _ = fs::write("./tasks.json", serde_json::to_string(tasks).unwrap());
}

fn handle_load_tasks(tasks: &mut Vec<Task>) {
    let raw_data = fs::read_to_string("./tasks.json").unwrap();

    *tasks = serde_json::from_str(&raw_data).unwrap();
}

fn read_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input.trim().to_string();
}
