use iced::widget::{container, text};
use iced::{Element, Font};

/// Estado inicial de Pluteo, todavía sin datos de biblioteca.
#[derive(Default)]
struct PluteoApp;

// No hay acciones de usuario en este prototipo, por eso no hay variantes.
#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut PluteoApp, message: Message) {
    match message {}
}

fn view(_state: &PluteoApp) -> Element<'_, Message> {
    container(text("Pluteo").size(20).font(Font {
        weight: iced::font::Weight::Bold,
        ..Font::DEFAULT
    }))
    .padding(8)
    .into()
}

fn main() -> iced::Result {
    iced::application(PluteoApp::default, update, view)
        .title("Pluteo")
        .window_size((400.0, 300.0))
        .run()
}
