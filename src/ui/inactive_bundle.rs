use super::v::{VNamedSecret, VBundle};
use egui::{Button, Color32, Context, FontFamily, FontId, Rgba, ScrollArea, TextEdit, Ui};
use egui_extras::{Size, StripBuilder};

pub(crate) fn show(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    index: usize,
    v_bundle: &mut VBundle,
    mut inner_bundle_strip: egui_extras::Strip<'_, '_>,
) {
    inner_bundle_strip.strip(|left_builder| {
        show_left_part(index, v_bundle, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        show_right_part(
            ctx,
            color_user,
            color_secret,
            index,
            v_bundle,
            right_builder,
        );
    });
}

fn show_left_part(index: usize, v_bundle: &mut VBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                ui.add(
                    TextEdit::singleline(&mut v_bundle.name.as_str())
                        .desired_width(330.)
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
                    set_faded_bg_color(ui, f32::INFINITY, index);
                    ui.add_sized(
                        [380., 80.],
                        TextEdit::multiline(&mut v_bundle.description.as_str()).interactive(true),
                    );
                });
            });
        });
}

fn show_right_part(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    index: usize,
    v_bundle: &mut VBundle,
    right_builder: StripBuilder<'_>,
) {
    right_builder
        .sizes(Size::exact(20.), v_bundle.v_named_secrets.len())
        .vertical(|mut right_strip| {
            for named_secret in &mut v_bundle.v_named_secrets {
                right_strip.strip(|cred_builder| {
                    show_cred(
                        ctx,
                        color_user,
                        color_secret,
                        index,
                        named_secret,
                        cred_builder,
                    );
                });
            }
        });
}

pub(crate) fn show_cred(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    index: usize,
    named_secret: &mut VNamedSecret,
    cred_builder: StripBuilder<'_>,
) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                ui.add(
                    TextEdit::singleline(&mut named_secret.name.as_str())
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(color_user)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                let response = ui
                    .add(
                        TextEdit::singleline(&mut named_secret.secret.as_str())
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(color_secret)
                            .password(!named_secret.show_secret)
                            .interactive(true),
                    )
                    .on_hover_ui(|ui| {
                        ui.style_mut().interaction.selectable_labels = true;
                        match named_secret.copied_at {
                            None => {
                                if ui
                                    .add(Button::new("  Copy").min_size([60., 10.].into()))
                                    .clicked()
                                {
                                    ctx.copy_text(named_secret.secret.clone());
                                    named_secret.copied_at = Some(std::time::Instant::now());
                                }
                            }
                            Some(instant) => {
                                ui.label("✔ Copied");
                                if std::time::Instant::now() - instant
                                    > std::time::Duration::from_millis(800)
                                {
                                    named_secret.copied_at = None;
                                }
                            }
                        }
                    });
                if response.hovered() {
                    named_secret.show_secret = true;
                } else {
                    named_secret.show_secret = false;
                };
            });
        });
}

fn set_faded_bg_color(ui: &mut Ui, height: f32, index: usize) {
    let dark_mode = ui.visuals().dark_mode;
    let bg_color = ui.visuals().window_fill();
    let t = if index % 2 == 0 {
        if dark_mode {
            0.95
        } else {
            0.91
        }
    } else {
        if dark_mode {
            0.95
        } else {
            0.8
        }
    };

    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(height);
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::lerp(Rgba::from(Color32::DARK_BLUE)..=Rgba::from(bg_color), t),
    );
}
