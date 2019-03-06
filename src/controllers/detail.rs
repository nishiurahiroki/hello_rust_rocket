use rocket_contrib::templates::Template;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::template_contents::detail;

#[get("/detail?<target_todo_id>")]
pub fn initialize(target_todo_id : i32) -> Template {
    let todo = todo_repository::find_by_id(target_todo_id).unwrap();
    Template::render(
        "detail",
        detail::TemplateContent {
            id : todo.id.unwrap(),
            title : todo.title.unwrap(),
            description : todo.description.unwrap()
        }
    )
}
