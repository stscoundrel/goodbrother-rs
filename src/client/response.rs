use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PullRequestUser {
    pub login: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PullRequestResponse {
    pub id: i32,
    pub title: String,
    pub user: PullRequestUser,
    pub html_url: String,
    pub repository_url: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResponse {
    pub total_count: i32,
    pub items: Vec<PullRequestResponse>,
}
