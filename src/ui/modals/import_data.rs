use crate::file_dialog::FileDialog;
use egui::{
    Button, Color32, ComboBox, Context, FontFamily, FontId, Frame, Grid, Rgba, RichText,
    ScrollArea, Stroke, TextEdit, TextStyle, Ui, scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};
use egui_modal_with_titlebar::ModalWithTitlebar;
use jiff::Zoned;

use crate::{
    ctrl::Controller,
    data::{Bundle, Cred, Document, Key, PlFile, Transient},
    ui::{
        Action,
        colors::{COLOR_SECRET, COLOR_USER},
        modals::buttons,
        show_error,
        sizes::{DOCUMENT_NAME_HEIGHT, MODAL_WIDTH_EX_IM},
        viz::{
            IMPORT_ACTIONS, ImportControl, ImportControls, ImportStartCondition, ImportTab,
            VImportStep,
        },
    },
};

const FRAME_COLOR: Color32 = Color32::DARK_GREEN;

pub fn import_data(
    step: &mut VImportStep,
    error: &mut Option<String>,
    controller: &mut Controller,
    ctx: &Context,
) {
    match step {
        VImportStep::AskForFileAndPassword {
            file_path,
            pw,
            file_dialog,
        } => {
            let modal_response = ModalWithTitlebar::new("import_data_1", t!("import_data"), true)
                .show(ctx, |ui| {
                    ui.set_width(MODAL_WIDTH_EX_IM);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(140.);
                            ui.set_height(140.);
                            ui.add_space(20.);
                            ui.label(
                                RichText::new("📦")
                                    .font(FontId::new(128., FontFamily::Proportional)),
                            );
                        });
                        ui.add_space(9.);
                        ui.vertical(|ui| {
                            ui.add_space(35.);

                            file_and_pw(file_path, pw, file_dialog, ui);

                            if let Some(ref e) = *error {
                                show_error(e, ui);
                            }
                        });
                    });
                    ui.add_space(5.);
                    ui.separator();
                    buttons::cancel_and_action(
                        ui,
                        controller,
                        Some(format!("{} ...", t!("inspect_entries")).into()),
                        Action::FinalizeImportData,
                    );
                });
            if modal_response.should_close() || modal_response.inner.1 {
                controller.set_action(Action::CloseModal);
            }
        }

        VImportStep::ChooseImportActions {
            file,
            import_tab,
            bundle_importcontrols,
            doc_importcontrols,
        } => {
            let modal_response = ModalWithTitlebar::new("import_data_2", t!("import_data"), true)
                .show(ctx, |ui| {
                    ui.set_width(MODAL_WIDTH_EX_IM);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(140.);
                            ui.set_height(140.);
                            ui.add_space(50.);
                            ui.label(
                                RichText::new("📦")
                                    .font(FontId::new(128., FontFamily::Proportional)),
                            );
                        });
                        ui.add_space(9.);
                        ui.vertical(|ui| {
                            ui.add_space(35.);
                            ui.label(RichText::new(file.file_path()).monospace());

                            ui.add_space(15.);
                            // show what to import
                            bundles_and_docs(
                                ui,
                                import_tab,
                                file,
                                bundle_importcontrols,
                                doc_importcontrols,
                            );
                        });
                    });
                    ui.add_space(15.);
                    ui.separator();
                    buttons::cancel_and_action(
                        ui,
                        controller,
                        Some(t!("import_entries")),
                        Action::FinalizeImportData,
                    );
                });
            if modal_response.should_close() || modal_response.inner.1 {
                controller.set_action(Action::CloseModal);
            }
        }
    }
}

