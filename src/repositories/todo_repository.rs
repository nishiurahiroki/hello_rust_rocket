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

pub fn get_todos(title : Option<String>, description : Option<String>) -> Vec<Todo> {
    let query_where = {
        let mut result = Some(" WHERE ".to_string());

        result = match title {
            Some(value) => Some(format!(" {} title LIKE '%{}%' ", result.unwrap().to_string(), value)),
            None        => Some(format!(" {} title LIKE '%{}%' ", result.unwrap().to_string(), "".to_string()))
        };

        result = match description {
            Some(value) => Some(format!(" {} AND description LIKE '%{}%' ", result.unwrap().to_string(), value)),
            None        => Some(format!(" {} AND description LIKE '%{}%' ", result.unwrap().to_string(), "".to_string()))
        };

        result
    };

    let connection = get_connection();
    let todos : Vec<Todo> = connection
                                .query(
                                    &format!(" SELECT id, title, description FROM todo {} ", query_where.unwrap()),
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
