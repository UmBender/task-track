mod controller;
mod model;
mod view;

use crate::model::date::Date;
use crate::model::priority::Priority;
use crate::model::task::*;
use crate::model::task_state::TaskState;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = ConcreteTaskRelationalManager::new(DATABASE_PATH);
    conn.init_db();
    controller::exec();
    return Ok(());
}
