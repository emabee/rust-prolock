use crate::{
    ui::{
        sizes::{BUNDLE_HEIGHT, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT},
        viz::{PlModal, VEditBundle, VNamedSecret},
        Colors, IMG_CANCEL, IMG_SAVE,
    },
    PlFile,
};
use egui::{
    Button, Color32, FontFamily, FontId, Image, Rgba, RichText, ScrollArea, Sides, TextEdit, Ui,
};
use egui_extras::{Size, StripBuilder};

pub(super) fn create_bundle(
    edit_bundle: &mut VEditBundle,
    pl_modal: &mut PlModal,
    pl_file: &mut PlFile,
    need_refresh: &mut bool,
    colors: &Colors,
    ui: &mut Ui,
) {
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
                                left_part(edit_bundle, left_builder);
                            });
                            inner_bundle_strip.strip(|right_builder| {
                                right_part(colors, edit_bundle, right_builder);
                            });
                        });
                });
            });
    });

    if let Some(e) = &edit_bundle.err {
        ui.label(RichText::new(e).color(Color32::RED));
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
                let (_orig_name, name, bundle) = edit_bundle.as_bundle();
                match pl_file.save_with_added_bundle(name, bundle) {
                    Ok(()) => {
                        *pl_modal = PlModal::None;
                        *need_refresh = true;
                    }
                    Err(e) => {
                        edit_bundle.err = Some(e.to_string());
                    }
                }
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
                *pl_modal = PlModal::None;
            }
        },
    );
}

fn left_part(edit_bundle: &mut VEditBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                set_faded_bg_color(ui, 20.);
                ui.add(
                    TextEdit::singleline(&mut edit_bundle.name)
                        .hint_text(t!("unique_name"))
                        .desired_width(400.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        })
                        .interactive(true),
                );
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    set_faded_bg_color(ui, f32::INFINITY);
                    ui.add_sized(
                        [400., 80.],
                        TextEdit::multiline(&mut edit_bundle.description)
                            .hint_text(t!("Further description (optional)"))
                            .interactive(true),
                    );
                });
            });
        });
}

fn right_part(colors: &Colors, edit_bundle: &mut VEditBundle, right_builder: StripBuilder<'_>) {
    right_builder
        .sizes(Size::exact(20.), edit_bundle.v_named_secrets.len())
        .vertical(|mut right_strip| {
            for named_secret in &mut edit_bundle.v_named_secrets {
                right_strip.strip(|cred_builder| {
                    single_cred(colors, named_secret, cred_builder);
                });
            }
        });
}

fn single_cred(colors: &Colors, named_secret: &mut VNamedSecret, cred_builder: StripBuilder<'_>) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20.);
                ui.add(
                    TextEdit::singleline(&mut named_secret.name)
                        .hint_text(t!("User name etc"))
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(colors.user)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20.);
                let response = ui
                    .add(
                        TextEdit::singleline(&mut named_secret.secret)
                            .hint_text(t!("_hint_secret"))
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(colors.secret)
                            .password(!named_secret.show_secret)
                            .interactive(true),
                    )
                    .on_hover_ui(|ui| {
                        ui.style_mut().interaction.selectable_labels = true;
                    });
                named_secret.show_secret = response.hovered();
            });
        });
}

fn set_faded_bg_color(ui: &mut Ui, height: f32) {
    let dark_mode = ui.visuals().dark_mode;
    let bg_color = ui.visuals().window_fill();
    let t = if dark_mode { 0.99 } else { 0.7 };
    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(height);
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::lerp(Rgba::from(Color32::DARK_BLUE)..=Rgba::from(bg_color), t),
    );
}
