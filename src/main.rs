#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

mod data;
mod ui;

use crate::{
    data::PlFile,
    ui::{
        sizes::{WIN_HEIGHT, WIN_MIN_HEIGHT, WIN_WIDTH},
        Ui,
    },
};
use anyhow::{anyhow, Context, Result};
use eframe::{run_native, NativeOptions};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use image::{ImageError, ImageReader};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

/* TODOs *************************************

Backlog:
- Header visuell abheben
- show some start-help text when plfile is empty
- Suchfeld, um Einträge schnell finden zu können
    - Konkretere Beispiele erzeugen
    - sehr viele Beispiele erzeugen

******************************************* */

fn main() -> ExitCode {
    rust_i18n::set_locale("de");

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
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    run_native(
        "ProLock",
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([WIN_WIDTH, WIN_HEIGHT])
                .with_min_inner_size([WIN_WIDTH, WIN_MIN_HEIGHT])
                .with_app_id("ProLock")
                .with_icon(
                    load_icon_from_path(&PathBuf::from("./src/ui/assets/logo.png")).unwrap(),
                ),
            ..Default::default()
        },
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(
                // build UiApp (which implements egui::App) and hand it over to eframe::run_native,
                // which will then call its method `update()` in an endless loop
                Ui::new(PlFile::read_or_create().context("File open error")?),
            ))
        }),
    )
    .map_err(|e| anyhow!("Couldn't start GUI, caused by {e}"))
}

fn load_icon_from_path(path: &Path) -> Result<IconData, ImageError> {
    let image = ImageReader::open(path)?.decode()?;
    Ok(IconData {
        rgba: image.to_rgba8().as_flat_samples().as_slice().to_vec(),
        width: image.width(),
        height: image.height(),
    })
}
