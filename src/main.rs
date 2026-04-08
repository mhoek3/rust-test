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
    pub mod term;
    pub mod meaning_group;
}
use models::term::{
    Term
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

async fn get_term_form() -> Html<String> {
    // add some caching later ..
    let html_content = fs::read_to_string("static/forms/term.html")
        .await
        .expect("Failed to read HTML file");
    Html(html_content)
}

// async fn get_form() -> Html<&'static str> {
//     Html(r#"
//         <form id="term_form">
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

async fn get_terms(
    State(db): State<MySqlPool>,
) -> (StatusCode, Json<Vec<Term>>) {
    let mut terms = Vec::new();

    if SIMULATE {
        for i in 1..=5 {
            let term = Term {
                id: i,
                group_id: i % 1,
                name: format!("Demo Name {}", i),
                details: format!("Demo Details {}", i),
            };
            terms.push(term);
        }

        return (StatusCode::OK, Json(terms));
    }

    terms = sqlx::query_as::<_, Term>(
        "SELECT id, group_id, name, details FROM terms"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    (StatusCode::OK, Json(terms))
}

async fn edit_term(
    State(db): State<MySqlPool>,
    Json(payload): Json<Term>
) -> StatusCode {

    println!("{:?}", payload);

    let result = sqlx::query(
        "UPDATE terms SET group_id = ?, name = ?, details = ? WHERE id = ?"
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

async fn add_term(
    State(db): State<MySqlPool>,
    Json(payload): Json<Term>,
) -> StatusCode {

    if SIMULATE {
        println!("{:?}", payload);

        return StatusCode::CREATED;
    }

    let result = sqlx::query(
        "INSERT INTO terms (group_id, name, details) VALUES (?, ?, ?)"
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
        .route("/get_term_form", get(get_term_form))
        .route("/get_terms", get(get_terms))
        .route("/add_term", post(add_term))
        .route("/edit_term", post(edit_term))
        .with_state(db)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}