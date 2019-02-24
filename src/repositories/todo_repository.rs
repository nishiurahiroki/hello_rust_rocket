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

pub fn get_todos() -> Vec<Todo> {
    let connection = get_connection();
    let result : Vec<Todo> = connection
                                .query("SELECT id, title, description FROM todo", &[])
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
    result
}
