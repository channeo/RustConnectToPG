use actix_web::{

    get,post,
    web::{
        Json, 
        Data,Path,
    },
    Responder,HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
#[derive(Deserialize, Serialize,FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub create_by : i32,
}



#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT id, name, age FROM users")
        .fetch_all(&state.db)
        .await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching users: {}", e)),
    } 
    //"GET/ user".to_string();
}
#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>,path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
   // format!("GET/ user articles for user id: {}", id)
    match sqlx::query_as::<_, Article>("SELECT id, title, content, create_by FROM articles WHERE create_by = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching articles: {}", e)),
    }
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    let id: i32 = path.into_inner();

    let sql = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by"
    )
    .bind(body.title.to_string())
    .bind(body.content.to_string())
    .bind(id)
    .fetch_one(&state.db)
    .await;

    match sql {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}

