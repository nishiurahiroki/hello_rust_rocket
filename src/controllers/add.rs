extern crate multipart;

use std::io::{self, Cursor, Write, Read};
use std::fs::File;
use std::path::PathBuf;

use multipart::mock::StdoutTee;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::save::SavedData::*;

use rocket::Data;
use rocket::request::Form;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use hello_rust_rocket::repositories::todo_repository;
use hello_rust_rocket::entity::todo::Todo;
use hello_rust_rocket::template_contents::list;
use hello_rust_rocket::template_contents::add;

#[derive(FromForm)]
pub struct TodoFromForm {
    pub title : String,
    pub description : String,
    pub edit_todo_id : Option<String>
}

#[get("/add")]
pub fn initialize() -> Template {
    Template::render("add", add::TemplateContent{
        page_title : "新規登録".to_string(),
        register_action : "./add_todo".to_string(),
        edit_todo_title : "".to_string(),
        edit_todo_description : "".to_string(),
        edit_todo_id : None,
        submit_button_label : "新規登録".to_string()
    })
}

#[post("/add_todo", data = "<todo_form_data>")]
pub fn add_todo(cont_type : &ContentType , todo_form_data : Data) -> Template {
    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").unwrap();

    let entries = parse_multipart_form_data(boundary, todo_form_data);
    let fields = entries.unwrap().fields;
    let dummy_path_buf = PathBuf::new();
    let (title, description, image) = {
        (
            match &fields["title"][0].data {
                Text(value) => value.to_string(),
                Bytes(_) => "".to_string(),
                File(_, _) => "".to_string()
            },
            match &fields["description"][0].data {
                Text(value) => value.to_string(),
                Bytes(_) => "".to_string(),
                File(_, _) => "".to_string()
            },
            match &fields["image"][0].data {
                Text(_)  => (&dummy_path_buf, &0x0123456789ABCDEFu64),
                Bytes(_) => (&dummy_path_buf, &0x0123456789ABCDEFu64),
                File(file_path, size) => (file_path, size)
            })
    };

    let (temp_file_path, _) = image;
    let mut file = File::open(temp_file_path.to_str().unwrap()).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).unwrap();

    todo_repository::add(
        Todo {
            id : Some(-1),
            title : Some(title),
            description : Some(description),
            image : Some(buffer)
        }
    );

    Template::render("list", list::TemplateContent {
        todos : todo_repository::get_todos(
            "".to_string(),
            "".to_string()
        ),
        search_title : "".to_string(),
        search_description : "".to_string()
    })
}

fn parse_multipart_form_data(boundary : &str, data : Data) -> Option<Entries> {
    let mut out = Vec::new();

    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => Some(entries),
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed : {:?}", reason).unwrap();
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field : {:?}", field.source.headers).unwrap();
            }

            Some(partial.entries)
        }
        Error(e) => None
    }
}
