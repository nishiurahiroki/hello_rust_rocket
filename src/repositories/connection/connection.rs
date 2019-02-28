use postgres::{Connection, TlsMode};

const DNS : &'static str = "postgres://postgres:password@localhost:5432";

pub fn get_connection() -> Connection {
    Connection::connect(DNS, TlsMode::None).unwrap()
}
