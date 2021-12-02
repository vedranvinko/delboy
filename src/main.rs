use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use cursive_async_view::{AsyncState, AsyncView};

use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    login: String,
    public_repos: String,
    avatar_url: String,
}

#[tokio::main]
async fn get_user_info(name: &str) -> Result<User, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}", name);
    let rsp = reqwest::get(url).await?;
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
