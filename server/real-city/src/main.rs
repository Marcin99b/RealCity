use rocket::serde::{Serialize};

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RoadsDTO {
    roads: Vec<Road>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Road {
    nodes: Vec<Node>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Node {
    lat: f64,
    lon: f64,
}

#[get("/roads", format = "application/json")]
fn roads() -> String {
    let roads = vec![
        Road {
            nodes: vec![
                Node { lat: 0.0, lon: 0.0 },
                Node { lat: 1.0, lon: 1.0 },
            ]
        },
        Road {
            nodes: vec![
                Node { lat: 2.0, lon: 2.0 },
                Node { lat: 3.0, lon: 3.0 },
            ]
        },
    ];
    return serde_json::to_string(&roads).unwrap();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index, 
        roads
        ])
}