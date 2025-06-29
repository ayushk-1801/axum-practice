use axum::{debug_handler, extract::Query, Json};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    name: String,
    age: u8,
    gender: String,
}

#[debug_handler]
pub async fn root() -> Json<Person> {
    println!("[GET] /");
    Json::from(Person {
        name: String::from("Ayush Kumar Anand"),
        age: 19,
        gender: String::from("Male"),
    })
}

pub async fn root_post(Json(p): Json<Person>) -> Json<Person> {
    println!("[POST] /");
    Json::from(p)
}

pub async fn root_delete(Query(p): Query<Person>) -> Json<Person> {
    println!("[POST] /");
    Json::from(p)
}
