use crate::ui::{
    colors::{COLOR_SECRET, COLOR_USER},
    show_error,
    viz::{VEditBundle, VEditCred},
};
use egui::{FontFamily, FontId, ScrollArea, TextEdit};
use egui_extras::{Size, Strip, StripBuilder};

pub fn edit(
    v_edit_bundle: &mut VEditBundle,
    error: &Option<String>,
    inner_bundle_strip: &mut Strip<'_, '_>,
) {
    inner_bundle_strip.strip(|left_builder| {
        left_part(v_edit_bundle, error, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        right_part(v_edit_bundle, right_builder);
    });
}

fn left_part(edit: &mut VEditBundle, error: &Option<String>, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                let response = ui.add(
                    TextEdit::singleline(&mut edit.name)
                        .hint_text(t!("_unique_bundle_name"))
                        .desired_width(400.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        })
                        .interactive(true),
                );
                if edit.request_focus {
                    edit.request_focus = false;
                    response.request_focus();
                }
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [400., 80.],
                        TextEdit::multiline(&mut edit.description)
                            .hint_text(t!("Further description (optional)"))
                            .interactive(true),
                    );
                });
                if let Some(e) = error {
                    show_error(e, ui);
                }
            });
        });
}

fn right_part(edit_bundle: &mut VEditBundle, right_builder: StripBuilder<'_>) {
    right_builder
        .sizes(Size::exact(20.), edit_bundle.v_edit_creds.len())
        .vertical(|mut right_strip| {
            for v_cred in &mut edit_bundle.v_edit_creds {
                right_strip.strip(|cred_builder| {
                    single_cred(v_cred, cred_builder);
                });
            }
        });
}

pub fn single_cred(v_edit_cred: &mut VEditCred, cred_builder: StripBuilder<'_>) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                ui.add(
                    TextEdit::singleline(&mut v_edit_cred.name)
                        .hint_text(t!("_hint_username"))
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(COLOR_USER)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                ui.add(
                    TextEdit::singleline(&mut v_edit_cred.secret)
                        .hint_text(t!("_hint_secret"))
                        .desired_width(160.)
                        .clip_text(true)
                        .text_color(COLOR_SECRET)
                        .interactive(true),
                );
            });
        });
}
