use clap::{Parser, Subcommand};

use crate::{
    init_folder,
    model::{priority::Priority, task_state::TaskState},
    view, ConcreteTaskBuilder, ConcreteTaskRelationalManager, Date, TaskBuilder,
    TaskRelationalManager, DATABASE_PATH,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Insert {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        term: Option<String>,

        #[arg(short = 's', long)]
        task_state: Option<String>,

        #[arg(short, long)]
        priority: Option<String>,
    },

    Show {
        #[arg(short, long, conflicts_with = "id")]
        name: Option<String>,

        #[arg(short, long, conflicts_with = "name")]
        id: Option<u64>,
    },

    Modify {
        #[arg(short, long, conflicts_with = "id")]
        name: Option<String>,

        #[arg(short, long, conflicts_with = "name")]
        id: Option<u64>,

        #[arg(short = 'N', long)]
        new_name: Option<String>,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        term: Option<String>,

        #[arg(short = 's', long)]
        task_state: Option<String>,

        #[arg(short, long)]
        priority: Option<String>,
    },
    Delete {
        #[arg(short, long, conflicts_with = "id")]
        name: Option<String>,

        #[arg(short, long, conflicts_with = "name")]
        id: Option<u64>,
    },
    Init {},
}

fn insert(
    name: Option<String>,
    description: Option<String>,
    term: Option<String>,
    task_state: Option<String>,
    priority: Option<String>,
) {
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);

    let mut task_build = ConcreteTaskBuilder::new();
    match name {
        Some(i) => task_build = task_build.set_name(i),
        None => {}
    }
    match description {
        Some(i) => task_build = task_build.set_description(i),
        None => {}
    }
    match term {
        Some(i) => {
            let date = Date::from_string(i);
            match date {
                Ok(j) => match j {
                    Some(k) => task_build = task_build.set_term(k),
                    None => {}
                },
                Err(err) => panic!("{}", err),
            }
        }
        None => {}
    }
    match task_state {
        Some(i) => match TaskState::from_string(&i) {
            Some(j) => task_build = task_build.set_task_state(j),
            None => {}
        },
        None => {}
    }
    match priority {
        Some(i) => match Priority::from_string(&i) {
            Some(j) => task_build = task_build.set_priority(j),
            None => {}
        },
        None => {}
    }
    let task = task_build.get_task();
    let mut result = conn.insert_task(&task);
    loop {
        match result {
            Ok(()) => return,
            Err(_) => result = conn.insert_task(&task),
        }
    }
}

fn show(name: Option<String>, id: Option<u64>) {
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);

    if let Some(i) = name {
        let task = conn.get_task_by_name(&i);
        match task {
            Some(i) => {
                let tasks = vec![i];
                view::show_query_tasks(&tasks);
            }
            None => {
                println!("Doesent exists task with this name");
            }
        }
        return;
    }

    if let Some(i) = id {
        let task = conn.get_task_by_id(i);
        match task {
            Some(i) => {
                let tasks = vec![i];
                view::show_query_tasks(&tasks);
            }
            None => {
                println!("Doesent exists task with this id");
            }
        }
        return;
    }

    let tasks = conn.get_tasks();
    match tasks {
        Some(i) => {
            view::show_query_tasks(&i);
        }
        None => {
            panic!("Problem with query the tasks");
        }
    }
}

fn modify(
    name: Option<String>,
    id: Option<u64>,
    new_name: Option<String>,
    description: Option<String>,
    term: Option<String>,
    task_state: Option<String>,
    priority: Option<String>,
) {
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);
    let mut task_build = ConcreteTaskBuilder::new();
    let mut task;
    let mut init = false;
    if let Some(i) = name.clone() {
        task = conn.get_task_by_name(&i);
        match task {
            Some(j) => {
                task_build = task_build.set_by_task(j);
                init = true;
            }
            None => {
                panic!("Dosent find task with this name");
            }
        }
    }
    if let Some(i) = id {
        task = conn.get_task_by_id(i);
        match task {
            Some(j) => {
                task_build = task_build.set_by_task(j);
                init = true;
            }
            None => {
                panic!("Dosent find task with this name");
            }
        }
    }

    if let Some(i) = new_name {
        task_build = task_build.set_name(i);
    }

    if let Some(i) = description {
        task_build = task_build.set_description(i);
    }

    if let Some(i) = term {
        match Date::from_string(i) {
            Ok(j) => {
                if let Some(k) = j {
                    task_build = task_build.set_term(k);
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    if let Some(i) = task_state {
        match TaskState::from_string(&i) {
            Some(j) => task_build = task_build.set_task_state(j),
            None => panic!("Invalid Task State"),
        }
    }

    if let Some(i) = priority {
        match Priority::from_string(&i) {
            Some(j) => task_build = task_build.set_priority(j),
            None => panic!("Invalid Priority"),
        }
    }

    if !init {
        panic!("Dosent have passed a key");
    }
    if let Some(i) = id {
        let mut times = 0;
        let mut result = conn.delete_task_by_id(i);
        loop {
            match result {
                Ok(()) => break,
                Err(_) => {
                    if times > 100000 {
                        panic!("Not capable to delete");
                    }
                    times += 1;
                    result = conn.delete_task_by_id(i)
                }
            }
        }
    }
    if let Some(i) = name {
        let mut times = 0;
        let mut result = conn.delete_task_by_name(&i);
        loop {
            match result {
                Ok(()) => break,
                Err(_) => {
                    if times > 100000 {
                        panic!("Not capable to delete");
                    }
                    times += 1;
                    result = conn.delete_task_by_name(&i)
                }
            }
        }
    }
    let mut times = 0;
    let mut result = conn.insert_task(&task_build.get_task());
    loop {
        match result {
            Ok(()) => break,
            Err(_) => {
                if times > 100000 {
                    panic!("Not capable to delete");
                }
                times += 1;
                result = conn.insert_task(&task_build.get_task());
            }
        }
    }
}

fn delete(name: Option<String>, id: Option<u64>) {
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);
    if let Some(i) = name {
        match conn.delete_task_by_name(&i) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        }
    }
    if let Some(i) = id {
        match conn.delete_task_by_id(i) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        }
    }
}

fn init() {
    init_folder();
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);
    conn.init_db();
    match conn.check_table("tasks") {
        Ok(1) => println!("TODO initialized"),
        _ => panic!("Cannot init the todo list"),
    }
}

pub fn exec() {
    match Cli::parse().command {
        Commands::Insert {
            name,
            description,
            term,
            task_state,
            priority,
        } => {
            insert(name, description, term, task_state, priority);
        }

        Commands::Show { name, id } => {
            show(name, id);
        }
        Commands::Modify {
            name,
            id,
            new_name,
            description,
            term,
            task_state,
            priority,
        } => {
            modify(name, id, new_name, description, term, task_state, priority);
        }
        Commands::Delete { name, id } => delete(name, id),
        Commands::Init {} => init(),
    }
}
