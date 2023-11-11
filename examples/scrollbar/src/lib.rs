use canvas_backend::CanvasBackend;
use ratatui::{
    prelude::*,
    symbols::scrollbar,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
};
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsCast};
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

#[wasm_bindgen(start)]
fn start() {
    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let rect = canvas.get_bounding_client_rect();
    rect.width();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    let stdout = io::stdout();
    let backend = CanvasBackend::new(context.clone(), stdout);
    let terminal = Rc::new(RefCell::new(Terminal::new(backend).unwrap()));
    let app = Rc::new(RefCell::new(App::default()));

    let result = run_app(terminal, app);

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
}

fn run_app(
    terminal: Rc<RefCell<Terminal<CanvasBackend<Stdout>>>>,
    app: Rc<RefCell<App>>,
) -> io::Result<()> {
    {
        let mut app = (*app).borrow_mut();
        let mut terminal = (*terminal).borrow_mut();
        terminal.draw(|f| ui(f, &mut app)).unwrap();
    }

    // key events
    {
        let app = Rc::clone(&app);
        let terminal = Rc::clone(&terminal);
        let func = Box::new(move |event: web_sys::KeyboardEvent| {
            let mut app = (*app).borrow_mut();
            let mut terminal = (*terminal).borrow_mut();
            terminal.clear().unwrap();

            match event.key().as_str() {
                "j" => {
                    app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                    app.vertical_scroll_state =
                        app.vertical_scroll_state.position(app.vertical_scroll);
                }
                "k" => {
                    app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                    app.vertical_scroll_state =
                        app.vertical_scroll_state.position(app.vertical_scroll);
                }
                "h" => {
                    app.horizontal_scroll = app.horizontal_scroll.saturating_sub(1);
                    app.horizontal_scroll_state =
                        app.horizontal_scroll_state.position(app.horizontal_scroll);
                }
                "l" => {
                    app.horizontal_scroll = app.horizontal_scroll.saturating_add(1);
                    app.horizontal_scroll_state =
                        app.horizontal_scroll_state.position(app.horizontal_scroll);
                }
                key => console::log_1(&format!("key={key}").into()),
            }
            terminal.draw(|f| ui(f, &mut app)).unwrap();
        });

        let closure = Closure::wrap(func as Box<dyn FnMut(_)>);
        let document = window().unwrap().document().unwrap();
        document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // if let Event::Key(key) = event::read()? {
    //     if key.kind == KeyEventKind::Press {
    //         match key.code {
    //             KeyCode::Char('q') => return Ok(()),
    //             KeyCode::Char('p') => app.show_popup = !app.show_popup,
    //             _ => {}
    //         }
    //     }
    // }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::default().black();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(size);

    let text = vec![
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.clone()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.clone()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
    ];
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(text.len());
    app.horizontal_scroll_state = app.horizontal_scroll_state.content_length(long_line.len());

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .gray()
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let title = Block::default()
        .title("Use h j k l to scroll ◄ ▲ ▼ ►")
        .title_alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block("Vertical scrollbar with arrows"))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[1]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Vertical scrollbar without arrows, without track symbol and mirrored",
        ))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[2]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        chunks[2].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar with only begin arrow & custom thumb symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[3]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("🬋")
            .end_symbol(None),
        chunks[3].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar without arrows & custom thumb and track symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[4]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("░")
            .track_symbol(Some("─")),
        chunks[4].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );
}
