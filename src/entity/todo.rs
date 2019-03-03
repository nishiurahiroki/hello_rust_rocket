use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct Todo {
    pub id : Option<i32>,
    pub title : Option<String>,
    pub description : Option<String>
}
