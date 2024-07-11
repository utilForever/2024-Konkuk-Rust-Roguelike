mod widget;
use widget::Widget;

fn main() {
    let mut window = widget::Window::new("Rust GUI Demo 1.23");

    window.add_widget(Box::new(widget::Label::new(
        "This is a small text GUI demo.",
    )));
    window.add_widget(Box::new(widget::Button::new("Click me!")));
    window.draw();
}
