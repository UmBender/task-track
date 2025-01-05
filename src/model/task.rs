use rand;
use rusqlite::{params, Connection};
use std::error::Error;
use std::fs;

use crate::Date;
use crate::Priority;
use crate::TaskState;

const DATABASE_PATH: &str = "./.todo_list/info.db3";
pub const DATABASE_DIR_PATH: &str = "./.todo_list";
pub const DATABASE_PATH_TEST: &str = "./.todo_list_test/test.db";
pub const DATABASE_DIR_TEST_PATH: &str = "./.todo_list_test";
pub trait TaskBuilder {
    fn new() -> Self;
    fn reset(&self) -> Self;
    fn set_priority(&self, priority: Priority) -> Self;
    fn set_task_state(&self, task_state: TaskState) -> Self;
    fn set_description(&self, description: String) -> Self;
    fn set_name(&self, name: String) -> Self;
    fn set_term(&self, date: Date) -> Self;
    fn get_task(&mut self) -> Task;
}

pub struct ConcreteTaskBuilder {
    task: Task,
}

impl TaskBuilder for ConcreteTaskBuilder {
    fn new() -> Self {
        let id = rand::random::<u64>();
        let task_builder = ConcreteTaskBuilder {
            task: Task {
                id,
                name: format!("TASK:{}", id),
                description: None,
                modification: Date {
                    day: 0,
                    month: 0,
                    year: 0,
                },
                term: None,
                task_state: None,
                priority: None,
            },
        };
        return task_builder;
    }

    fn reset(&self) -> Self {
        return ConcreteTaskBuilder { task: Task::new() };
    }

    fn set_priority(&self, priority: Priority) -> Self {
        let mut new_task = self.task.clone();
        new_task.priority = Some(priority);
        return ConcreteTaskBuilder { task: new_task };
    }

    fn set_task_state(&self, task_state: TaskState) -> Self {
        let mut new_task = self.task.clone();
        new_task.task_state = Some(task_state);
        return ConcreteTaskBuilder { task: new_task };
    }

    fn set_description(&self, description: String) -> Self {
        let mut new_task = self.task.clone();
        new_task.description = Some(description);
        return ConcreteTaskBuilder { task: new_task };
    }

    fn set_name(&self, name: String) -> Self {
        let mut new_task = self.task.clone();
        new_task.name = name;
        return ConcreteTaskBuilder { task: new_task };
    }

    fn set_term(&self, date: Date) -> Self {
        let mut new_task = self.task.clone();
        new_task.term = Some(date);
        return ConcreteTaskBuilder { task: new_task };
    }

    fn get_task(&mut self) -> Task {
        let mut new_task = self.task.clone();
        new_task.modification = Date::get_local_date();
        self.task = Task::new();
        return new_task;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: u64,
    name: String,
    description: Option<String>,
    modification: Date,
    term: Option<Date>,
    task_state: Option<TaskState>,
    priority: Option<Priority>,
}

fn description_from_string(description: String) -> Option<String> {
    match description.as_str() {
        "None" => None,
        _ => Some(description),
    }
}

impl Task {
    pub fn get_id(&self) -> u64 {
        return self.id;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
    pub fn get_description(&self) -> String {
        return match self.description.clone() {
            Some(i) => i,
            None => String::from("None"),
        };
    }
    pub fn get_modification(&self) -> String {
        return self.modification.to_string();
    }
    pub fn get_term(&self) -> String {
        return match self.term.clone() {
            Some(i) => i.to_string(),
            None => String::from("None"),
        };
    }
    pub fn get_state(&self) -> String {
        return match self.task_state.clone() {
            Some(i) => i.to_string(),
            None => String::from("None"),
        };
    }

    pub fn get_priority(&self) -> String {
        return match self.priority.clone() {
            Some(i) => i.to_string(),
            None => String::from("None"),
        };
    }

    fn new() -> Self {
        let id = rand::random::<u64>();
        return Task {
            id,
            name: format!("TASK:{}", id),
            description: None,
            modification: Date {
                day: 0,
                month: 0,
                year: 0,
            },
            term: None,
            task_state: None,
            priority: None,
        };
    }
}

#[cfg(test)]
mod tests_task {
    use super::*;

