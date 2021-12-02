use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView};
use cursive::Cursive;

fn show_info(root: &mut Cursive, name: &str) {
    root.pop_layer();

    let text = format!("User {}", name);

    root.add_layer(Dialog::around(TextView::new(text)).button("Finish", |s| s.quit()));
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
