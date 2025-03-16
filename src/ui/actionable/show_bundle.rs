use crate::{
    data::{Bundle, Cred, Transient},
    ui::{
        Colors,
        viz::{VBundle, VCred},
    },
};
use egui::{
    Button, Color32, Context, FontFamily, FontId, Rgba, RichText, ScrollArea, TextEdit, TextStyle,
    Ui,
};
use egui_extras::{Size, Strip, StripBuilder};
use either::Either;
use jiff::Zoned;

#[allow(clippy::too_many_arguments)] // TODO
pub fn ui(
    ctx: &Context,
    colors: &Colors,
    index: usize,
    bundle: &Bundle,
    v_bundle: &mut VBundle,
    name: &str,
    transient: &Transient,
    inner_bundle_strip: &mut Strip<'_, '_>,
) {
    inner_bundle_strip.strip(|left_builder| {
        ui_left_part(index, bundle, name, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        ui_right_part(
            index,
            bundle,
            transient,
            v_bundle,
            right_builder,
            colors,
            ctx,
        );
    });
}

fn ui_left_part(index: usize, bundle: &Bundle, name: &str, left_builder: StripBuilder<'_>) {
    let mut name2 = name;
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
                set_faded_bg_color(ui, 95., color, true);
                ui.add(
                    TextEdit::singleline(&mut name2)
                        .desired_width(330.)
                        .clip_text(true)
                        .font(TextStyle::Heading)
                        .interactive(true),
                )
                .on_hover_text(t!("Name of the entry"));
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [380., 80.],
                        TextEdit::multiline(&mut bundle.description()).interactive(true),
                    )
                    .on_hover_text(t!("Description"));
                });
            });
            left_strip.cell(|ui| {
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

fn ui_right_part(
    index: usize,
    bundle: &Bundle,
    transient: &Transient,
    v_bundle: &mut VBundle,
    right_builder: StripBuilder<'_>,
    colors: &Colors,
    ctx: &Context,
) {
    right_builder
        .sizes(Size::exact(20.), bundle.creds().len())
        .vertical(|mut right_strip| {
            let mut first = true;
            for (cred, v_cred) in bundle.creds().iter().zip(v_bundle.v_creds.iter_mut()) {
                right_strip.strip(|cred_builder| {
                    show_cred(
                        first,
                        index,
                        cred,
                        transient,
                        v_cred,
                        cred_builder,
                        colors,
                        ctx,
                    );
                    first = false;
                });
            }
        });
}

#[allow(clippy::too_many_arguments)] // TODO
pub fn show_cred(
    first: bool,
    index: usize,
    cred: &Cred,
    transient: &Transient,
    v_cred: &mut VCred,
    cred_builder: StripBuilder<'_>,
    colors: &Colors,
    ctx: &Context,
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
                if first {
                    set_faded_bg_color(ui, 95., color_switch, false);
                }
                ui.add(
                    TextEdit::singleline(&mut cred.name(transient))
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(colors.user)
                        .interactive(true),
                )
                .on_hover_text(t!("_hover_username"));
            });
            cred_strip.cell(|ui| {
                if first {
                    set_faded_bg_color(ui, 95., color_switch, false);
                }
                let response = ui
                    .add(
                        TextEdit::singleline(&mut cred.secret(transient))
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(colors.secret)
                            .password(!v_cred.show_secret)
                            .interactive(true),
                    )
                    .on_hover_ui(|ui| {
                        ui.style_mut().interaction.selectable_labels = true;
                        match v_cred.copied_at {
                            None => {
                                if ui
                                    .add(Button::new(t!("_copy")).min_size([60., 10.].into()))
                                    .clicked()
                                {
                                    ctx.copy_text(cred.secret(transient).to_string());
                                    v_cred.copied_at = Some(std::time::Instant::now());
                                }
                            }
                            Some(instant) => {
                                ui.label(t!("_copied"));
                                if instant.elapsed() > std::time::Duration::from_millis(800) {
                                    v_cred.copied_at = None;
                                }
                            }
                        }
                    })
                    .on_hover_text(t!("Secret"));
                v_cred.show_secret = response.hovered();
            });
        });
}

fn set_faded_bg_color(ui: &mut Ui, height: f32, color_switch: Either<(), ()>, left: bool) {
    let bg_color = ui.visuals().window_fill();
    let t = if color_switch.is_left() { 0.91 } else { 0.8 };

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