    #[test]
    fn test_creation_random_id() {
        let mut constructor = ConcreteTaskBuilder::new()
            .set_name(String::from("Teste1"))
            .set_term(
                Date::from_string(String::from("01-01-2025"))
                    .unwrap()
                    .unwrap(),
            )
            .set_priority(Priority::Low)
            .set_task_state(TaskState::Pending);
        let task1 = constructor.get_task();

        constructor = constructor
            .set_name(String::from("Teste1"))
            .set_term(
                Date::from_string(String::from("01-01-2025"))
                    .unwrap()
                    .unwrap(),
            )
            .set_priority(Priority::Low)
            .set_task_state(TaskState::Pending);
        let task2 = constructor.get_task();

        assert_ne!(task1.id, task2.id)
    }
    #[test]
    fn test_get_name() {
        let task = ConcreteTaskBuilder::new()
            .set_name(String::from("Task Test"))
            .get_task();
        assert_eq!(task.get_name(), String::from("Task Test"));
    }

    #[test]
    fn test_get_description() {
        let task = ConcreteTaskBuilder::new()
            .set_description(String::from("This is a test activitie"))
            .get_task();
        assert_eq!(
            task.get_description(),
            String::from("This is a test activitie"),
            "Description would be the seted description on the builder"
        );

        let task = ConcreteTaskBuilder::new().get_task();
        assert_eq!(
            task.get_description(),
            String::from("None"),
            "With unseted description should return \"None\""
        );
    }

    #[test]
    fn test_get_modification() {
        let now_date = Date::get_local_date();
        let task = ConcreteTaskBuilder::new().get_task();
        assert_eq!(
            now_date.to_string(),
            task.get_modification(),
            "Now time should be equal to a task created now"
        );
    }

    #[test]
    fn test_get_term() {
        let date = String::from("1-12-2000");
        let task = ConcreteTaskBuilder::new()
            .set_term(Date::from_string(date.clone()).unwrap().unwrap())
            .get_task();
        assert_eq!(
            task.get_term(),
            date.clone(),
            "Term date should be equal to same string"
        );
    }

    #[test]
    fn test_get_state() {
        let task_pending = ConcreteTaskBuilder::new()
            .set_task_state(TaskState::Pending)
            .get_task();

        assert_eq!(
            task_pending.get_state(),
            TaskState::Pending.to_string(),
            "Pending task state should be equal to \"pending\""
        );

        assert_ne!(
            task_pending.get_state(),
            TaskState::InProgress.to_string(),
            "Pending task state should be equal to \"pending\""
        );

        assert_ne!(
            task_pending.get_state(),
            TaskState::Ended.to_string(),
            "Pending task state should be equal to \"pending\""
        );

        assert_ne!(
            task_pending.get_state(),
            String::from("None"),
            "Pending task state should be equal to \"pending\""
        );
        let task_in_progress = ConcreteTaskBuilder::new()
            .set_task_state(TaskState::InProgress)
            .get_task();

        assert_eq!(
            task_in_progress.get_state(),
            TaskState::InProgress.to_string(),
            "In progress task state should be equal to \"in progress\""
        );

        assert_ne!(
            task_in_progress.get_state(),
            TaskState::Pending.to_string(),
            "In progress task state should be equal to \"in progress\""
        );

        assert_ne!(
            task_in_progress.get_state(),
            TaskState::Ended.to_string(),
            "In progress task state should be equal to \"in progress\""
        );

        assert_ne!(
            task_in_progress.get_state(),
            String::from("None"),
            "In progress task state should be equal to \"in progress\""
        );

        let task_ended = ConcreteTaskBuilder::new()
            .set_task_state(TaskState::Ended)
            .get_task();

        assert_eq!(
            task_ended.get_state(),
            TaskState::Ended.to_string(),
            "Ended task state should be equal to \"ended\""
        );

        assert_ne!(
            task_ended.get_state(),
            TaskState::Pending.to_string(),
            "Ended task state should be equal to \"ended\""
        );

        assert_ne!(
            task_ended.get_state(),
            TaskState::InProgress.to_string(),
            "Ended task state should be equal to \"ended\""
        );

        assert_ne!(
            task_ended.get_state(),
            String::from("None"),
            "Ended task state should be equal to \"ended\""
        );

        let task_unseted_state = ConcreteTaskBuilder::new().get_task();

        assert_eq!(
            task_unseted_state.get_state(),
            String::from("None"),
            "Unseted state task state should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_state.get_state(),
            TaskState::Pending.to_string(),
            "Unseted state task state should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_state.get_state(),
            TaskState::InProgress.to_string(),
            "Unseted state task state should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_state.get_state(),
            TaskState::Ended.to_string(),
            "Unseted state task state should be equal to \"None\""
        );
    }

