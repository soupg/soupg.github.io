#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

//== Use ==//
use std::collections::HashMap;
use std::io;
use rocket::response::NamedFile;


//== Web Pages ==//

// Home
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/home/index.html") 
}

#[get("/style.css")]
fn style() -> io::Result<NamedFile> {
    NamedFile::open("static/home/style.css") 
}


// Test
#[get("/test")]
fn test() -> io::Result<NamedFile> {
    NamedFile::open("static/test/index.html") 
}

#[get("/test/style.css")]
fn teststyle() -> io::Result<NamedFile> {
    NamedFile::open("static/test/style.css") 
}



//== API ==//
#[get("/signup/<username>/<password>")]
fn signup(username: String, password: String) -> String {
    format!("user: {} | pass: {}", username, password)
}




//== Main ==//
fn main() {
    let mut passwords: HashMap<&str, &str> = HashMap::new();

    rocket::ignite().mount("/", routes![index, style, signup, test, teststyle]).launch();
}

