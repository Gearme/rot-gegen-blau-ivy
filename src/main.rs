use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/ping", get(ping))
        .route("/move", get(execute_move))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    ""
}

async fn ping() -> &'static str {
    "pong"
}

fn string_move(field: &str) {}

async fn execute_move(state: Query<InputField>) -> (StatusCode, Json<FieldState>) {
    let len = state.field.chars().filter(|c| *c == 'o').count();
    let first_free_field_idx = state.field.find('o').expect("no more free fields!");
    let mut new_field = state.field.clone();
    let my_colour = if first_free_field_idx == 0 {
        'B'
    } else {
        let lastchar = state.field.chars().nth(first_free_field_idx - 1).unwrap();
        match state.field.chars().nth(first_free_field_idx - 1).unwrap() {
            'R' => 'B',
            'B' => 'R',
            _ => 'B',
        }
    };
    let _rest = new_field.split_off(first_free_field_idx);
    let my_move = match len.rem_euclid(3) {
        1 => 1,
        2 => 2,
        _ => 1,
    };
    for i in 0..my_move {
        new_field.push(my_colour);
    }
    for i in 0..len - my_move {
        new_field.push('o');
    }

    (
        StatusCode::OK,
        Json(FieldState {
            field: new_field,
            state: "".to_string(),
        }),
    )
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize)]
struct InputField {
    field: String,
}

#[derive(Serialize)]
struct FieldState {
    field: String,
    state: String,
}