    #[test]
    fn test_get_priority() {
        let task_low = ConcreteTaskBuilder::new()
            .set_priority(Priority::Low)
            .get_task();

        assert_eq!(
            task_low.get_priority(),
            Priority::Low.to_string(),
            "Low priority task should be equal to \"low\""
        );

        assert_ne!(
            task_low.get_priority(),
            Priority::Normal.to_string(),
            "Low priority task should be equal to \"low\""
        );

        assert_ne!(
            task_low.get_priority(),
            Priority::High.to_string(),
            "Low priority task should be equal to \"low\""
        );

        assert_ne!(
            task_low.get_priority(),
            Priority::Urgent.to_string(),
            "Low priority task should be equal to \"low\""
        );

        assert_ne!(
            task_low.get_priority(),
            String::from("None"),
            "Low priority task should be equal to \"low\""
        );

        let task_normal = ConcreteTaskBuilder::new()
            .set_priority(Priority::Normal)
            .get_task();

        assert_eq!(
            task_normal.get_priority(),
            Priority::Normal.to_string(),
            "Normal priority task should be equal to \"normal\""
        );

        assert_ne!(
            task_normal.get_priority(),
            Priority::Low.to_string(),
            "Normal priority task should be equal to \"normal\""
        );

        assert_ne!(
            task_normal.get_priority(),
            Priority::High.to_string(),
            "Normal priority task should be equal to \"normal\""
        );

        assert_ne!(
            task_normal.get_priority(),
            Priority::Urgent.to_string(),
            "Normal priority task should be equal to \"normal\""
        );

        assert_ne!(
            task_normal.get_priority(),
            String::from("None"),
            "Normal priority task should be equal to \"normal\""
        );

        let task_high = ConcreteTaskBuilder::new()
            .set_priority(Priority::High)
            .get_task();

        assert_eq!(
            task_high.get_priority(),
            Priority::High.to_string(),
            "High priority task should be equal to \"high\""
        );

        assert_ne!(
            task_high.get_priority(),
            Priority::Low.to_string(),
            "High priority task should be equal to \"high\""
        );

        assert_ne!(
            task_high.get_priority(),
            Priority::Normal.to_string(),
            "High priority task should be equal to \"high\""
        );

        assert_ne!(
            task_high.get_priority(),
            Priority::Urgent.to_string(),
            "High priority task should be equal to \"high\""
        );

        assert_ne!(
            task_high.get_priority(),
            String::from("None"),
            "High priority task should be equal to \"high\""
        );

        let task_urgent = ConcreteTaskBuilder::new()
            .set_priority(Priority::Urgent)
            .get_task();

        assert_eq!(
            task_urgent.get_priority(),
            Priority::Urgent.to_string(),
            "Urgent priority task should be equal to \"urgent\""
        );

        assert_ne!(
            task_urgent.get_priority(),
            Priority::Low.to_string(),
            "Urgent priority task should be equal to \"urgent\""
        );

        assert_ne!(
            task_urgent.get_priority(),
            Priority::Normal.to_string(),
            "Urgent priority task should be equal to \"urgent\""
        );

        assert_ne!(
            task_urgent.get_priority(),
            Priority::High.to_string(),
            "Urgent priority task should be equal to \"urgent\""
        );

        assert_ne!(
            task_urgent.get_priority(),
            String::from("None"),
            "Urgent priority task should be equal to \"urgent\""
        );

        let task_unseted_priority = ConcreteTaskBuilder::new().get_task();

        assert_eq!(
            task_unseted_priority.get_priority(),
            String::from("None"),
            "Unseted priority task should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_priority.get_priority(),
            Priority::Low.to_string(),
            "Unseted priority task should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_priority.get_priority(),
            Priority::Normal.to_string(),
            "Unseted priority task should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_priority.get_priority(),
            Priority::High.to_string(),
            "Unseted priority task should be equal to \"None\""
        );

        assert_ne!(
            task_unseted_priority.get_priority(),
            Priority::Urgent.to_string(),
            "Unseted priority task should be equal to \"None\""
        );
    }
}

pub trait TaskRelationalManager {
    fn insert_task(&self, task: &Task) -> Result<(), Box<dyn Error>>;
    fn get_tasks(&self) -> Option<Vec<Task>>;
    fn get_task_by_id(&self, task_id: u64) -> Option<Task>;
    fn get_task_by_name(&self, task_name: &String) -> Option<Task>;
    fn delete_task_by_name(&self, task_name: &String) -> Result<(), Box<dyn Error>>;
    fn delete_task_by_id(&self, task_id: u64) -> Result<(), Box<dyn Error>>;
}

pub struct ConcreteTaskRelationalManager {
    conn: Connection,
}

impl TaskRelationalManager for ConcreteTaskRelationalManager {
    fn insert_task(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        let parameters = params![
            task.get_id().to_string(),
            task.get_name(),
            task.get_description(),
            task.get_modification(),
            task.get_term(),
            task.get_state(),
            task.get_priority()
        ];

        self.conn.execute(
            "INSERT INTO tasks(id, name, description, date, term, task_state,priority) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            parameters
        ).expect("There was an error trying to insert");

        return Ok(());
    }

