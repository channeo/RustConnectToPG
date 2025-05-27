use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password_hash: String,
        full_name: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Sẽ được set bởi database
            username,
            email,
            password_hash,
            full_name,
            created_at: now,
            updated_at: now,
            is_active: true,
            role: UserRole::User,
        }
    }

    pub fn update(&mut self, full_name: Option<String>, email: Option<String>) {
        if let Some(name) = full_name {
            self.full_name = name;
        }
        if let Some(new_email) = email {
            self.email = new_email;
        }
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn change_role(&mut self, new_role: UserRole) {
        self.role = new_role;
        self.updated_at = Utc::now();
    }
}
