use ratatui::widgets::Table;
use ratatui::widgets::{Block, Cell, Row, TableState};
use color_eyre::eyre::Result;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::widgets::block::{Position};
use ratatui::style::{Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::block::Title;
use crate::action::Action;
use crate::components::Component;
use crate::domain::library::{Library, Rating};
use crate::tui::Frame;

pub struct LibraryTable<'a> {
    table_state: TableState,
    library: &'a Library
}

impl<'a> LibraryTable<'a> {
    pub fn new(library: &'a Library) -> Self {
        Self {
            table_state: TableState::new().with_selected(0),
            library,
        }
    }
}

impl Component<'_> for LibraryTable<'_> {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let number_of_books = self.library.audiobooks.len() as u16;
        let title = Title::from(vec![" Audiobook Library ".bold(), "(".into(), Span::from(number_of_books.to_string()), " items) ".into()]);
        let instructions = Title::from(Line::from(vec![
            " Next ".into(),
            "<Down>".blue().bold(),
            " Previous ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED);

        let max_artist_length = self.library.audiobooks.iter().map(|b| b.artist.len()).max().unwrap_or(5);
        let max_genre_length = self.library.audiobooks.iter().map(|b| b.genre.as_deref().unwrap_or("").len()).max().unwrap_or(5);

        let rows = self.library.audiobooks.iter().map(|book| {
            return Row::new(vec![
                Cell::new(book.title.clone().unwrap_or("<unknown>".to_string())),
                Cell::new(book.artist.clone()),
                Cell::new(book.genre.clone().unwrap_or("<unk>".to_string())),
                Cell::new(Text::from(book.rating.clone().unwrap_or(Rating { rating: 0 }).as_symbol().to_string()).alignment(Alignment::Center)) ])
        });

        let widths = [
            Constraint::Min(10),
            Constraint::Length(u16::try_from(max_artist_length).unwrap_or(5)),
            Constraint::Length(u16::try_from(max_genre_length).unwrap_or(5)),
            Constraint::Length(3)
        ];
        let table =
            Table::new(rows, widths)
                .column_spacing(1)
                .header(Row::new(vec!["Title", "Author", "Genre", "Rtg"]).style(Style::new().bold()).bottom_margin(1))
                .block(Block::new().title("table"))
                .highlight_style(Style::new().reversed())
                .highlight_symbol(">>").block(block);

        f.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }

    fn name(&mut self) -> &str {
        return "library table"
    }
}