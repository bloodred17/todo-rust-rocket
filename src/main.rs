#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

#[get("/todo")]
fn todo<'r>() -> Json<Task<'r>> {
    Json(Task {
        description: "Hello World",
        complete: false,
    })
}

#[post("/todo", data = "<task>")]
fn create_todo(task: Json<Task<'_>>) -> Json<Task<'_>> {
    println!("{:?}", task);
    return task;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 1111));

    rocket::custom(figment)
        .mount("/", routes![index, todo, create_todo])
}