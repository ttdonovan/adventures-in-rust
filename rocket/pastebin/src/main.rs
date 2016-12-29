#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;

mod paste_id;

use std::io;
use std::fs::File;
use std::path::Path;

use rocket::Data;
use rocket::response::content;

use paste_id::PasteID;

const HOST: &'static str = "http://localhost:8000";
const ID_LENGTH: usize = 5;

#[post("/", data="<paste>")]
fn upload(paste: Data) -> io::Result<content::Plain<String>> {
  let id = PasteID::new(ID_LENGTH);
  let filename = format!("upload/{id}", id = id);
  let url = format!("{host}/{id}\n", host = HOST, id = id);

  // Write the paste out to the file and return the URL.
  paste.stream_to_file(Path::new(&filename))?;
  Ok(content::Plain(url))
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<content::Plain<File>> {
  let filename = format!("upload/{id}", id = id);
  File::open(&filename).map(|f| content::Plain(f)).ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve]).launch();
}
