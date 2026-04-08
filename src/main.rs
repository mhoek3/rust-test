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
    pub mod term_kind;
}
use models::term::{
    Term
};
use models::term_kind::{
    TermKind
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

async fn get_term_kinds(
    State(db): State<MySqlPool>,
) -> (StatusCode, Json<Vec<TermKind>>) {
    let mut kinds = Vec::new();

    if SIMULATE {
        for i in 1..=5 {
            let kind = TermKind {
                id: i,
                name: format!("Kind {}", i),
            };
            kinds.push(kind);
        }

        return (StatusCode::OK, Json(kinds));
    }

    kinds = sqlx::query_as::<_, TermKind>(
        "SELECT id, name FROM term_kind"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    (StatusCode::OK, Json(kinds))
}

async fn get_terms(
    State(db): State<MySqlPool>,
) -> (StatusCode, Json<Vec<Term>>) {
    let mut terms = Vec::new();

    if SIMULATE {
        for i in 1..=5 {
            let term = Term {
                id: i,
                term_kind: i % 1,
                name: format!("Demo Name {}", i),
                details: format!("Demo Details {}", i),
            };
            terms.push(term);
        }

        return (StatusCode::OK, Json(terms));
    }

    terms = sqlx::query_as::<_, Term>(
        "SELECT id, term_kind, name, details FROM terms"
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
        "UPDATE terms SET term_kind = ?, name = ?, details = ? WHERE id = ?"
    )
    .bind(&payload.term_kind)
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
        "INSERT INTO terms (term_kind, name, details) VALUES (?, ?, ?)"
    )
    .bind(&payload.term_kind)
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

async fn remove_term(
    State(db): State<MySqlPool>,
    Json(payload): Json<Term>
) -> StatusCode {

    println!("{:?}", payload);

    let result = sqlx::query(
        "DELETE FROM terms WHERE id = ?"
    )
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
        .route("/get_term_kinds", get(get_term_kinds))
        .route("/get_term_form", get(get_term_form))
        .route("/get_terms", get(get_terms))
        .route("/add_term", post(add_term))
        .route("/edit_term", post(edit_term))
        .route("/remove_term", post(remove_term))
        .with_state(db)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}