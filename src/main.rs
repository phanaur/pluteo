use iced::widget::{Column, Row, checkbox, container, row, text};
use iced::{Element, Length};

struct Book {
    title: String,
    author: String,
    is_read: bool,
}

/// Stores the books in the library.
struct PluteoApp {
    books: Vec<Book>,
}

impl Default for PluteoApp {
    fn default() -> Self {
        let mut books = vec![];
        let book1 = Book {
            title: String::from("Código"),
            author: String::from("Charles Petzold"),
            is_read: false,
        };

        books.push(book1);

        let book2 = Book {
            title: String::from("El Niño con el Pijama de Rayas"),
            author: String::from("John Boyne"),
            is_read: true,
        };
        books.push(book2);

        PluteoApp { books }
    }
}

// No user actions are supported yet.
#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut PluteoApp, message: Message) {
    match message {}
}

fn view(state: &PluteoApp) -> Element<'_, Message> {
    let mut context: Column<Message> = Column::new();
    let header: Row<Message> = row![
        text("Title").width(Length::FillPortion(3)),
        text("Author").width(Length::FillPortion(2)),
        text("Reading Status").width(Length::FillPortion(2)),
    ]
    .width(Length::Fill)
    .spacing(12);
    context = context.push(header);

    let spacing: Row<Message> = row![].width(Length::Fill).height(12);
    context = context.push(spacing);
    for book in &state.books {
        let reading_status: &str = if book.is_read { "Read" } else { "Unread" };
        let row: Row<Message> = row![
            text(book.title.as_str()).width(Length::FillPortion(3)),
            text(book.author.as_str()).width(Length::FillPortion(2)),
            checkbox(book.is_read)
                .label(reading_status)
                .width(Length::FillPortion(2)),
        ]
        .width(Length::Fill)
        .spacing(12);
        context = context.push(row);
    }

    container(context).padding(8).into()
}

fn main() -> iced::Result {
    iced::application(PluteoApp::default, update, view)
        .title("Plúteo")
        .window_size((400.0, 300.0))
        .run()
}