    fn get_tasks(&self) -> Option<Vec<Task>> {
        let mut select_tasks = self.conn.prepare("SELECT * FROM tasks").unwrap();
        let tasks_iter = select_tasks
            .query_map([], |row| {
                let id: String = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                let description: String = row.get(2).unwrap();
                let date: String = row.get(3).unwrap();
                let term: String = row.get(4).unwrap();
                let task_state: String = row.get(5).unwrap();
                let priority: String = row.get(6).unwrap();
                let task = Task {
                    id: id.parse().unwrap(),
                    name,
                    description: description_from_string(description),
                    modification: Date::from_string(date).unwrap().unwrap(),
                    term: Date::from_string(term).unwrap(),
                    task_state: TaskState::from_string(&task_state),
                    priority: Priority::from_string(&priority),
                };
                return Ok(task);
            })
            .unwrap();
        let mut tasks_vec = Vec::new();
        for i in tasks_iter {
            tasks_vec.push(i.unwrap());
        }
        return Some(tasks_vec);
    }

    fn get_task_by_id(&self, task_id: u64) -> Option<Task> {
        let mut select_tasks = self
            .conn
            .prepare("SELECT * FROM tasks WHERE id = ?1")
            .unwrap();
        let tasks_iter = select_tasks
            .query_map([task_id.to_string()], |row| {
                let id: String = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                let description: String = row.get(2).unwrap();
                let date: String = row.get(3).unwrap();
                let term: String = row.get(4).unwrap();
                let task_state: String = row.get(5).unwrap();
                let priority: String = row.get(6).unwrap();
                let task = Task {
                    id: id.parse().unwrap(),
                    name,
                    description: description_from_string(description),
                    modification: Date::from_string(date).unwrap().unwrap(),
                    term: Date::from_string(term).unwrap(),
                    task_state: TaskState::from_string(&task_state),
                    priority: Priority::from_string(&priority),
                };
                return Ok(task);
            })
            .unwrap();
        let mut tasks_vec = Vec::new();
        for i in tasks_iter {
            tasks_vec.push(i.unwrap());
        }

        if tasks_vec.is_empty() {
            return None;
        }

        return Some(tasks_vec[0].clone());
    }

