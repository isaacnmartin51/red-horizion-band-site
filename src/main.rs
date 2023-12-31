use askama::Template;
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
    show: Show
}

struct Show {
    date: &'static str,
    event: &'static str,
}

#[derive(Responder)]
struct IndexResponder<'a> {
    template: IndexTemplate<'a>
}


#[get("/")]
fn index() -> IndexTemplate<'static> {
    IndexTemplate{name: "Isaac", show: Show { date: "5-3-1994", event: "Isaac was born" }}
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
                   .mount("/public", FileServer::from("static/images/"))
}
