extern crate cursive;

use cursive::Cursive;
use cursive::traits::*;
use cursive::view::{Offset, Position};
use cursive::views::{Dialog, OnEventView, TextView};

fn show_popup(siv: &mut Cursive) {
    // Let's center the popup horizontally, but offset it down a few rows
    siv.screen_mut().add_layer_at(
        Position::new(Offset::Center, Offset::Parent(3)),
        Dialog::around(TextView::new("Tak!"))
            .button("Change", |s| {
                // Look for a view tagged "text".
                // We _know_ it's there, so unwrap it.
                s.call_on_id("text", |view: &mut TextView| {
                    let content = reverse(&view.get_content());
                    view.set_content(content);
                });
            })
            .dismiss_button("Ok"),
    );
}

fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

fn main() {
    let mut siv = Cursive::new();

    let content = "Press Q to quit the application.\n\nPress P to open the \
                   popup.";

    siv.add_global_callback('q', |s| s.quit());

    // Let's wrap the view to give it a recognizable ID, so we can look for it.
    // We add the P callback on the textview only (and not globally),
    // so that we can't call it when the popup is already visible.
    siv.add_layer(
        OnEventView::new(TextView::new(content).with_id("text"))
            .on_event('p', |s| show_popup(s)),
    );

    siv.run();
}
