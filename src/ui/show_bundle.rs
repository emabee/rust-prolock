use super::{
    v::{VBundle, VNamedSecret},
    Colors,
};
use egui::{Button, Color32, Context, FontFamily, FontId, Rgba, ScrollArea, TextEdit, Ui};
use egui_extras::{Size, Strip, StripBuilder};
use either::Either;

pub(crate) fn ui(
    ctx: &Context,
    colors: &Colors,
    index: usize,
    v_bundle: &mut VBundle,
    inner_bundle_strip: &mut Strip<'_, '_>,
) {
    inner_bundle_strip.strip(|left_builder| {
        ui_left_part(index, v_bundle, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        ui_right_part(ctx, colors, index, v_bundle, right_builder);
    });
}

fn ui_left_part(index: usize, v_bundle: &VBundle, left_builder: StripBuilder<'_>) {
    let color = if index % 2 == 0 {
        Either::Left(())
    } else {
        Either::Right(())
    };
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., color);
                ui.add(
                    TextEdit::singleline(&mut v_bundle.name.as_str())
                        .desired_width(330.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        })
                        .interactive(true),
                )
                .on_hover_text("Name of the entry");
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    set_faded_bg_color(ui, f32::INFINITY, color);
                    ui.add_sized(
                        [380., 80.],
                        TextEdit::multiline(&mut v_bundle.description.as_str()).interactive(true),
                    )
                    .on_hover_text("Description");
                });
            });
        });
}

fn ui_right_part(
    ctx: &Context,
    colors: &Colors,
    index: usize,
    v_bundle: &mut VBundle,
    right_builder: StripBuilder<'_>,
) {
    right_builder
        .sizes(Size::exact(20.), v_bundle.v_named_secrets.len())
        .vertical(|mut right_strip| {
            for v_named_secret in &mut v_bundle.v_named_secrets {
                right_strip.strip(|cred_builder| {
                    show_cred(ctx, colors, index, v_named_secret, cred_builder);
                });
            }
        });
}

pub(crate) fn show_cred(
    ctx: &Context,
    colors: &Colors,
    index: usize,
    v_named_secret: &mut VNamedSecret,
    cred_builder: StripBuilder<'_>,
) {
    let color_switch = if index % 2 == 0 {
        Either::Left(())
    } else {
        Either::Right(())
    };
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., color_switch);
                ui.add(
                    TextEdit::singleline(&mut v_named_secret.name.as_str())
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(colors.user)
                        .interactive(true),
                )
                .on_hover_text("Username, etc");
            });
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., color_switch);
                let response = ui
                    .add(
                        TextEdit::singleline(&mut v_named_secret.secret.as_str())
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(colors.secret)
                            .password(!v_named_secret.show_secret)
                            .interactive(true),
                    )
                    .on_hover_ui(|ui| {
                        ui.style_mut().interaction.selectable_labels = true;
                        match v_named_secret.copied_at {
                            None => {
                                if ui
                                    .add(Button::new("  Copy").min_size([60., 10.].into()))
                                    .clicked()
                                {
                                    ctx.copy_text(v_named_secret.secret.clone());
                                    v_named_secret.copied_at = Some(std::time::Instant::now());
                                }
                            }
                            Some(instant) => {
                                ui.label("✔ Copied");
                                if instant.elapsed() > std::time::Duration::from_millis(800) {
                                    v_named_secret.copied_at = None;
                                }
                            }
                        }
                    })
                    .on_hover_text("Secret");
                v_named_secret.show_secret = response.hovered();
            });
        });
}

fn set_faded_bg_color(ui: &mut Ui, height: f32, color_switch: Either<(), ()>) {
    let dark_mode = ui.visuals().dark_mode;
    let bg_color = ui.visuals().window_fill();
    let t = if color_switch.is_left() {
        if dark_mode {
            0.95
        } else {
            0.91
        }
    } else if dark_mode {
        0.95
    } else {
        0.8
    };

    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(height);
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::lerp(Rgba::from(Color32::DARK_BLUE)..=Rgba::from(bg_color), t),
    );
}
