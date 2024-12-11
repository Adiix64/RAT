pub struct AdminManager {
    admins: Vec<i64>,
    secret_token: String,
}

impl AdminManager {
    pub fn new(admins: Vec<i64>) -> Self {
        Self {
            admins,
            secret_token: "your_secret_token".to_string(),
        }
    }

    pub fn is_admin(&self, user_id: i64) -> bool {
        self.admins.contains(&user_id)
    }

    pub fn validate_token_and_add_admin(&mut self, user_id: i64, token: &str) -> bool {
        if token == self.secret_token {
            if !self.is_admin(user_id) {
                self.admins.push(user_id);
            }
            true
        } else {
            false
        }
    }
}
