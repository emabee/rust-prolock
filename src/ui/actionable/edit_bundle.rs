use crate::ui::{
    Colors,
    viz::{VEditBundle, VEditCred},
};
use egui::{FontFamily, FontId, ScrollArea, TextEdit};
use egui_extras::{Size, Strip, StripBuilder};

pub fn ui(colors: &Colors, edit_bundle: &mut VEditBundle, inner_bundle_strip: &mut Strip<'_, '_>) {
    inner_bundle_strip.strip(|left_builder| {
        left_part(edit_bundle, left_builder);
    });
    inner_bundle_strip.strip(|right_builder| {
        right_part(colors, edit_bundle, right_builder);
    });
}

fn left_part(edit_bundle: &mut VEditBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                ui.add(
                    TextEdit::singleline(&mut edit_bundle.name)
                        .hint_text(t!("_unique_name"))
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
        .sizes(Size::exact(20.), edit_bundle.v_edit_creds.len())
        .vertical(|mut right_strip| {
            for v_cred in &mut edit_bundle.v_edit_creds {
                right_strip.strip(|cred_builder| {
                    single_cred(colors, v_cred, cred_builder);
                });
            }
        });
}

pub fn single_cred(colors: &Colors, v_edit_cred: &mut VEditCred, cred_builder: StripBuilder<'_>) {
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
                        .text_color(colors.user)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                ui.add(
                    TextEdit::singleline(&mut v_edit_cred.secret)
                        .hint_text(t!("_hint_secret"))
                        .desired_width(160.)
                        .clip_text(true)
                        .text_color(colors.secret)
                        .interactive(true),
                )
                .on_hover_ui(|ui| {
                    ui.style_mut().interaction.selectable_labels = true;
                });
            });
        });
}
