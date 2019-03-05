use serde_derive::Serialize;

#[derive(Serialize)]
pub struct TemplateContent {
    pub register_action : String,
    pub edit_todo_title : String,
    pub edit_todo_description : String,
    pub edit_todo_id : Option<String>,
    pub submit_button_label : String
}
