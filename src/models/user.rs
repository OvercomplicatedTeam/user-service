use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub login: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}
