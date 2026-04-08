use crate::{
    ctrl::Controller,
    ui::{
        Action,
        modals::buttons,
        show_error,
        sizes::MODAL_WIDTH,
        viz::{PwFocus, VExportData},
    },
};
use egui::{
    Context, FontFamily, FontId, Grid, Key, Modal, RichText, ScrollArea, Sides, TextEdit, TextStyle,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

pub fn export_data(v_export_data: &mut VExportData, controller: &mut Controller, ctx: &Context) {
    let go_for_it = false;

    let modal_response = Modal::new("change_password".into()).show(ctx, |ui| {
        ui.set_width(MODAL_WIDTH + 30.);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(140.);
                ui.set_height(300.);
                ui.add_space(50.);
                ui.label(
                    RichText::new("📦") // or 🚚?
                        .font(FontId::new(128., FontFamily::Proportional)),
                );
            });
            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(RichText::new(t!("export_data")).size(24.));

                ui.add_space(30.);
                StripBuilder::new(ui)
                    .size(Size::exact(140.))
                    .size(Size::exact(100.))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            bundles_and_docs(ui, v_export_data);
                        });

                        strip.cell(|ui| {
                            password_and_file(v_export_data, go_for_it, ui);

                            if let Some(e) = &v_export_data.pw.error {
                                show_error(e, ui);
                            }

                            show_buttons(v_export_data, controller, go_for_it, ui);
                        });
                    });
            });
        });
    });
    if modal_response.should_close() {
        controller.set_action(Action::CloseModal);
    }
}

fn bundles_and_docs(ui: &mut egui::Ui, v_export_data: &mut VExportData) {
    let text_height = egui::TextStyle::Body
        .resolve(ui.style())
        .size
        .max(ui.spacing().interact_size.y);
    let available_height = ui.available_height();

    ScrollArea::horizontal().show(ui, |ui| {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(350.).clip(true).resizable(false))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);
        table.body(|body| {
            body.rows(
                text_height,
                v_export_data.bundles_to_export.len(),
                |mut row| {
                    let row_index = row.index();
                    let x = &mut v_export_data.bundles_to_export[row_index];
                    row.col(|ui| {
                        ui.checkbox(&mut x.0, &x.1);
                    });
                },
            );
        });
    });

    ui.add_space(5.);

    let bundle_count = v_export_data
        .bundles_to_export
        .iter()
        .filter(|(selected, _)| *selected)
        .count();
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(t!(
                "selected_data",
                bundle_count = bundle_count,
                docs_count = 0
            ))
            .italics()
            .font(FontId::proportional(11.0)),
        );
    });
}

fn password_and_file(v_export_data: &mut VExportData, mut go_for_it: bool, ui: &mut egui::Ui) {
    ui.add_space(20.);
    // TODO: provide some help text about the password, e.g. how important it is
    // to NOT use the same password as for the vault,
    // and that the password is required to import the data.
    ui.label("Exportdatei:");
    ui.add(
        TextEdit::singleline(&mut v_export_data.file_path)
            .hint_text(t!("File path"))
            .font(TextStyle::Monospace),
    );

    ui.add_space(15.);

    Grid::new("ExportPassword").num_columns(2).show(ui, |ui| {
        ui.label(format!("{}:", t!("export_password")));
        let response = ui.add(
            TextEdit::singleline(&mut v_export_data.pw.pw2)
                .desired_width(120.)
                .password(true),
        );
        if matches!(v_export_data.pw.focus, PwFocus::Pw2) {
            response.request_focus();
            v_export_data.pw.focus = PwFocus::None;
        }
        if response.lost_focus()
            && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
        {
            v_export_data.pw.focus = PwFocus::Pw3;
        }
        ui.end_row();

        ui.label(format!("{}:", t!("repeat_password")));
        let response = ui.add(
            TextEdit::singleline(&mut v_export_data.pw.pw3)
                .desired_width(120.)
                .password(true),
        );
        if matches!(v_export_data.pw.focus, PwFocus::Pw3) {
            response.request_focus();
            v_export_data.pw.focus = PwFocus::None;
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            go_for_it = true;
        }
        ui.end_row();
    });
}

fn show_buttons(
    v_export_data: &mut VExportData,
    controller: &mut Controller,
    mut go_for_it: bool,
    ui: &mut egui::Ui,
) {
    ui.add_space(15.);
    ui.separator();

    Sides::new().show(
        ui,
        |_ui| {},
        |ui| {
            if ui.button(buttons::ok(None)).clicked() {
                go_for_it = true;
            }

            if ui.button(buttons::cancel()).clicked() {
                controller.set_action(Action::CloseModal);
            }
        },
    );

    if go_for_it {
        if !v_export_data.pw.pw2.is_empty() && v_export_data.pw.pw2 == v_export_data.pw.pw3 {
            if v_export_data
                .bundles_to_export
                .iter()
                .all(|(selected, _)| !*selected)
            {
                v_export_data.pw.error = Some(t!("_export_nothing_selected").to_string());
            } else {
                controller.set_action(Action::FinalizeExportData);
            }
        } else {
            v_export_data.pw.error = Some(t!("_export_passwords_dont_match").to_string());
        }
    }
}