fn file_and_pw(file_path: &mut String, pw: &mut String, file_dialog: &mut FileDialog, ui: &mut Ui) {
    Grid::new("Open import file").num_columns(2).show(ui, |ui| {
        ui.label(format!("{}:", t!("File")));
        ui.horizontal(|ui| {
            ui.add(
                TextEdit::singleline(file_path)
                    .hint_text(t!("File path"))
                    .font(FontId::new(12., FontFamily::Monospace))
                    .desired_width(400.),
            );

            if ui.button(format!("📂 {}...", t!("_select"))).clicked() {
                file_dialog.pick_file();
            }

            file_dialog.update(ui.ctx());

            if let Some(path) = file_dialog.take_picked() {
                *file_path = path.display().to_string();
            }
        });

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
    import_tab: &mut ImportTab,
    file: &PlFile,
    bundle_importcontrols: &mut ImportControls,
    doc_importcontrols: &mut ImportControls,
) {
    tabs(ui, import_tab);

    // ui.add_space(20.);
    match import_tab {
        ImportTab::Bundles => {
            StripBuilder::new(ui)
                .size(Size::exact(350.))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::vertical()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                            .show(ui, |ui| {
                                StripBuilder::new(ui)
                                    .sizes(Size::exact(60.), file.bundles().len())
                                    .vertical(|mut bundle_strip| {
                                        for ((key, bundle), import_control) in
                                            file.bundles().iter().zip(bundle_importcontrols)
                                        {
                                            bundle_strip.cell(|ui| {
                                                Frame::default()
                                                    .stroke(Stroke::new(2_f32, FRAME_COLOR))
                                                    .inner_margin(6.0)
                                                    .show(ui, |ui| {
                                                        show_a_bundle(
                                                            StripBuilder::new(ui),
                                                            false,
                                                            key,
                                                            bundle,
                                                            file.transient().unwrap(),
                                                            import_control,
                                                        );
                                                    });
                                                ui.add_space(10.);
                                            });
                                        }
                                    });
                            });
                    });
                });
        }
        ImportTab::Documents => {
            StripBuilder::new(ui)
                .size(Size::exact(250.))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::vertical()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                            .show(ui, |ui| {
                                StripBuilder::new(ui)
                                    .sizes(
                                        Size::exact(DOCUMENT_NAME_HEIGHT),
                                        file.documents().len(),
                                    )
                                    .vertical(|mut doc_strip| {
                                        for ((key, doc), import_control) in
                                            file.documents().iter().zip(doc_importcontrols)
                                        {
                                            doc_strip.cell(|ui| {
                                                Frame::default()
                                                    .stroke(Stroke::new(2_f32, FRAME_COLOR))
                                                    .inner_margin(2.0)
                                                    .show(ui, |ui| {
                                                        show_a_doc(
                                                            StripBuilder::new(ui),
                                                            false,
                                                            key,
                                                            doc,
                                                            file.transient().unwrap(),
                                                            import_control,
                                                        );
                                                    });
                                                ui.add_space(10.);
                                            });
                                        }
                                    });
                            });
                    });
                });
        }
    }
}

