use egui::{
    Color32, ComboBox, Context, FontFamily, FontId, Grid, Modal, Rgba, RichText, ScrollArea,
    TextEdit, TextStyle, Ui, scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};
use jiff::Zoned;

use crate::{
    ctrl::Controller,
    data::{Bundle, Cred, Key, PlFile, Transient},
    ui::{
        Action,
        colors::{COLOR_SECRET, COLOR_USER},
        modals::buttons,
        show_error,
        sizes::{BUNDLE_HEIGHT, MODAL_WIDTH},
        viz::{IMPORT_ACTIONS, ImportAction, ImportStartCondition, ImportStep},
    },
};

pub fn import_data(
    pl_file: &PlFile,
    step: &mut ImportStep,
    error: &mut Option<String>,
    controller: &mut Controller,
    ctx: &Context,
) {
    match step {
        ImportStep::AskForFileAndPassword { file_path, pw } => {
            let modal_response = Modal::new("import_data_1".into()).show(ctx, |ui| {
                ui.set_width(MODAL_WIDTH);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(140.);
                        ui.set_height(140.);
                        ui.add_space(50.);
                        ui.label(
                            RichText::new("📦").font(FontId::new(128., FontFamily::Proportional)),
                        );
                    });
                    ui.add_space(9.);
                    ui.vertical(|ui| {
                        ui.add_space(50.);
                        ui.label(RichText::new(t!("import_data")).size(24.));
                        ui.add_space(15.);
                        file_and_pw(file_path, pw, ui);
                        if let Some(ref e) = *error {
                            show_error(e, ui);
                        }
                    });
                });
                ui.add_space(15.);
                ui.separator();
                buttons::buttons(
                    ui,
                    controller,
                    Some(t!("import_entries")),
                    Action::FinalizeImportData,
                );
            });
            if modal_response.should_close() {
                controller.set_action(Action::CloseModal);
            }
        }

        ImportStep::ChooseImportActions {
            file,
            start_conditions,
            actions,
        } => {
            let modal_response = Modal::new("import_data_2".into()).show(ctx, |ui| {
                ui.set_width(MODAL_WIDTH + 45.);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(140.);
                        ui.set_height(140.);
                        ui.add_space(50.);
                        ui.label(
                            RichText::new("📦").font(FontId::new(128., FontFamily::Proportional)),
                        );
                    });
                    ui.add_space(9.);
                    ui.vertical(|ui| {
                        ui.add_space(50.);
                        ui.label(RichText::new(t!("import_data")).size(24.));
                        ui.add_space(10.);
                        ui.label(RichText::new(file.file_path()).monospace());

                        ui.add_space(15.);
                        // show what to import
                        bundles_and_docs(ui, file, start_conditions, actions, pl_file);
                    });
                });
                ui.add_space(15.);
                ui.separator();
                buttons::buttons(
                    ui,
                    controller,
                    Some(t!("import_entries")),
                    Action::FinalizeImportData,
                );
            });
            if modal_response.should_close() {
                controller.set_action(Action::CloseModal);
            }
        }
    }
}

fn file_and_pw(file_path: &mut String, pw: &mut String, ui: &mut Ui) {
    Grid::new("Open import file").num_columns(2).show(ui, |ui| {
        ui.label(format!("{}:", t!("File")));
        ui.add(
            TextEdit::singleline(file_path)
                .hint_text(t!("File path"))
                .desired_width(200.),
        );
        ui.end_row();

        ui.label(format!("{}:", t!("Password")));
        ui.add(
            TextEdit::singleline(pw)
                .hint_text(t!("Password"))
                .desired_width(120.)
                .password(true),
        );
        ui.end_row();
    });
}

fn bundles_and_docs(
    ui: &mut Ui,
    file: &PlFile,
    start_conditions: &[ImportStartCondition],
    actions: &mut [ImportAction],
    pl_file: &PlFile,
) {
    let _ = pl_file;
    ui.add_space(20.);
    StripBuilder::new(ui)
        .size(Size::exact(350.))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ScrollArea::vertical()
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                    .show(ui, |ui| {
                        StripBuilder::new(ui)
                            .sizes(Size::exact(BUNDLE_HEIGHT), file.bundles().len())
                            .vertical(|mut bundle_strip| {
                                let mut alternate = false;
                                for (((key, bundle), start_condition), action) in
                                    file.bundles().iter().zip(start_conditions).zip(actions)
                                {
                                    alternate = !alternate;
                                    bundle_strip.strip(|bundle_builder| {
                                        show_a_bundle(
                                            bundle_builder,
                                            alternate,
                                            key,
                                            bundle,
                                            file.transient().unwrap(),
                                            *start_condition,
                                            action,
                                        );
                                    });
                                }
                            });
                    });
            });
        });
}

// —————————————————————-
// o Import as new bundle         o Skip
// Name of the bundle
// Description and creds in display mode

// —————————————————————-
// o Skip identically existing bundle
// Name of the bundle
// Description and creds in display mode

