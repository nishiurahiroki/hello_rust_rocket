use postgres::{Connection, TlsMode};

const CONNECTION_CONFIG : &'static str = "postgres://postgres:password@localhost:5432";

pub fn get_connection() -> Connection {
    Connection::connect(CONNECTION_CONFIG, TlsMode::None).unwrap()
}
