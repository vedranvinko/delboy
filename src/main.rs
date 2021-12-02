use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

fn main() {
    let mut root = cursive::default();

    root.add_global_callback(Key::Esc, |s| s.quit());

    root.run();
}
