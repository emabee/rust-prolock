use crate::ui::{
    IMG_CANCEL, IMG_SAVE,
    assets::IMG_WIZARD,
    colors::{COLOR_SECRET, COLOR_USER},
    controller::{Action, Controller},
    show_error,
    sizes::{BUNDLE_HEIGHT, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT},
    viz::{VEditBundle, VEditCred},
};
use egui::{
    Button, Color32, Context, FontFamily, FontId, Image, Modal, Rgba, RichText, ScrollArea, Sides,
    TextEdit,
};
use egui_extras::{Size, StripBuilder};

pub fn create_bundle(
    bundle: &mut VEditBundle,
    error: Option<&str>,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new("create_bundle".into()).show(ctx, |ui| {
        ui.vertical(|ui| {
            StripBuilder::new(ui)
                .sizes(Size::exact(BUNDLE_HEIGHT), 1)
                .vertical(|mut bundle_strip| {
                    bundle_strip.strip(|bundle_builder| {
                        bundle_builder
                            .size(Size::exact(BUNDLE_WIDTH_LEFT))
                            .size(Size::exact(BUNDLE_WIDTH_RIGHT))
                            .horizontal(|mut inner_bundle_strip| {
                                inner_bundle_strip.strip(|left_builder| {
                                    left_part(bundle, left_builder);
                                });
                                inner_bundle_strip.strip(|right_builder| {
                                    right_part(bundle, right_builder, controller);
                                });
                            });
                    });
                });
        });

        if let Some(e) = error {
            show_error(e, ui);
        }

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .add(
                        Button::image_and_text(
                            Image::new(IMG_SAVE),
                            RichText::new(t!("Save")).color(Color32::DARK_GREEN),
                        )
                        .fill(Color32::TRANSPARENT),
                    )
                    .clicked()
                {
                    controller.set_action(Action::FinalizeAddBundle);
                }

                if ui
                    .add(
                        Button::image_and_text(
                            Image::new(IMG_CANCEL)
                                .maintain_aspect_ratio(true)
                                .fit_to_original_size(0.22),
                            t!("Cancel"),
                        )
                        .fill(Color32::TRANSPARENT),
                    )
                    .clicked()
                {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}

fn left_part(edit_bundle: &mut VEditBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(20.))
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //header
            left_strip.cell(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new(t!("Unprotected header")).color(Color32::GRAY));
                    ui.add_space(3.);
                });
            });
            //name
            left_strip.cell(|ui| {
                let response = ui.add(
                    TextEdit::singleline(edit_bundle.key.as_mut())
                        .hint_text(t!("_unique_bundle_name"))
                        .desired_width(400.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        })
                        .background_color(
                            egui::lerp(
                                Rgba::from(Color32::DARK_GRAY)
                                    ..=Rgba::from(ui.visuals().window_fill()),
                                0.91,
                            )
                            .into(),
                        )
                        .interactive(true),
                );
                if edit_bundle.request_focus {
                    edit_bundle.request_focus = false;
                    response.request_focus();
                }
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [400., 80.],
                        TextEdit::multiline(&mut edit_bundle.description)
                            .hint_text(t!("Further description (optional)"))
                            .background_color(
                                egui::lerp(
                                    Rgba::from(Color32::DARK_GRAY)
                                        ..=Rgba::from(ui.visuals().window_fill()),
                                    0.91,
                                )
                                .into(),
                            )
                            .interactive(true),
                    );
                });
            });
        });
}

fn right_part(
    edit_bundle: &mut VEditBundle,
    right_builder: StripBuilder<'_>,
    controller: &mut Controller,
) {
    right_builder
        .sizes(Size::exact(20.), edit_bundle.v_edit_creds.len() + 1)
        .vertical(|mut right_strip| {
            right_strip.cell(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new(t!("Encrypted data")).color(Into::<Color32>::into(
                            egui::lerp(
                                Rgba::from(Color32::DARK_BLUE)
                                    ..=Rgba::from(ui.visuals().window_fill()),
                                0.5,
                            ),
                        )),
                    );
                });
                ui.add_space(3.);
            });
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
                        .background_color(
                            egui::lerp(
                                Rgba::from(Color32::DARK_BLUE)
                                    ..=Rgba::from(ui.visuals().window_fill()),
                                0.91,
                            )
                            .into(),
                        )
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
                        .background_color(
                            egui::lerp(
                                Rgba::from(Color32::DARK_BLUE)
                                    ..=Rgba::from(ui.visuals().window_fill()),
                                0.91,
                            )
                            .into(),
                        )
                        .interactive(true),
                )
                .on_hover_ui(|ui| {
                    ui.style_mut().interaction.selectable_labels = true;
                });
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
