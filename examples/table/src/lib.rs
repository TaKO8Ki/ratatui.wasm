use canvas_backend::CanvasBackend;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell as TuiCell, Row, Table, TableState},
};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
};
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsCast};
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

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
    let app = Rc::new(RefCell::new(App::new()));

    let res = run_app(terminal, app);

    if let Err(err) = res {
        console::log_1(&format!("{err:?}").into());
    }
}

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl App {
    fn new() -> App {
        App {
            state: TableState::default(),
            items: vec![
                vec![
                    "Row11".to_string(),
                    "Row12".to_string(),
                    "Row13".to_string(),
                ],
                vec![
                    "Row21".to_string(),
                    "Row22".to_string(),
                    "Row23".to_string(),
                ],
                vec![
                    "Row31".to_string(),
                    "Row32".to_string(),
                    "Row33".to_string(),
                ],
                vec![
                    "Row41".to_string(),
                    "Row42".to_string(),
                    "Row43".to_string(),
                ],
                vec![
                    "Row51".to_string(),
                    "Row52".to_string(),
                    "Row53".to_string(),
                ],
                vec![
                    "Row61".to_string(),
                    "Row62\nTest".to_string(),
                    "Row63".to_string(),
                ],
                vec![
                    "Row71".to_string(),
                    "Row72".to_string(),
                    "Row73".to_string(),
                ],
                vec![
                    "Row81".to_string(),
                    "Row82".to_string(),
                    "Row83".to_string(),
                ],
                vec![
                    "Row91".to_string(),
                    "Row92".to_string(),
                    "Row93".to_string(),
                ],
                vec![
                    "Row101".to_string(),
                    "Row102".to_string(),
                    "Row103".to_string(),
                ],
                vec![
                    "Row111".to_string(),
                    "Row112".to_string(),
                    "Row113".to_string(),
                ],
                vec![
                    "Row121".to_string(),
                    "Row122".to_string(),
                    "Row123".to_string(),
                ],
                vec![
                    "Row131".to_string(),
                    "Row132".to_string(),
                    "Row133".to_string(),
                ],
                vec![
                    "Row141".to_string(),
                    "Row142".to_string(),
                    "Row143".to_string(),
                ],
                vec![
                    "Row151".to_string(),
                    "Row152".to_string(),
                    "Row153".to_string(),
                ],
                vec![
                    "Row161".to_string(),
                    "Row162".to_string(),
                    "Row163".to_string(),
                ],
                vec![
                    "Row171".to_string(),
                    "Row172".to_string(),
                    "Row173".to_string(),
                ],
                vec![
                    "Row181".to_string(),
                    "Row182".to_string(),
                    "Row183".to_string(),
                ],
                vec![
                    "Row191".to_string(),
                    "Row192".to_string(),
                    "Row193".to_string(),
                ],
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
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
                "ArrowDown" => app.next(),
                "ArrowUp" => app.previous(),
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

    // resize events
    {
        let func = Box::new(move |_: web_sys::Event| {
            console::log_1(&"fhoweahfoahweofhaowehfoahefoawe".to_string().into());
            let mut app = (*app).borrow_mut();
            let mut terminal = (*terminal).borrow_mut();
            terminal.clear().unwrap();
            terminal.autoresize().unwrap();
            terminal.draw(|f| ui(f, &mut app)).unwrap();
        });

        let closure = Closure::wrap(func as Box<dyn FnMut(_)>);
        window()
            .unwrap()
            .set_onresize(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Header1", "Header2", "Header3"]
        .iter()
        .map(|h| TuiCell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| TuiCell::from(c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Max(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
