use crate::Instruction::user::{User, UserRole};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
}

pub async fn create_user(user_data: web::Json<CreateUserRequest>) -> impl Responder {
    // TODO: Thêm xử lý mã hóa mật khẩu
    let password_hash = user_data.password.clone(); // Tạm thời chưa mã hóa
    
    let new_user = User::new(
        user_data.username.clone(),
        user_data.email.clone(),
        password_hash,
        user_data.full_name.clone(),
    );

    // TODO: Thêm logic lưu vào database
    HttpResponse::Created().json(new_user)
}

pub async fn get_user(user_id: web::Path<i32>) -> impl Responder {
    // TODO: Thêm logic lấy user từ database
    HttpResponse::Ok().json("User details")
}

pub async fn update_user(
    user_id: web::Path<i32>,
    user_data: web::Json<UpdateUserRequest>,
) -> impl Responder {
    // TODO: Thêm logic cập nhật user trong database
    HttpResponse::Ok().json("User updated successfully")
}

pub async fn delete_user(user_id: web::Path<i32>) -> impl Responder {
    // TODO: Thêm logic xóa user từ database
    HttpResponse::Ok().json("User deleted successfully")
}

pub async fn deactivate_user(user_id: web::Path<i32>) -> impl Responder {
    // TODO: Thêm logic vô hiệu hóa user trong database
    HttpResponse::Ok().json("User deactivated successfully")
}

pub async fn activate_user(user_id: web::Path<i32>) -> impl Responder {
    // TODO: Thêm logic kích hoạt user trong database
    HttpResponse::Ok().json("User activated successfully")
}

pub async fn change_user_role(
    user_id: web::Path<i32>,
    new_role: web::Json<UserRole>,
) -> impl Responder {
    // TODO: Thêm logic thay đổi role của user trong database
    HttpResponse::Ok().json("User role changed successfully")
} 