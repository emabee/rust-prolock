use crate::{
    PlFile,
    data::Settings,
    ui::{
        IMG_BURGER, VERY_LIGHT_GRAY,
        assets::{IMG_CHANGE_FILE, IMG_CHANGE_FILE_INACTIVE},
        viz::{PlModal, Pw, PwFocus, V},
    },
};
use egui::{
    Button, Color32, Context, FontFamily, Image, RichText, TopBottomPanel, menu::menu_custom_button,
};
use egui_extras::{Size, StripBuilder};

use super::IMG_LOGO;

pub fn top_panel(settings: &Settings, pl_file: &PlFile, v: &mut V, ctx: &Context) {
    TopBottomPanel::top("file").show(ctx, |ui| {
        ui.horizontal(|ui| {
            StripBuilder::new(ui)
                .size(Size::exact(300.0))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            VERY_LIGHT_GRAY,
                        );
                        if ui
                            .add_enabled(
                                v.edit_idx.is_none(),
                                Button::image(if v.edit_idx.is_none() {
                                    Image::new(IMG_CHANGE_FILE)
                                        .maintain_aspect_ratio(true)
                                        .fit_to_original_size(0.18)
                                } else {
                                    Image::new(IMG_CHANGE_FILE_INACTIVE)
                                        .maintain_aspect_ratio(true)
                                        .fit_to_original_size(0.18)
                                })
                                .fill(VERY_LIGHT_GRAY),
                            )
                            .clicked()
                        {
                            v.pl_modal = PlModal::ChangeFile;
                        }

                        ui.label(
                            RichText::new(pl_file.file_path.display().to_string())
                                .family(FontFamily::Monospace)
                                .color(Color32::DARK_GRAY)
                                .background_color(VERY_LIGHT_GRAY),
                        );
                    });
                });
            ui.add_space(10.);
            ui.label("  –—  ");
            ui.add_space(10.);

            ui.label(t!(
                "entries_with_secrets %{n1} %{n2}",
                n1 = pl_file.stored.readable.bundles.len(),
                n2 = pl_file.stored.readable.bundles.count_secrets()
            ));

            ui.add_space(ui.available_width() - 80.);

            menu_custom_button(
                ui,
                Button::image(Image::new(IMG_BURGER)).fill(Color32::TRANSPARENT),
                |ui| {
                    if ui
                        .add(Button::image_and_text(
                            Image::new(IMG_LOGO),
                            format!("{}", t!("About ProLock")),
                        ))
                        .clicked()
                    {
                        v.pl_modal = PlModal::About;
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            pl_file.is_actionable(),
                            Button::new(format!("🔐 {}", t!("Change password"))),
                        )
                        .clicked()
                    {
                        v.pl_modal = PlModal::ChangePassword;
                        v.pw = Pw::default();
                        v.pw.focus = PwFocus::PwOld;
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            pl_file.is_actionable(),
                            Button::new(format!("🌐 {}", t!("Change language"))),
                        )
                        .clicked()
                    {
                        v.lang.init(&settings.language);
                        v.pl_modal = PlModal::ChangeLanguage;
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            false, //self.pl_file.is_actionable(),
                            Button::new(format!("📄 {}", t!("Show content as printable document"))),
                        )
                        .clicked()
                    {
                        v.pl_modal = PlModal::ShowPrintable;
                        ui.close_menu();
                    }
                },
            );
        });

        ui.add_space(10.);
    });
}
