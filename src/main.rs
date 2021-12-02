use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use cursive_async_view::{AsyncState, AsyncView};

use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    login: Option<String>,
    id: Option<u32>,
    node_id: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    html_url: Option<String>,
    followers_url: Option<String>,
    following_url: Option<String>,
    gists_url: Option<String>,
    starred_url: Option<String>,
    subscriptions_url: Option<String>,
    organizations_url: Option<String>,
    repos_url: Option<String>,
    events_url: Option<String>,
    received_events_url: Option<String>,
    type_: Option<String>,
    site_admin: Option<bool>,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    email: Option<String>,
    hireable: Option<bool>,
    bio: Option<String>,
    twitter_username: Option<String>,
    public_repos: Option<u32>,
    public_gists: Option<u32>,
    followers: Option<u32>,
    following: Option<u32>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[tokio::main]
async fn get_user_info(name: &str) -> Result<User, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}", name);
    let rsp = reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, "vedranvinko")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;
    let body = rsp.text().await?;
    let user: User = serde_json::from_str(&body).unwrap();

    Ok(user)
}

fn show_info(root: &mut Cursive, name: &str) {
    root.pop_layer();

    let text = format!("User {}", name);

    root.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Name: "))
                .child(TextView::new("Public repos: ")),
        )
        .button("OK", |s| s.quit()),
    );
}

fn main() {
    let mut root = cursive::default();

    root.add_layer(
        Dialog::new()
            .title("Enter Github user")
            .padding_lrtb(1, 1, 1, 0)
            .content(EditView::new().on_submit(show_info).with_name("name")),
    );

    root.add_global_callback(Key::Esc, |s| s.quit());

    root.run();
}
