use super::super::viz::{VEditBundle, VNamedSecret};
use super::Colors;
use egui::{Button, Color32, Context, FontFamily, FontId, Rgba, ScrollArea, TextEdit, Ui};
use egui_extras::{Size, Strip, StripBuilder};

pub(crate) fn ui(
    ctx: &Context,
    colors: &Colors,
    v_edit_bundle: &mut VEditBundle,
    inner_bundle_strip: &mut Strip<'_, '_>,
) {
    inner_bundle_strip.strip(|left_builder| {
        left_part(v_edit_bundle, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        right_part(ctx, colors, v_edit_bundle, right_builder);
    });
}

fn left_part(v_edit_bundle: &mut VEditBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                set_faded_bg_color(ui, 20.);
                ui.add(
                    TextEdit::singleline(&mut v_edit_bundle.name)
                        .hint_text("Unique name of the entry")
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
                    set_faded_bg_color(ui, f32::INFINITY);
                    ui.add_sized(
                        [380., 80.],
                        TextEdit::multiline(&mut v_edit_bundle.description)
                            .hint_text("Further description (optional)")
                            .interactive(true),
                    );
                });
            });
        });
}

fn right_part(
    ctx: &Context,
    colors: &Colors,
    v_edit_bundle: &mut VEditBundle,
    right_builder: StripBuilder<'_>,
) {
    right_builder
        .sizes(Size::exact(20.), v_edit_bundle.v_named_secrets.len())
        .vertical(|mut right_strip| {
            for named_secret in &mut v_edit_bundle.v_named_secrets {
                right_strip.strip(|cred_builder| {
                    single_cred(ctx, colors, named_secret, cred_builder);
                });
            }
        });
}

pub(crate) fn single_cred(
    ctx: &Context,
    colors: &Colors,
    named_secret: &mut VNamedSecret,
    cred_builder: StripBuilder<'_>,
) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20.);
                ui.add(
                    TextEdit::singleline(&mut named_secret.name)
                        .hint_text("User name etc")
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
                            .hint_text("Secret password etc")
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(colors.secret)
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
                                if instant.elapsed() > std::time::Duration::from_millis(800) {
                                    named_secret.copied_at = None;
                                }
                            }
                        }
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