fn tabs(ui: &mut Ui, import_tab: &mut ImportTab) {
    ui.horizontal(|ui| {
        // ui.add_space(14.);
        if ui
            .add(
                Button::new(
                    RichText::new(t!("Structured entries")).size(20.), // .line_height(Some(18.)),
                )
                .fill(if import_tab.is_bundles() {
                    Color32::GRAY
                } else {
                    Color32::LIGHT_GRAY
                })
                .frame(true),
            )
            .clicked()
        {
            *import_tab = ImportTab::Bundles;
        }
        ui.add_space(4.);
        if ui
            .add(
                Button::new(
                    RichText::new(t!("Documents")).size(20.), // .line_height(Some(18.)),
                )
                .fill(if import_tab.is_documents() {
                    Color32::GRAY
                } else {
                    Color32::LIGHT_GRAY
                })
                .frame(true),
            )
            .clicked()
        {
            *import_tab = ImportTab::Documents;
        }
    });
    ui.add_space(5.);
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
fn show_a_bundle(
    bundle_builder: StripBuilder<'_>,
    alternate: bool,
    key: &Key,
    bundle: &Bundle,
    transient: &Transient,
    import_control: &mut ImportControl,
) {
    if matches!(
        import_control.start_condition,
        ImportStartCondition::Identical
    ) {
        bundle_builder
            .size(Size::exact(30.))
            .size(Size::exact(20.))
            .vertical(|mut inner_bundle_strip| {
                inner_bundle_strip.strip(|descr_builder| {
                    bundle_description_part(descr_builder, alternate, bundle, key, false);
                });
                inner_bundle_strip.cell(|ui| {
                    ui_action_selection(ui, import_control);
                });
            });
    } else {
        bundle_builder
            .size(Size::exact(90.))
            .size(Size::exact(90.))
            .size(Size::exact(20.))
            .vertical(|mut inner_bundle_strip| {
                inner_bundle_strip.strip(|descr_builder| {
                    bundle_description_part(descr_builder, alternate, bundle, key, true);
                });
                inner_bundle_strip.strip(|cred_builder| {
                    cred_part(cred_builder, alternate, bundle, transient);
                });
                inner_bundle_strip.cell(|ui| {
                    ui_action_selection(ui, import_control);
                });
            });
    }
}

fn show_a_doc(
    doc_builder: StripBuilder<'_>,
    alternate: bool,
    key: &Key,
    doc: &Document,
    transient: &Transient,
    import_control: &mut ImportControl,
) {
    doc_builder
        .size(Size::exact(15.))
        .size(Size::exact(20.))
        .vertical(|mut inner_doc_strip| {
            inner_doc_strip.strip(|descr_builder| {
                doc_description_part(descr_builder, alternate, doc, key, transient, false);
            });
            inner_doc_strip.cell(|ui| {
                ui_action_selection(ui, import_control);
            });
        });
}

fn ui_action_selection(ui: &mut Ui, import_control: &mut ImportControl) {
    ui.separator();
    //ui.add_space(-2.);
    Frame::default()
        .stroke(Stroke {
            width: 2.,
            color: FRAME_COLOR.gamma_multiply(0.3),
        })
        .fill(FRAME_COLOR.gamma_multiply(0.5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if matches!(
                    import_control.start_condition,
                    ImportStartCondition::Identical
                ) {
                    ui.label(
                        RichText::new(format!("{}.", import_control.start_condition))
                            .italics()
                            .monospace()
                            .size(15.)
                            .color(Color32::WHITE),
                    );
                } else {
                    ui.label(
                        RichText::new(format!("{}:", import_control.start_condition))
                            .italics()
                            .monospace()
                            .size(15.)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ComboBox::from_id_salt("choose_action")
                        .selected_text(
                            RichText::new(format!("{}", import_control.action))
                                .italics()
                                .monospace()
                                .size(15.)
                                .strong()
                                .color(Color32::DARK_GREEN),
                        )
                        .show_ui(ui, |ui| {
                            // ui.style_mut().visuals.panel_fill()
                            for act in IMPORT_ACTIONS[import_control.start_condition as usize] {
                                ui.selectable_value(
                                    &mut import_control.action,
                                    *act,
                                    RichText::new(format!("{act}"))
                                        .italics()
                                        .monospace()
                                        .size(15.)
                                        .color(Color32::DARK_GREEN),
                                );
                            }
                        });
                }
                ui.take_available_space();
            })
        });
}

fn bundle_description_part(
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
            set_faded_bg_color(ui, if full { 90. } else { 23. }, alternate, true);
            ui.add(
                TextEdit::singleline(&mut key.as_str())
                    .desired_width(530.)
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

fn doc_description_part(
    descr_builder: StripBuilder<'_>,
    alternate: bool,
    doc: &Document,
    key: &Key,
    transient: &Transient,
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
                    .desired_width(530.)
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
                        TextEdit::multiline(&mut doc.text(transient)).interactive(true),
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
