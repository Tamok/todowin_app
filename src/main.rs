// main.rs
// This is the main entry point for the ToDoWin application.
// It initializes the Dioxus application and starts the event loop.

use dioxus::core::component::Component;
use dioxus::core::context::Context;
use dioxus::core::element::Element;
use dioxus::html::div;
use dioxus::runtime::start;

fn main() {
    // Start the Dioxus application with the App component.
    start::<App>();
}

struct App;

impl Component for App {
    fn render(&self, _cx: &mut Context) -> Element {
        // The app's main UI goes here
        // For now, let's just display a simple message
        div().text("Welcome to ToDoWin!").into()
    }
}
