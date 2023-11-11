use canvas_backend::CanvasBackend;
use ratatui::{
    prelude::*,
    widgets::calendar::{CalendarEventStore, DateStyler, Monthly},
};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
};
use time::{Date, Month, OffsetDateTime};
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

    let stdout = io::stdout();
    let backend = CanvasBackend::new(context.clone(), stdout);
    let terminal = Rc::new(RefCell::new(Terminal::new(backend).unwrap()));

    let res = run_app(terminal);

    if let Err(err) = res {
        console::log_1(&format!("{err:?}").into());
    }
}

fn run_app(terminal: Rc<RefCell<Terminal<CanvasBackend<Stdout>>>>) -> io::Result<()> {
    {
        let mut terminal = (*terminal).borrow_mut();
        terminal.draw(draw).unwrap();
    }

    // resize events
    {
        let func = Box::new(move |_: web_sys::Event| {
            let mut terminal = (*terminal).borrow_mut();
            terminal.clear().unwrap();
            terminal.autoresize().unwrap();
            terminal.draw(draw).unwrap();
        });

        let closure = Closure::wrap(func as Box<dyn FnMut(_)>);
        window()
            .unwrap()
            .set_onresize(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}

fn draw(f: &mut Frame) {
    let app_area = f.size();

    let calarea = Rect {
        x: app_area.x + 1,
        y: app_area.y + 1,
        height: app_area.height - 1,
        width: app_area.width - 1,
    };

    let mut start = OffsetDateTime::now_local()
        .unwrap()
        .date()
        .replace_month(Month::January)
        .unwrap()
        .replace_day(1)
        .unwrap();

    let list = make_dates(start.year());

    for chunk in split_rows(&calarea)
        .iter()
        .flat_map(|row| split_cols(row).to_vec())
    {
        let cal = cals::get_cal(start.month(), start.year(), &list);
        f.render_widget(cal, chunk);
        start = start.replace_month(start.month().next()).unwrap();
    }
}

fn split_rows(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

    list_layout.split(*area)
}

fn split_cols(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

    list_layout.split(*area)
}

fn make_dates(current_year: i32) -> CalendarEventStore {
    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );

    // Holidays
    let holiday_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::UNDERLINED);

    // new year's
    list.add(
        Date::from_calendar_date(current_year, Month::January, 1).unwrap(),
        holiday_style,
    );
    // next new_year's for December "show surrounding"
    list.add(
        Date::from_calendar_date(current_year + 1, Month::January, 1).unwrap(),
        holiday_style,
    );
    // groundhog day
    list.add(
        Date::from_calendar_date(current_year, Month::February, 2).unwrap(),
        holiday_style,
    );
    // april fool's
    list.add(
        Date::from_calendar_date(current_year, Month::April, 1).unwrap(),
        holiday_style,
    );
    // earth day
    list.add(
        Date::from_calendar_date(current_year, Month::April, 22).unwrap(),
        holiday_style,
    );
    // star wars day
    list.add(
        Date::from_calendar_date(current_year, Month::May, 4).unwrap(),
        holiday_style,
    );
    // festivus
    list.add(
        Date::from_calendar_date(current_year, Month::December, 23).unwrap(),
        holiday_style,
    );
    // new year's eve
    list.add(
        Date::from_calendar_date(current_year, Month::December, 31).unwrap(),
        holiday_style,
    );

    // seasons
    let season_style = Style::default()
        .fg(Color::White)
        .bg(Color::Yellow)
        .add_modifier(Modifier::UNDERLINED);
    // spring equinox
    list.add(
        Date::from_calendar_date(current_year, Month::March, 22).unwrap(),
        season_style,
    );
    // summer solstice
    list.add(
        Date::from_calendar_date(current_year, Month::June, 21).unwrap(),
        season_style,
    );
    // fall equinox
    list.add(
        Date::from_calendar_date(current_year, Month::September, 22).unwrap(),
        season_style,
    );
    list.add(
        Date::from_calendar_date(current_year, Month::December, 21).unwrap(),
        season_style,
    );
    list
}

mod cals {
    use super::*;

    pub(super) fn get_cal<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        use Month::*;
        match m {
            May => example1(m, y, es),
            June => example2(m, y, es),
            July => example3(m, y, es),
            December => example3(m, y, es),
            February => example4(m, y, es),
            November => example5(m, y, es),
            _ => default(m, y, es),
        }
    }

    fn default<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(Style::default())
            .default_style(default_style)
    }

    fn example1<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(default_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example2<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::DIM)
            .fg(Color::LightYellow);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example3<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(Style::default().add_modifier(Modifier::DIM))
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn example4<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
    }

    fn example5<'a, S: DateStyler>(m: Month, y: i32, es: S) -> Monthly<'a, S> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(header_style)
            .default_style(default_style)
    }
}
