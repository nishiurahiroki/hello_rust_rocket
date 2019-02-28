use serde_derive::Serialize;

use super::super::entity::todo::Todo;

#[derive(Serialize)]
pub struct TemplateContent {
    pub todos : Vec<Todo>,
    pub search_title : String,
    pub search_description : String
}
