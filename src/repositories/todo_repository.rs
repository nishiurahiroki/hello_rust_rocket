use super::connection::connection::get_connection;

struct Todo {
    title : String,
    description : String
}

pub fn add() {
    let connection = get_connection();
    connection
        .execute(
            "INSERT INTO todo VALUES($1, $2, NULL, NULL)",
            &[&1, &"2"]
        ).unwrap();
}
