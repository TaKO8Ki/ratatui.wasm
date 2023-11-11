//! This module provides the [`CanvasBackend`] implementation for the [`Backend`] trait.

use std::io::{self, Write};

use wasm_bindgen::JsValue;
use web_sys::{console, window, CanvasRenderingContext2d};

use ratatui::{
    backend::{Backend, ClearType, WindowSize},
    buffer::Cell,
    layout::Size,
    prelude::Rect,
    style::{Color, Modifier},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CanvasBackend<W: Write> {
    context: CanvasRenderingContext2d,
    writer: W,
}

impl<W> CanvasBackend<W>
where
    W: Write,
{
    pub fn new(context: CanvasRenderingContext2d, writer: W) -> CanvasBackend<W> {
        let canvas = context.canvas().unwrap();
        let rect = canvas.get_bounding_client_rect();

        canvas.set_width((rect.width()) as u32);
        canvas.set_height((rect.height()) as u32);

        CanvasBackend { context, writer }
    }
}

impl<W> Write for CanvasBackend<W>
where
    W: Write,
{
    /// Writes a buffer of bytes to the underlying buffer.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    /// Flushes the underlying buffer.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W> Backend for CanvasBackend<W>
where
    W: Write,
{
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut underline_color = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;

        self.context.set_fill_style(&JsValue::from("#263238"));
        let canvas = self.context.canvas().unwrap();
        let rect = canvas.get_bounding_client_rect();
        self.context
            .fill_rect(0.0, 0.0, rect.width(), rect.height());

        for (x, y, cell) in content {
            // self.context.close_path()
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                self.context.move_to(x as f64, y.saturating_add(10) as f64)
            }

            let canvas = self.context.canvas().unwrap();
            let rect = canvas.get_bounding_client_rect();
            let res = window().unwrap().device_pixel_ratio();
            // console::log_1(&format!("symbol={}, x={x}, y={y}, res={res}", cell.symbol).into());

            last_pos = Some((x, y));

            let ch = 16.0;
            let cw = 16.0;
            let lh = 18.0;
            let descent = (lh - ch) / 2.0;

            let x = x as f64 * cw;
            let y = y as f64 * lh + 5.0;
            let yi = (y + lh - descent).floor();

            if cell.modifier != modifier {
                // let diff = ModifierDiff {
                //     from: modifier,
                //     to: cell.modifier,
                // };
                // diff.queue(&mut self.writer)?;
                // modifier = cell.modifier;
            }
            if cell.bg != bg {
                // let color = CColor::from(cell.bg);
                // queue!(self.writer, SetBackgroundColor(color))?;
                // console::log_1(&format!("color={}", cell.bg).into());
                // console::log_1(&format!("init={:?}", self.context.fill_style().as_string()).into());

                self.context
                    .set_fill_style(&JsValue::from(cell.bg.to_string()));
                self.context.fill_rect(x, y, 16.0, 18.0);

                self.context.set_fill_style(&JsValue::from("#000000"));

                // bg = cell.bg;
            }
            // if cell.underline_color != underline_color {
            //     // let color = CColor::from(cell.underline_color);
            //     // queue!(self.writer, SetUnderlineColor(color))?;
            //     // underline_color = cell.underline_color;
            // }

            // queue!(self.writer, Print(&cell.symbol))?;

            // console::log_1(&format!("after symbol={}, x={x}, y={y}", cell.symbol).into());

            // console::log_1(&format!("symbol={}, x={x}, y={y}", cell.symbol).into());
            // let width = rect.width() as u16;
            // let height = rect.height() as u16;

            // if *chars.first().unwrap() == ' ' {
            //     continue;
            // }

            // console::log_1(&format!("font height={}", ch.floor()).into());

            self.context
                .set_font(&format!("{}px monospace", ch.floor()));

            self.context
                .set_fill_style(&JsValue::from(Color::White.to_string()));
            if cell.fg != fg {
                // let color = CColor::from(cell.fg);
                // queue!(self.writer, SetForegroundColor(color))?;
                self.context
                    .set_fill_style(&JsValue::from(cell.fg.to_string()));

                // fg = cell.fg;
            }
            self.context.fill_text(&cell.symbol, x, yi).unwrap();

            if cell.fg != fg {
                self.context.set_fill_style(&JsValue::from("#000000"));
            }
        }

        // #[cfg(feature = "underline-color")]
        // return queue!(
        //     self.writer,
        //     SetForegroundColor(CColor::Reset),
        //     SetBackgroundColor(CColor::Reset),
        //     SetUnderlineColor(CColor::Reset),
        //     SetAttribute(CAttribute::Reset),
        // );
        // #[cfg(not(feature = "underline-color"))]
        // return queue!(
        //     self.writer,
        //     SetForegroundColor(CColor::Reset),
        //     SetBackgroundColor(CColor::Reset),
        //     SetAttribute(CAttribute::Reset),
        // );
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        // execute!(self.writer, Hide)
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        // execute!(self.writer, Show)
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        // crossterm::cursor::position()
        //     .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        Ok((0, 0))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.context.move_to(x as f64, y as f64);
        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.clear_region(ClearType::All)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        // self.writer.flush().unwrap();

        let rect = self.context.canvas().unwrap().get_bounding_client_rect();
        self.context
            .clear_rect(0.0, 0.0, rect.width(), rect.height());

        Ok(())
    }

    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        for _ in 0..n {
            self.context.fill_text("lineeeeeeeeee", 0.0, 0.0).unwrap();
            // self.context.
        }
        self.writer.flush()
    }

    fn size(&self) -> io::Result<Rect> {
        let canvas = self.context.canvas().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let res = window().unwrap().device_pixel_ratio();
        let width = (rect.width() / 16.0) as u16;
        let height = (rect.height() / 18.0) as u16;

        canvas.set_width((rect.width()) as u32);
        canvas.set_height((rect.height()) as u32);

        console::log_1(&format!("width={width}, height={height}").into());

        Ok(Rect::new(0, 0, width, height))
    }

    fn window_size(&mut self) -> Result<WindowSize, io::Error> {
        let canvas = self.context.canvas().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let res = window().unwrap().device_pixel_ratio();
        let width = rect.width();
        let height = rect.height();

        Ok(WindowSize {
            columns_rows: Size {
                width: (30) as u16,
                height: (10) as u16,
            },
            pixels: Size {
                width: (width * res) as u16,
                height: (height * res) as u16,
            },
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

// impl From<Color> for CColor {
//     fn from(color: Color) -> Self {
//         match color {
//             Color::Reset => CColor::Reset,
//             Color::Black => CColor::Black,
//             Color::Red => CColor::DarkRed,
//             Color::Green => CColor::DarkGreen,
//             Color::Yellow => CColor::DarkYellow,
//             Color::Blue => CColor::DarkBlue,
//             Color::Magenta => CColor::DarkMagenta,
//             Color::Cyan => CColor::DarkCyan,
//             Color::Gray => CColor::Grey,
//             Color::DarkGray => CColor::DarkGrey,
//             Color::LightRed => CColor::Red,
//             Color::LightGreen => CColor::Green,
//             Color::LightBlue => CColor::Blue,
//             Color::LightYellow => CColor::Yellow,
//             Color::LightMagenta => CColor::Magenta,
//             Color::LightCyan => CColor::Cyan,
//             Color::White => CColor::White,
//             Color::Indexed(i) => CColor::AnsiValue(i),
//             Color::Rgb(r, g, b) => CColor::Rgb { r, g, b },
//         }
//     }
// }

/// The `ModifierDiff` struct is used to calculate the difference between two `Modifier`
/// values. This is useful when updating the terminal display, as it allows for more
/// efficient updates by only sending the necessary changes.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct ModifierDiff {
    pub from: Modifier,
    pub to: Modifier,
}

impl ModifierDiff {
    fn queue<W>(&self, mut w: W) -> io::Result<()>
    where
        W: io::Write,
    {
        //use crossterm::Attribute;
        // let removed = self.from - self.to;
        // if removed.contains(Modifier::REVERSED) {
        //     queue!(w, SetAttribute(CAttribute::NoReverse))?;
        // }
        // if removed.contains(Modifier::BOLD) {
        //     queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
        //     if self.to.contains(Modifier::DIM) {
        //         queue!(w, SetAttribute(CAttribute::Dim))?;
        //     }
        // }
        // if removed.contains(Modifier::ITALIC) {
        //     queue!(w, SetAttribute(CAttribute::NoItalic))?;
        // }
        // if removed.contains(Modifier::UNDERLINED) {
        //     queue!(w, SetAttribute(CAttribute::NoUnderline))?;
        // }
        // if removed.contains(Modifier::DIM) {
        //     queue!(w, SetAttribute(CAttribute::NormalIntensity))?;
        // }
        // if removed.contains(Modifier::CROSSED_OUT) {
        //     queue!(w, SetAttribute(CAttribute::NotCrossedOut))?;
        // }
        // if removed.contains(Modifier::SLOW_BLINK) || removed.contains(Modifier::RAPID_BLINK) {
        //     queue!(w, SetAttribute(CAttribute::NoBlink))?;
        // }

        // let added = self.to - self.from;
        // if added.contains(Modifier::REVERSED) {
        //     queue!(w, SetAttribute(CAttribute::Reverse))?;
        // }
        // if added.contains(Modifier::BOLD) {
        //     queue!(w, SetAttribute(CAttribute::Bold))?;
        // }
        // if added.contains(Modifier::ITALIC) {
        //     queue!(w, SetAttribute(CAttribute::Italic))?;
        // }
        // if added.contains(Modifier::UNDERLINED) {
        //     queue!(w, SetAttribute(CAttribute::Underlined))?;
        // }
        // if added.contains(Modifier::DIM) {
        //     queue!(w, SetAttribute(CAttribute::Dim))?;
        // }
        // if added.contains(Modifier::CROSSED_OUT) {
        //     queue!(w, SetAttribute(CAttribute::CrossedOut))?;
        // }
        // if added.contains(Modifier::SLOW_BLINK) {
        //     queue!(w, SetAttribute(CAttribute::SlowBlink))?;
        // }
        // if added.contains(Modifier::RAPID_BLINK) {
        //     queue!(w, SetAttribute(CAttribute::RapidBlink))?;
        // }

        Ok(())
    }
}
