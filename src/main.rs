// main.rs
// This is the main entry point for the ToDoWin application.
// It initializes the Dioxus application and starts the event loop.

use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}

fn main() {
    // TODO: Start your app. You'll need to use a Dioxus renderer for this.
    // The exact code will depend on the renderer you're using.
}