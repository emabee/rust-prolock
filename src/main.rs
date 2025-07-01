#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

mod args;
mod data;
mod ui;
mod util;

use crate::{
    data::PlFile,
    ui::{
        pl_app::PlApp,
        sizes::{WIN_HEIGHT, WIN_MIN_HEIGHT, WIN_WIDTH},
    },
};
use anyhow::{Result, anyhow};
use args::Args;
use data::Settings;
use eframe::{NativeOptions, run_native};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use flexi_logger::{LogSpecification, Logger};
use image::{ImageError, ImageFormat, ImageReader};
use log::LevelFilter;
use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
    process::ExitCode,
};

pub const PROG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Language = (&'static str, &'static str);
pub const SUPPORTED_LANGUAGES: [Language; 2] = [("en", "English"), ("de", "Deutsch")];
pub const DEFAULT_LANGUAGE: &Language = &SUPPORTED_LANGUAGES[0];

fn main() -> ExitCode {
    i18n!("locales", fallback = "en");

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error occured: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    // use a logspec that focuses on the "prolock" module and sets the default level to Info
    // log to a buffer and use a custom log format that fits to show_log()
    let logger_handle = Logger::with(
        LogSpecification::builder()
            .default(LevelFilter::Info)
            .module("prolock", LevelFilter::Trace)
            .build(),
    )
    .log_to_buffer(60_000, Some(pl_action_log_format))
    .start()?;

    let args = Args::from_command_line();
    let mut settings = Settings::read_or_create(args.is_test())?;

    log::info!(
        "{}",
        if args.is_test() {
            t!("_started_in_test_mode %{p}", p = PROG_NAME)
        } else {
            t!("_started %{p}", p = PROG_NAME)
        }
    );

    if let Some(file) = args.file() {
        log::info!("{}: {file}", t!("File given on commandline"));
        settings.add_and_set_file(&PathBuf::from(file))?;
    }

    if args.list_known_files() {
        settings.files.iter().enumerate().for_each(|(idx, f)| {
            println!(
                "{} {}{}",
                f.display(),
                settings.default_marker(idx),
                settings.current_marker(idx)
            );
        });
        return Ok(());
    }

    if let Some(file) = args.forget_file() {
        settings.forget_file(&PathBuf::from(file))?;
        return Ok(());
    }

    run_native(
        PROG_NAME,
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([WIN_WIDTH, WIN_HEIGHT])
                .with_min_inner_size([WIN_WIDTH, WIN_MIN_HEIGHT])
                .with_app_id(PROG_NAME)
                .with_icon(pl_load_icon()),
            ..Default::default()
        },
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(
                // build PlApp (which implements egui::App) and hand it over to eframe::run_native,
                // which will then call its method `update()` in an endless loop
                PlApp::new(logger_handle, settings)?,
            ))
        }),
    )
    .map_err(|e| anyhow!("Couldn't start GUI, caused by {e:?}"))
}

fn pl_load_icon() -> IconData {
    if let Ok(image) = read_logo() {
        IconData {
            rgba: image.to_rgba8().as_flat_samples().as_slice().to_vec(),
            width: image.width(),
            height: image.height(),
        }
    } else {
        IconData::default()
    }
}
fn read_logo() -> Result<image::DynamicImage, ImageError> {
    let bytes = include_bytes!("ui/assets/logo.png");
    ImageReader::with_format(BufReader::new(Cursor::new(bytes)), ImageFormat::Png).decode()
}

fn pl_action_log_format(
    write: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    write!(
        write,
        "{} {} : {}",
        match record.level() {
            log::Level::Error => "E",
            log::Level::Warn => "W",
            log::Level::Info => "I",
            log::Level::Debug => "D",
            log::Level::Trace => "T",
        },
        now.format("%Y-%m-%d %H:%M:%S"),
        record.args(),
    )
}
