use rocket::request::Form;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::template_contents::list;
use hello_rust_rocket::template_contents::add;
use hello_rust_rocket::template_contents::detail;

#[derive(FromForm)]
pub struct EditTodoForm {
    pub title : String,
    pub description : String,
    pub edit_todo_id : i32
}

#[get("/edit_initialize?<target_todo_id>")]
pub fn initialize(target_todo_id : String) -> Template {
    let todo = todo_repository::find_by_id(target_todo_id.parse::<i32>().unwrap()).unwrap();
    Template::render("add", add::TemplateContent {
        page_title : "編集画面".to_string(),
        register_action : "./edit_todo".to_string(),
        edit_todo_title : todo.title.unwrap(),
        edit_todo_description : todo.description.unwrap(),
        edit_todo_id : Some(target_todo_id.to_string()),
        submit_button_label : "編集".to_string()
    })
}

#[post("/edit_todo", data = "<editTodoForm>")]
pub fn edit_todo(editTodoForm : Form<EditTodoForm>) -> Template {
    let editTodoForm = editTodoForm.into_inner();
    todo_repository::update(
        Todo {
            id : Some(editTodoForm.edit_todo_id),
            title : Some(editTodoForm.title.clone()),
            description : Some(editTodoForm.description.clone()),
            image : Some(Vec::new())
        }
    );
    Template::render(
        "detail",
        detail::TemplateContent {
            id : editTodoForm.edit_todo_id,
            title : editTodoForm.title,
            description : editTodoForm.description,
            image : Some(Vec::new())
        }
    )
}
