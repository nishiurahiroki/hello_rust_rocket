use super::connection::connection::get_connection;
use super::super::entity::todo::Todo;

pub fn add(todo : Todo) {
    let connection = get_connection();
    connection
        .execute(
            "INSERT INTO todo(title, description) VALUES($1, $2)",
            &[&todo.title.unwrap(), &todo.description.unwrap()]
        ).unwrap();
}

pub fn get_todos(title : String, description : String) -> Vec<Todo> {
    let connection = get_connection();
    let todos : Vec<Todo> = connection
                                .query(
                                    &format!("SELECT id, title, description FROM todo WHERE title LIKE '%{}%' AND description LIKE '%{}%' ", title.to_string(), description.to_string()),
                                    &[]
                                )
                                .unwrap()
                                .iter()
                                .map(|row| {
                                    Todo {
                                        id : row.get("id"),
                                        title : row.get("title"),
                                        description : row.get("description")
                                    }
                                })
                                .collect();
    todos
}

pub fn delete(todo_id : String) {
    let connection = get_connection();
    connection.execute(
        "DELETE FROM todo WHERE id = $1",
        &[&todo_id.parse::<i32>().unwrap()]
    );
}
