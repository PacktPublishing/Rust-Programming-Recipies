use crate::schema::*;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    password: String,
}

impl User {
    pub fn verify_pass(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
    password: String,
}

pub fn new_user<'a>(name: &'a str, password: &str) -> NewUser<'a> {
    NewUser {
        name,
        password: bcrypt::hash(password, 7).unwrap(),
    }
}

#[derive(Queryable, Debug)]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub options: String,
    pub owner: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "polls"]
pub struct NewPoll<'a> {
    pub question: &'a str,
    pub options: &'a str,
    pub owner: Option<i32>,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "responses"]
pub struct Response {
    pub poll_id: i32,
    pub user_id: i32,
    pub selected: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct FullResponse {
    pub response: Response,
    pub user: User,
    pub poll: Poll,
}
