use super::connection::connection::get_connection;

struct Todo {
    title : String,
    description : String
}

pub fn add() {
    let connection = get_connection();
}
