use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct AddTaskRequest {
    pub title: String,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateTask {
    pub uuid: String,
}
