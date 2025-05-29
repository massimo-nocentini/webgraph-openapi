// fn main() {
//     println!("Hello, world!");
// }

#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Neighborhood {
    vertex: usize,
    neighborhood: Vec<usize>,
}

#[get("/webgraph-api/neighborhood/<graph>/<vertex>")]
fn neighborhood(graph: &str, vertex: usize) -> Json<Neighborhood> {
    Json(Neighborhood {
        vertex,
        neighborhood: vec![1, 2, 3], // Example data
    })
}

#[get("/webgraph-api/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![neighborhood,hello])
}
