#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(dead_code)] // FIXME

#[macro_use]
extern crate serde;

mod action;
mod bundle;
mod bundles;
mod colors;
mod pl_file;
mod secrets;
mod transient;
mod ui;
mod v_bundles;

use anyhow::{anyhow, Context, Result};
use bundle::Bundle;
use eframe::{run_native, NativeOptions};
use egui::{IconData, ViewportBuilder};
use egui_extras::install_image_loaders;
use image::{ImageError, ImageReader};
use pl_file::PlFile;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use ui::UiApp;

fn main() -> ExitCode {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // FIXME    color_setup_win10();

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", colors::StdOutColored::red(format!("{e:?}")));
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

    pl_file.add_bundle(
        "Bank of North America",
        Bundle::new_with_creds("aaa_dscr", &[("aaa_cn", "aaa_cs")]),
    )?;
    pl_file.add_bundle(
        "Bank of South America",
        Bundle::new_with_creds(
            "http://one_bank.de\n\n\
            Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
            Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
            Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
            Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
            Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
            Hello world! Hello world! ",
            &[("aaa_cn", "aaa_cs"), ("asdaqweqweqwe", "rtzrtzfhfghgfh")],
        ),
    )?;
    // pl_file.add_bundle(
    //     "Some Bank with a very very very very very very long name",
    //     Bundle::new_with_creds(
    //         "http://some_bank.de\n\n\
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! ",
    //         &[
    //             (
    //                 "some.pretty.long.name@t-online.de",
    //                 "bbb_cs_adawdeewqfdf-rgrdt-xyz-123",
    //             ),
    //             (
    //                 "someotherveryveryveryveryveryveryveryveryveryvery.long.name@t-online.de",
    //                 "bbb_cs erw rtrz werert",
    //             ),
    //         ],
    //     ),
    // )?;
    // pl_file.add_bundle(
    //     "Some Bank with a pretty long long name",
    //     Bundle::new_with_creds(
    //         "http://some_other_bank.de\n\n\
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
    //         Hello world! Hello world! ",
    //         &[
    //             (
    //                 "some.pretty.long.name@t-online.de",
    //                 "bbb_cs_adawdeewqfdf-rgrdt-xyz-123",
    //             ),
    //             (
    //                 "someotherveryveryveryveryveryveryveryveryveryvery.long.name@t-online.de",
    //                 "bbb_cs erw rtrz werert",
    //             ),
    //         ],
    //     ),
    // )?;
    pl_file.add_bundle(
        "ccc",
        Bundle::new_with_creds(
            "ccc_dscr1\n\
            ccc_dscr2\n\
            ccc_dscr3\n\
            ccc_dscr4\n\
            ccc_dscr5",
            &[
                ("ccc_cn1", "ccc_cs"),
                ("ccc_cn2", "ccc_cs"),
                ("ccc_cn3", "ccc_cs"),
            ],
        ),
    )?;
    pl_file.add_bundle(
        "ddd",
        Bundle::new_with_creds(
            "ddd_dscr",
            &[
                ("ddd_cn1", "ddd_cs"),
                ("ddd_cn2", "ddd_cs"),
                ("ddd_cn3", "ddd_cs"),
            ],
        ),
    )?;
    pl_file.add_bundle(
        "eee",
        Bundle::new_with_creds("eee_dscr", &[("eee_cn", "eee_cs")]),
    )?;
    pl_file.add_bundle(
        "fff",
        Bundle::new_with_creds("fff_dscr", &[("fff_cn", "fff_cs")]),
    )?;
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
                .with_inner_size([925.0, 700.0])
                .with_min_inner_size([925.0, 200.0])
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
