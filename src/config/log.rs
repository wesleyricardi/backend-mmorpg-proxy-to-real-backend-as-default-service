use std::fmt;

use crate::error::AppError;
use chrono::Local;
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    White,
    _Custom(String),
}

impl Color {
    fn to_rgb(&self) -> (i32, i32, i32) {
        match self {
            Color::Red => (255, 0, 0),
            Color::Blue => (0, 0, 255),
            Color::Green => (0, 128, 0),
            Color::Yellow => (255, 255, 0),
            Color::White => (255, 255, 255),
            Color::_Custom(hex) => parse_hex_rgb(hex).unwrap_or_else(|| {
                log::warn!("invalid custom log color '{}', falling back to white", hex);
                (255, 255, 255)
            }),
        }
    }
}

fn parse_hex_rgb(hex: &str) -> Option<(i32, i32, i32)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = i32::from_str_radix(&hex[0..2], 16).ok()?;
    let g = i32::from_str_radix(&hex[2..4], 16).ok()?;
    let b = i32::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn format_styled(text: impl fmt::Display, color: Color, bold: bool) -> String {
    let (r, g, b) = color.to_rgb();

    if bold {
        format!("\x1B[1m\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
    } else {
        format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
    }
}

pub fn setup_logger() -> Result<(), AppError> {
    fern::Dispatch::new()
        .chain(get_terminal_config())
        .chain(get_log_file_config()?)
        .apply()
        .map_err(|error| crate::unexpected_error!(error, "failed to apply logger configuration"))?;
    Ok(())
}

fn get_terminal_config() -> fern::Dispatch {
    fern::Dispatch::new()
        .format(|out, message, record| match record.level() {
            log::Level::Error => out.finish(format_args!(
                "{}: {}\t {}\n",
                format_styled(record.level(), Color::Red, true),
                format_styled(record.target(), Color::White, true),
                message,
            )),
            log::Level::Warn => out.finish(format_args!(
                "{}: {}\t {}\n",
                format_styled(record.level(), Color::Yellow, true),
                format_styled(record.target(), Color::White, true),
                message,
            )),
            log::Level::Info => out.finish(format_args!(
                "{}: {}\t {}\n",
                format_styled(record.level(), Color::Green, true),
                format_styled(record.target(), Color::White, true),
                message,
            )),
            log::Level::Debug => out.finish(format_args!(
                "{}: {}\t {}\n",
                format_styled(record.level(), Color::Blue, true),
                format_styled(record.target(), Color::White, true),
                message,
            )),
            log::Level::Trace => out.finish(format_args!(
                "{}: {}\t {}\n",
                format_styled(record.level(), Color::White, true),
                format_styled(record.target(), Color::White, true),
                message,
            )),
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
}

fn get_log_file_config() -> Result<fern::Dispatch, AppError> {
    Ok(fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}: {} - {}",
                Local::now().format("%d-%m-%Y %H:%M:%S %z"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            fern::log_file(format!("logs/{}.log", Local::now().date_naive())).map_err(|error| {
                crate::unexpected_error!(error, "failed to initialize log file")
            })?,
        ))
}
