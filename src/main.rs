#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

mod args;
mod controller;
mod data;
mod ui;

use crate::{
    data::PlFile,
    ui::{
        Ui,
        sizes::{WIN_HEIGHT, WIN_MIN_HEIGHT, WIN_WIDTH},
    },
};
use anyhow::{Result, anyhow};
use args::Args;
use data::Settings;
use eframe::{NativeOptions, run_native};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use image::{ImageError, ImageFormat, ImageReader};
use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
    process::ExitCode,
};

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
    // if cfg!(debug_assertions) {
    //     unsafe { std::env::set_var("RUST_BACKTRACE", "1") }
    // }

    let mut settings = Settings::read_or_create()?;

    let args = Args::from_command_line();
    if let Some(file) = args.file() {
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
        "ProLock",
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([WIN_WIDTH, WIN_HEIGHT])
                .with_min_inner_size([WIN_WIDTH, WIN_MIN_HEIGHT])
                .with_app_id("ProLock")
                .with_icon(pl_load_icon()),
            ..Default::default()
        },
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(
                // build UiApp (which implements egui::App) and hand it over to eframe::run_native,
                // which will then call its method `update()` in an endless loop
                Ui::new(settings)?,
            ))
        }),
    )
    .map_err(|e| anyhow!("Couldn't start GUI, caused by {e}"))
}

fn pl_load_icon() -> IconData {
    if let Ok(image) = read_logo() {
        return IconData {
            rgba: image.to_rgba8().as_flat_samples().as_slice().to_vec(),
            width: image.width(),
            height: image.height(),
        };
    } else {
        IconData::default()
    }
}
fn read_logo() -> Result<image::DynamicImage, ImageError> {
    let bytes = include_bytes!("ui/assets/logo.png");
    ImageReader::with_format(BufReader::new(Cursor::new(bytes)), ImageFormat::Png).decode()
}