    fn get_task_by_name(&self, task_name: &String) -> Option<Task> {
        let mut select_tasks = self
            .conn
            .prepare("SELECT * FROM tasks WHERE name = ?1")
            .unwrap();
        let tasks_iter = select_tasks
            .query_map([task_name.clone()], |row| {
                let id: String = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                let description: String = row.get(2).unwrap();
                let date: String = row.get(3).unwrap();
                let term: String = row.get(4).unwrap();
                let task_state: String = row.get(5).unwrap();
                let priority: String = row.get(6).unwrap();
                let task = Task {
                    id: id.parse().unwrap(),
                    name,
                    description: description_from_string(description),
                    modification: Date::from_string(date).unwrap().unwrap(),
                    term: Date::from_string(term).unwrap(),
                    task_state: TaskState::from_string(&task_state),
                    priority: Priority::from_string(&priority),
                };
                return Ok(task);
            })
            .unwrap();
        let mut tasks_vec = Vec::new();
        for i in tasks_iter {
            tasks_vec.push(i.unwrap());
        }

        if tasks_vec.is_empty() {
            return None;
        }

        return Some(tasks_vec[0].clone());
    }

    fn delete_task_by_name(&self, task_name: &String) -> Result<(), Box<dyn Error>> {
        let deleted_tasks = self.conn.execute(
            "DELETE FROM tasks WHERE name = ?1",
            params![task_name.clone()],
        );
        match deleted_tasks {
            Ok(i) if i > 0 => return Ok(()),
            _ => Err("The task neither exists or it was not able to delete".into()),
        }
    }

    fn delete_task_by_id(&self, task_id: u64) -> Result<(), Box<dyn Error>> {
        let deleted_tasks = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id]);
        match deleted_tasks {
            Ok(i) if i > 0 => return Ok(()),
            _ => Err("The task neither exists or it was not able to delete".into()),
        }
    }
}

impl ConcreteTaskRelationalManager {
    pub fn new(path: &str) -> Self {
        let mut conn = Connection::open(path);
        loop {
            match conn {
                Ok(i) => {
                    return Self { conn: i };
                }
                Err(_) => conn = Connection::open(path),
            }
        }
    }

    pub fn init_db(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS tasks(id TEXT PRIMARY KEY, name TEXT, description TEXT, date TEXT, term TEXT, task_state TEXT, priority TEXT)", []).unwrap();
    }

