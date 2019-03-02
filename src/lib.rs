pub mod controllers {
    pub mod add {}
    pub mod list {}
    pub mod detail {}
}

pub mod repositories {
    pub mod connection;
    pub mod todo_repository;
}

pub mod entity {
    pub mod todo;
}

pub mod template_contents {
    pub mod list;
}
