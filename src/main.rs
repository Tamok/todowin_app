// main.rs
// This is the main entry point for the ToDoWin application.
// It initializes the Dioxus application and starts the event loop.

use dioxus::prelude::*;

fn main() {
    // Start the Dioxus application with the App component.
    dioxus::start::<App>();
}

struct App;

impl Component for App {
    fn render(&self, _cx: &mut Context) -> Element {
        // The app's main UI goes here
    }
}
