use axum::{
    response::Html,
    routing::{get, post}, 
    http::StatusCode,
    Json, Router, 
    extract::State
};
use tower_http::services::ServeDir;

//use std::sync::Arc;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

mod models {
    pub mod meaning;
    pub mod meaning_group;
}
use models::meaning::{
    Meaning
};
use models::meaning_group::{
    MeaningGroup
};

const SIMULATE : bool = false;

use tokio::fs;
async fn index() -> Html<String> {
    // add some caching later ..
    let html_content = fs::read_to_string("static/index.html")
        .await
        .expect("Failed to read HTML file");
    Html(html_content)
}

async fn get_form() -> Html<String> {
    // add some caching later ..
    let html_content = fs::read_to_string("static/form.html")
        .await
        .expect("Failed to read HTML file");
    Html(html_content)
}

// async fn get_form() -> Html<&'static str> {
//     Html(r#"
//         <form id="meaning_form">
//             <label for="name">Name:</label>
//             <input type="text" id="name" name="name"><br>
//             <label for="details">Details:</label>
//             <textarea id="details" name="details" cols="10"></textarea><br>
//             <button type="submit">Submit</button>
//         </form>
//     "#)
// }

async fn get_meaning_groups(
    State(db): State<MySqlPool>,
) -> (StatusCode, Json<Vec<MeaningGroup>>) {
    let mut groups = Vec::new();

    if SIMULATE {
        for i in 1..=5 {
            let group = MeaningGroup {
                id: i,
                name: format!("Group {}", i),
            };
            groups.push(group);
        }

        return (StatusCode::OK, Json(groups));
    }

    groups = sqlx::query_as::<_, MeaningGroup>(
        "SELECT id, name FROM meaning_group"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    (StatusCode::OK, Json(groups))
}

async fn get_meanings(
    State(db): State<MySqlPool>,
) -> (StatusCode, Json<Vec<Meaning>>) {
    let mut meanings = Vec::new();

    if SIMULATE {
        for i in 1..=5 {
            let meaning = Meaning {
                id: i,
                group_id: i % 1,
                name: format!("Demo Name {}", i),
                details: format!("Demo Details {}", i),
            };
            meanings.push(meaning);
        }

        return (StatusCode::OK, Json(meanings));
    }

    meanings = sqlx::query_as::<_, Meaning>(
        "SELECT id, group_id, name, details FROM meanings"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    (StatusCode::OK, Json(meanings))
}

async fn edit_meaning(
    State(db): State<MySqlPool>,
    Json(payload): Json<Meaning>
) -> StatusCode {

    println!("{:?}", payload);

    let result = sqlx::query(
        "UPDATE meanings SET group_id = ?, name = ?, details = ? WHERE id = ?"
    )
    .bind(&payload.group_id)
    .bind(&payload.name)
    .bind(&payload.details)
    .bind(&payload.id)  // Use payload.id for the WHERE clause
    .execute(&db)
    .await;

    match result {
        Ok(_) => StatusCode::OK,  // For updates, usually OK
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn add_meaning(
    State(db): State<MySqlPool>,
    Json(payload): Json<Meaning>,
) -> StatusCode {

    if SIMULATE {
        println!("{:?}", payload);

        return StatusCode::CREATED;
    }

    let result = sqlx::query(
        "INSERT INTO meanings (group_id, name, details) VALUES (?, ?, ?)"
    )
    .bind(&payload.group_id)
    .bind(&payload.name)
    .bind(&payload.details)
    .execute(&db)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn init_db() -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://rs_user:rs_pass@127.0.0.1:3306/rs_db")
        .await
        .expect("Failed to connect to DB")
}

#[tokio::main]
async fn main() {
    let db = init_db().await;

    let app = Router::new()
        .route("/", get(index))
        .route("/get_meaning_groups", get(get_meaning_groups))
        .route("/get_meanings", get(get_meanings))
        .route("/get_form", get(get_form))
        .route("/add_meaning", post(add_meaning))
        .route("/edit_meaning", post(edit_meaning))
        .with_state(db)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}