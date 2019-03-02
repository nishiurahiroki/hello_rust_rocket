use rocket::request::Form;
use rocket_contrib::templates::Template;

use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::template_contents::list::TemplateContent;

#[derive(FromForm)]
pub struct ListFromForm {
    pub title : String,
    pub description : String,
    pub target_todo_id : String
}

#[get("/list")]
pub fn initialize() -> Template {
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            "".to_string(),
            "".to_string()
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}

#[get("/search?<title>&<description>")]
pub fn search(title : String, description : String) -> Template {
    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            title.clone(),
            description.clone()
        ),
        search_title : title,
        search_description : description
    })
}

#[post("/delete", data = "<listFromForm>")]
pub fn delete(listFromForm : Form<ListFromForm>) -> Template {
    let listForm : ListFromForm = listFromForm.into_inner();

    todo_repository::delete(listForm.target_todo_id);

    Template::render("list", TemplateContent {
        todos : todo_repository::get_todos(
            listForm.title.to_string(),
            listForm.description.to_string()
        ),
        search_title : listForm.title.to_string(),
        search_description : listForm.description.to_string()
    })
}
