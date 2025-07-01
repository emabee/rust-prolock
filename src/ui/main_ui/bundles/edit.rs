use crate::ui::{
    assets::IMG_WIZARD,
    colors::{COLOR_SECRET, COLOR_USER},
    controller::{Action, Controller},
    show_error,
    viz::{VEditBundle, VEditCred},
};
use egui::{Button, Color32, FontFamily, FontId, Image, ScrollArea, TextEdit};
use egui_extras::{Size, Strip, StripBuilder};

pub fn edit(
    v_edit_bundle: &mut VEditBundle,
    error: Option<&str>,
    inner_bundle_strip: &mut Strip<'_, '_>,
    controller: &mut Controller,
) {
    inner_bundle_strip.strip(|left_builder| {
        left_part(v_edit_bundle, error, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        right_part(v_edit_bundle, right_builder, controller);
    });
}

fn left_part(v_edit_bundle: &mut VEditBundle, error: Option<&str>, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                let response = ui.add(
                    TextEdit::singleline(v_edit_bundle.key.as_mut())
                        .hint_text(t!("_unique_bundle_name"))
                        .desired_width(400.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        }),
                );
                if v_edit_bundle.request_focus {
                    v_edit_bundle.request_focus = false;
                    response.request_focus();
                }
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [400., 80.],
                        TextEdit::multiline(&mut v_edit_bundle.description)
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

fn right_part(
    edit_bundle: &mut VEditBundle,
    right_builder: StripBuilder<'_>,
    controller: &mut Controller,
) {
    right_builder
        .sizes(Size::exact(20.), edit_bundle.v_edit_creds.len())
        .vertical(|mut right_strip| {
            for (cred_idx, v_cred) in &mut edit_bundle.v_edit_creds.iter_mut().enumerate() {
                right_strip.strip(|cred_builder| {
                    single_cred(v_cred, cred_idx, cred_builder, controller);
                });
            }
        });
}

fn single_cred(
    v_edit_cred: &mut VEditCred,
    cred_idx: usize,
    cred_builder: StripBuilder<'_>,
    controller: &mut Controller,
) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(158.))
        .size(Size::exact(10.))
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
            cred_strip.cell(|ui| {
                if ui
                    .add(
                        Button::image(
                            Image::new(IMG_WIZARD)
                                .maintain_aspect_ratio(true)
                                .fit_to_original_size(0.12),
                        )
                        .fill(Color32::WHITE),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(t!("Generate password"));
                    })
                    .clicked()
                {
                    controller.set_action(Action::StartGeneratePassword(cred_idx));
                }
            });
        });
}