    fn check_table(&self, table_name: &str) -> Result<i64, Box<dyn Error>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT COUNT (*)
                FROM sqlite_master
                WHERE type = 'table' AND name = ?1",
            )
            .unwrap();
        let count = stmt.query_row(params![table_name], |row| row.get::<_, i64>(0))?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests_manager {

    use super::*;

    fn init_folder() {
        fs::create_dir(DATABASE_DIR_TEST_PATH)
            .expect("Was not possible to create the directory for de database");
    }

    fn remove_test_files() {
        fs::remove_file(DATABASE_PATH_TEST).expect("The file was not created in the write place");
        fs::remove_dir(DATABASE_DIR_TEST_PATH)
            .expect("The directory was not there, or not was capable to delete it");
    }

    #[test]
    fn test_creation_database() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();
        let count = conn.check_table("tasks");
        assert_eq!(count.unwrap(), 1, "The number of tables should be one");
        remove_test_files();
    }

    #[test]
    fn test_insertion() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();

        let mut stmt = conn.conn.prepare("SELECT COUNT (*) FROM tasks ").unwrap();
        let count_before = stmt
            .query_row(params![], |row| row.get::<_, i64>(0))
            .unwrap();

        assert_eq!(count_before, 0, "The number of insertions should be zero");

        let task = ConcreteTaskBuilder::new().get_task();

        let _ = conn
            .insert_task(&task)
            .expect("The insertion should be okay");

        let mut stmt = conn.conn.prepare("SELECT COUNT (*) FROM tasks ").unwrap();
        let count_after = stmt
            .query_row(params![], |row| row.get::<_, i64>(0))
            .unwrap();

        assert_eq!(count_after, 1, "The number of insertions should be zero");
        remove_test_files();
    }

    #[test]
    fn test_get_id() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();
        let mut task = ConcreteTaskBuilder::new()
            .set_description(String::from("Nice"))
            .set_priority(Priority::High)
            .set_name(String::from("Name Nie"))
            .set_task_state(TaskState::Ended)
            .set_term(Date::get_local_date())
            .get_task();
        task.id = 0;
        let _ = conn
            .insert_task(&task)
            .expect("The insertion should be okay");
        let task_db = conn
            .get_task_by_id(0)
            .expect("Should exists a value with this id");
        assert_eq!(task_db, task, "Should be the same task");
        remove_test_files();
    }

    #[test]
    fn test_get_name() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();
        let name = String::from("Teste Name");
        let mut task = ConcreteTaskBuilder::new()
            .set_name(name.clone())
            .set_description(String::from("Nice"))
            .set_priority(Priority::High)
            .set_task_state(TaskState::Ended)
            .set_term(Date::get_local_date())
            .get_task();
        task.id = 0;
        let _ = conn
            .insert_task(&task)
            .expect("The insertion should be okay");
        let task_db = conn
            .get_task_by_name(&name)
            .expect("Should exists a value with this id");

        assert_eq!(task_db, task, "Should be the same task");
        remove_test_files();
    }

    #[test]
    fn test_delete_by_id() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();
        let task_id = 1;

        let mut task = ConcreteTaskBuilder::new().get_task();

        task.id = task_id;

        let _ = conn
            .insert_task(&task)
            .expect("Should be possible to insert");

        let result = conn
            .get_task_by_id(task_id)
            .expect("Should exist a inserted value");

        assert_eq!(result, task, "should be the same tasks");

        let _ = conn
            .delete_task_by_id(task_id)
            .expect("Should be possible to delete a value");

        let result = conn.get_task_by_id(task_id);
        match result {
            Some(_) => assert!(false, "Should not exist any element"),
            None => {}
        }

        remove_test_files();
    }

    #[test]
    fn test_delete_by_name() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();
        let task_name = String::from("Task test");

        let task = ConcreteTaskBuilder::new()
            .set_name(task_name.clone())
            .get_task();

        let _ = conn
            .insert_task(&task)
            .expect("Should be possible to insert");

        let result = conn
            .get_task_by_name(&task_name)
            .expect("Should exist a inserted value");

        assert_eq!(result, task, "should be the same tasks");

        let _ = conn
            .delete_task_by_name(&task_name)
            .expect("Should be possible to delete a value");

        let result = conn.get_task_by_name(&task_name);
        match result {
            Some(_) => assert!(false, "Should not exist any element"),
            None => {}
        }

        remove_test_files();
    }

    #[test]
    fn test_get_tasks() {
        init_folder();
        let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
        conn.init_db();

        let task_1 = ConcreteTaskBuilder::new()
            .set_name(String::from("Task 1"))
            .set_priority(Priority::High)
            .set_term(Date::get_local_date())
            .get_task();

        let task_2 = ConcreteTaskBuilder::new()
            .set_name(String::from("Task 2"))
            .set_priority(Priority::High)
            .set_task_state(TaskState::Ended)
            .set_term(Date::get_local_date())
            .get_task();

        let task_3 = ConcreteTaskBuilder::new()
            .set_name(String::from("Task 3"))
            .set_priority(Priority::Low)
            .set_task_state(TaskState::Pending)
            .set_description(String::from("Big description"))
            .set_term(Date::get_local_date())
            .get_task();

        conn.insert_task(&task_1).unwrap();
        conn.insert_task(&task_2).unwrap();
        conn.insert_task(&task_3).unwrap();

        let mut tasks = vec![task_1, task_2, task_3];
        tasks.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

        let mut db_tasks = conn.get_tasks().unwrap();
        db_tasks.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

        assert_eq!(
            db_tasks.len(),
            tasks.len(),
            "The vectors should have the same size"
        );

        for i in 0..db_tasks.len() {
            assert_eq!(tasks[i], db_tasks[i], "The tasks should be the same");
        }

        remove_test_files();
    }
}
