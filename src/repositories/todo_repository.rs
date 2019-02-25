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
            Some(title) => Some(format!(" {} title = %'{}'% ", result.unwrap().to_string(), title)),
            None        => Some(format!(" {} title = %'{}'% ", result.unwrap().to_string(), "".to_string()))
        };

        result = match description {
            Some(description) => Some(format!(" AND {} description = %'{}'% ", result.unwrap().to_string(), description)),
            None              => Some(format!(" AND {} description = %'{}'% ", result.unwrap().to_string(), "".to_string()))
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
