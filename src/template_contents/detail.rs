use serde_derive::Serialize;

#[derive(Serialize)]
pub struct TemplateContent {
    pub id : i32,
    pub title : String,
    pub description : String
}
