use rocket::request::Form;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::template_contents::list;
use hello_rust_rocket::template_contents::add;

#[get("/edit_initialize?<target_todo_id>")]
pub fn initialize(target_todo_id : String) -> Template {
    let todo = todo_repository::find_by_id(target_todo_id.parse::<i32>().unwrap()).unwrap();
    Template::render("add", add::TemplateContent {
        register_action : "./edit_todo".to_string(),
        edit_todo_title : todo.title.unwrap(),
        edit_todo_description : todo.description.unwrap(),
        edit_todo_id : Some(target_todo_id.to_string()),
        submit_button_label : "編集".to_string()
    })
}
