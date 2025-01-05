mod controller;
mod model;
mod view;

use crate::model::date::Date;
use crate::model::priority::Priority;
use crate::model::task::*;
use crate::model::task_state::TaskState;
use chrono::NaiveDate;
use clap::{Arg, Command};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    let task = ConcreteTaskBuilder::new()
        .set_name(String::from("Nova task legappl"))
        .set_description(String::from(
            "Uma task diferente para poder fazer a query ser legalAHHHHHHHh",
        ))
        .get_task();
    let writer = ConcreteTaskRelationalManager::new(DATABASE_PATH_TEST);
    writer.init_db();
    let _ = writer.insert_task(&task);
    let atividade = writer.get_task_by_name(&String::from("Task kbsurda"));

    match atividade {
        Some(i) => println!("{:?}", i),
        None => println!("Não achou"),
    }

    let atividades = writer.get_tasks().unwrap();
    for i in atividades.iter() {
        println!("{:?}", i);
    }

    let atividade = writer.get_task_by_id(2678940846310413546).unwrap();
    println!("{:?}", atividade);

    let result = writer.delete_task_by_id(2678940846310413546);
    match result {
        Ok(()) => println!("It was deleted"),
        Err(_) => println!("It wan not able to delete it"),
    }

    return Ok(());
}
