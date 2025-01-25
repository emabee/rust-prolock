#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(dead_code)] // FIXME

#[macro_use]
extern crate serde;

mod data;

mod sizes;
mod ui;

use anyhow::{anyhow, Context, Result};
use data::PlFile;
use eframe::{run_native, NativeOptions};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use image::{ImageError, ImageReader};
use sizes::{WIN_HEIGHT, WIN_MIN_HEIGHT, WIN_WIDTH};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
use ui::UiApp;

fn main() -> ExitCode {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // FIXME    color_setup_win10();

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("Error occured: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    // evaluate args:: do we need some?
    // let args = Args::from_command_line();

    // read the persistence
    let mut pl_file = PlFile::read_or_create().context("File open error")?;

    //
    //vv temp section (to get rid of "unused" messages vv//
    let o_storage_password: Option<String> = Some("test".to_string());
    if let Some(storage_password) = o_storage_password {
        pl_file
            .set_password(storage_password)
            .expect("Wrong password");
    }

    pl_file.add_test_bundles(false)?;
    //^^ temp section (to get rid of "unused" messages ^^//

    // prepare and show ui
    go(pl_file)?;
    // save
    //    pl_file.save().expect("save failed");
    Ok(())
}

// Build UiApp (which implements egui::App) and hand it over to eframe::run_native, which will then
// call its method `update()` in an endless loop.
pub(crate) fn go(pl_file: PlFile) -> Result<()> {
    match run_native(
        "ProLock",
        NativeOptions {
            // viewport = native OS window
            viewport: ViewportBuilder::default()
                .with_inner_size([WIN_WIDTH, WIN_HEIGHT])
                .with_min_inner_size([WIN_WIDTH, WIN_MIN_HEIGHT])
                .with_app_id("ProLock")
                .with_icon(load_icon_from_path(&PathBuf::from("./src/assets/logo.png")).unwrap()),
            ..Default::default()
        },
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(UiApp::new(pl_file)))
        }),
    ) {
        Ok(()) => Ok(()),
        Err(e) => Err(anyhow!("Couldn't start GUI, caused by {e}")),
    }
}

fn load_icon_from_path(path: &Path) -> Result<IconData, ImageError> {
    let image = ImageReader::open(path)?.decode()?;
    Ok(IconData {
        rgba: image.to_rgba8().as_flat_samples().as_slice().to_vec(),
        width: image.width(),
        height: image.height(),
    })
}