// —————————————————————-
// o overwrite existing bundle     o Rename old and import new bundle     o skip
// Name of the bundle
// new description (edit) |   existing description
// new cred         |
//                  |  deleted cred
// identical cred   =  identical cred
// deviating cred   |  existing cred
#[allow(clippy::too_many_arguments)]
fn show_a_bundle(
    bundle_builder: StripBuilder<'_>,
    alternate: bool,
    key: &Key,
    bundle: &Bundle,
    transient: &Transient,
    start_condition: ImportStartCondition,
    action: &mut ImportAction,
) {
    if matches!(start_condition, ImportStartCondition::Identical) {
        bundle_builder
            .size(Size::exact(15.))
            .size(Size::exact(20.))
            .vertical(|mut inner_bundle_strip| {
                inner_bundle_strip.strip(|descr_builder| {
                    description_part(descr_builder, alternate, bundle, key, false);
                });
                inner_bundle_strip.cell(|ui| {
                    ui_action_selection(ui, start_condition, action);
                    ui.add_space(10.);
                    ui.separator();
                    ui.add_space(20.);
                });
            });
    } else {
        bundle_builder
            .size(Size::exact(90.))
            .size(Size::exact(90.))
            .size(Size::exact(20.))
            .vertical(|mut inner_bundle_strip| {
                inner_bundle_strip.strip(|descr_builder| {
                    description_part(descr_builder, alternate, bundle, key, true);
                });
                inner_bundle_strip.strip(|cred_builder| {
                    cred_part(cred_builder, alternate, bundle, transient);
                });
                inner_bundle_strip.cell(|ui| {
                    ui_action_selection(ui, start_condition, action);
                    ui.add_space(10.);
                    ui.separator();
                });
            });
    }
}

fn ui_action_selection(
    ui: &mut Ui,
    start_condition: ImportStartCondition,
    action: &mut ImportAction,
) {
    ui.horizontal(|ui| {
        if matches!(start_condition, ImportStartCondition::Identical) {
            ui.label(RichText::new(start_condition.to_string()).italics());
        } else {
            ui.label(
                RichText::new(format!("{start_condition}:"))
                    .italics()
                    .color(Color32::DARK_GREEN),
            );
            ComboBox::from_id_salt("choose_action")
                .selected_text(action.to_string())
                .show_ui(ui, |ui| {
                    for act in IMPORT_ACTIONS[start_condition as usize] {
                        ui.selectable_value(action, *act, act.to_string());
                    }
                });
        }
    });
}

fn description_part(
    descr_builder: StripBuilder<'_>,
    alternate: bool,
    bundle: &Bundle,
    key: &Key,
    full: bool,
) {
    let mut descr_builder = descr_builder.size(Size::exact(15.));
    if full {
        descr_builder = descr_builder.size(Size::exact(40.));
    }
    descr_builder.vertical(|mut strip| {
        //name
        strip.cell(|ui| {
            ui.separator();
            set_faded_bg_color(ui, if full { 90. } else { 23. }, alternate, true);
            ui.add(
                TextEdit::singleline(&mut key.as_str())
                    .desired_width(395.)
                    .clip_text(true)
                    .font(TextStyle::Heading)
                    .interactive(true),
            );
        });

        if full {
            // description
            strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [390., 65.],
                        TextEdit::multiline(&mut bundle.description()).interactive(true),
                    );
                });
            });
        }
    });
}

fn cred_part(
    cred_builder: StripBuilder<'_>,
    alternate: bool,
    bundle: &Bundle,
    transient: &Transient,
) {
    cred_builder
        .sizes(Size::exact(20.), 4)
        .size(Size::exact(7.))
        .vertical(|mut strip| {
            let mut first = true;
            for cred in bundle.creds() {
                strip.strip(|cred_builder| {
                    show_cred(first, alternate, cred, transient, cred_builder);
                    first = false;
                });
            }
            for _ in bundle.creds().len()..4 {
                strip.strip(|cred_builder| {
                    cred_builder
                        .size(Size::exact(20.))
                        .horizontal(|mut cred_strip| {
                            cred_strip.cell(|ui| {
                                ui.label("");
                            });
                        });
                });
            }
            strip.cell(|ui| {
                ui.horizontal(|ui| {
                    if bundle.last_changed_at() != Zoned::default() {
                        ui.label(
                            RichText::new(t!("_last_update_at"))
                                .color(Color32::GRAY)
                                .font(FontId::new(8., FontFamily::Proportional)),
                        );
                        ui.label(
                            RichText::new(bundle.last_changed_at().to_string())
                                .color(Color32::GRAY)
                                .font(FontId::new(8., FontFamily::Proportional)),
                        );
                    }
                });
            });
        });
}

pub fn show_cred(
    first: bool,
    alternate: bool,
    cred: &Cred,
    transient: &Transient,
    cred_builder: StripBuilder<'_>,
) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                if first {
                    set_faded_bg_color(ui, 95., alternate, false);
                }
                ui.add(
                    TextEdit::singleline(&mut cred.name(transient))
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(COLOR_USER)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                if first {
                    set_faded_bg_color(ui, 95., alternate, false);
                }
                ui.add(
                    TextEdit::singleline(&mut cred.secret(transient))
                        .desired_width(160.)
                        .clip_text(true)
                        .text_color(COLOR_SECRET)
                        .interactive(true),
                );
            });
        });
}

fn set_faded_bg_color(ui: &mut Ui, height: f32, color_switch: bool, left: bool) {
    let bg_color = ui.visuals().window_fill();
    let t = if color_switch { 0.91 } else { 0.8 };

    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(height);
    ui.painter().rect_filled(
        rect,
        0.0,
        if left {
            egui::lerp(Rgba::from(Color32::DARK_GRAY)..=Rgba::from(bg_color), t)
        } else {
            egui::lerp(Rgba::from(Color32::DARK_BLUE)..=Rgba::from(bg_color), t)
        },
    );
}
