use crate::{
    Controller, PlFile,
    ui::{
        Action, IMG_BURGER, IMG_LOGO, LIGHT_GRAY, VERY_LIGHT_GRAY, assets::IMG_CHANGE_FILE, viz::V,
    },
};
use egui::{Button, Color32, FontFamily, Image, MenuBar, Panel, RichText, Ui};
use egui_extras::{Size, StripBuilder};

pub fn top_panel(pl_file: &PlFile, v: &mut V, controller: &mut Controller, ui: &mut Ui) {
    Panel::top("file").show(ui, |ui| {
        ui.add_space(2.);
        ui.horizontal(|ui| {
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::initial(700.0))
                .size(Size::remainder())
                .size(Size::exact(100.))
                .horizontal(|mut strip| {
                    strip.empty();

                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 10.0, LIGHT_GRAY);
                        ui.add_space(20.);
                        ui.label(
                            RichText::new(pl_file.file_path())
                                .family(FontFamily::Monospace)
                                .color(Color32::DARK_GRAY)
                                .background_color(VERY_LIGHT_GRAY),
                        );
                        ui.add_space(10.);
                        ui.label("  –—  ");
                        ui.add_space(10.);

                        ui.label(t!(
                            "entries_with_secrets %{n1} %{n2}",
                            n1 = pl_file.bundles().len(),
                            n2 = pl_file.bundles().count_secrets()
                        ));
                        ui.add_space(20.);
                    });

                    strip.empty();

                    strip.cell(|ui| {
                        burger_menu_button(pl_file, v, controller, ui);
                        ui.add_space(10.);
                    });
                });
        });
        ui.add_space(2.);
    });
}

fn burger_menu_button(pl_file: &PlFile, v: &mut V, controller: &mut Controller, ui: &mut egui::Ui) {
    MenuBar::new().ui(ui, |ui| {
        ui.menu_image_button(Image::new(IMG_BURGER), |ui| {
            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::image_and_text(
                        Image::new(IMG_LOGO),
                        format!("{}", t!("About ProLock")),
                    ),
                )
                .clicked()
            {
                controller.set_action(Action::ShowAbout);
            }

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::new(format!("🌐 {}…", t!("Change language"))),
                )
                .clicked()
            {
                controller.set_action(Action::StartChangeLanguage);
            }

            if ui
                .add_enabled(
                    v.modal_state.is_ready_for_modal(),
                    Button::image_and_text(
                        Image::new(IMG_CHANGE_FILE),
                        format!("{}…", t!("_choose_other_file")),
                    ),
                )
                .clicked()
            {
                controller.set_action(Action::StartChangeFile);
            }

            if ui
                .add_enabled(
                    pl_file.is_actionable() && v.modal_state.is_ready_for_modal(),
                    Button::new(format!("🔐 {}…", t!("Change password"))),
                )
                .clicked()
            {
                controller.set_action(Action::StartChangePassword);
            }

            ui.separator();

            if ui
                .add_enabled(
                    pl_file.is_actionable() && v.modal_state.is_ready_for_modal(),
                    Button::new(format!("📦 {}…", t!("export_data"))),
                )
                .clicked()
            {
                controller.set_action(Action::StartExportData);
            }

            if ui
                .add_enabled(
                    pl_file.is_actionable() && v.modal_state.is_ready_for_modal(),
                    Button::new(format!("📬 {}…", t!("import_data"))),
                )
                .clicked()
            {
                controller.set_action(Action::StartImportData);
            }

            ui.separator();

            if ui
                .add(Button::new(format!("📄 {}", t!("Show log"))))
                .clicked()
            {
                controller.set_action(Action::ShowLog);
            }

            if ui
                .add_enabled(
                    false, //v.ui_state.is_ready_for_modal(),
                    Button::new(format!("📄 {}", t!("Show content as printable document"))),
                )
                .clicked()
            {}
        });
    });
}
