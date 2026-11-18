use iced::widget::{container, text, Column};
use iced::{Element, Font};

struct Book {
    title: String,
    author: String,
    is_read: bool,
}

/// Initial state of Plúteo. Now, it contains a vector of elements book.

struct PluteoApp {
    books: Vec<Book>,
}

impl Default for PluteoApp {
    fn default() -> Self {
        let mut books = vec![];
        let book = Book {
            title: String::from("Código"),
            author: String::from("Charles Petzold"),
            is_read: false,
        };
        
        books.push(book);
        
        PluteoApp {
            books,
        }
    }
}

// There are no user actions for now. Thus, no variants here.
#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut PluteoApp, message: Message) {
    match message {}
}

fn view(state: &PluteoApp) -> Element<'_, Message> {
    let mut context = Column::new();
    context = context.push(text("title"));
    for book in &state.books {
        context = context.push(text(&book.title));
    }
    
    container(context)
    .padding(8).into()
}

fn main() -> iced::Result {
    iced::application(PluteoApp::default, update, view)
        .title("Plúteo")
        .window_size((400.0, 300.0))
        .run()
}
