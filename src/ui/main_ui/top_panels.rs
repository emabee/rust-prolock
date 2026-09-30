use crate::{
    Controller,
    data::Documents,
    ui::{
        Action, IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_ERASE,
        sizes::SEARCH_TEXT_WIDTH,
        viz::{BundleState, DocumentState, Find, MainState, V},
    },
};
use egui::{Button, Color32, Image, Panel, RichText, Slider, SliderClamping, TextEdit, Ui};

pub(super) fn panel_with_tabs(
    v: &mut V,
    documents: &Documents,
    controller: &mut Controller,
    ui: &mut Ui,
) {
    // two tabs: Bundles and Documents
    Panel::top("panel_with_tabs").show(ui, |ui| {
        ui.add_space(10.);

        ui.horizontal(|ui| {
            ui.add_space(14.);
            if ui
                // disable the button when in edit mode
                .add_enabled(
                    v.main_state.tabs_and_create_ok(),
                    Button::new(
                        RichText::new(t!("Structured entries")).size(20.), // .line_height(Some(18.)),
                    )
                    .fill(if v.main_state.is_bundles() {
                        Color32::GRAY
                    } else {
                        Color32::LIGHT_GRAY
                    })
                    .frame(true),
                )
                .clicked()
            {
                v.main_state = MainState::Bundles(BundleState::Default);
                controller.set_action(Action::StartFilter);
            }
            ui.add_space(4.);
            if ui
                .add_enabled(
                    v.main_state.tabs_and_create_ok(),
                    Button::new(
                        RichText::new(t!("Documents")).size(20.), // .line_height(Some(18.)),
                    )
                    .fill(if v.main_state.is_documents() {
                        Color32::GRAY
                    } else {
                        Color32::LIGHT_GRAY
                    })
                    .frame(true),
                )
                .clicked()
            {
                v.main_state =
                    MainState::Documents(DocumentState::Default(if v.documents.is_empty() {
                        None
                    } else {
                        documents.iter().next().map(|(key, _)| key.clone())
                    }));
                controller.set_action(Action::StartFilter);
            }
        });
        ui.add_space(-12.);
    });
}

pub(super) fn panel_with_create_and_filter(v: &mut V, controller: &mut Controller, ui: &mut Ui) {
    Panel::top("header").show(ui, |ui| {
        ui.add_space(16.);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    v.main_state.tabs_and_create_ok(),
                    Button::image(
                        Image::new(if v.main_state.tabs_and_create_ok() {
                            IMG_ADD_ENTRY
                        } else {
                            IMG_ADD_ENTRY_INACTIVE
                        })
                        .maintain_aspect_ratio(true)
                        .fit_to_original_size(0.22),
                    )
                    .fill(Color32::WHITE),
                )
                .on_hover_ui(|ui| {
                    ui.label(t!("New entry"));
                })
                .clicked()
            {
                if v.main_state.is_bundles() {
                    controller.set_action(Action::StartAddBundle);
                } else {
                    controller.set_action(Action::StartAddDocument);
                }
            }

            ui.add_space(10.);
            ui.separator();
            ui.add_space(10.);

            let search_pattern_response = ui.add_enabled(
                v.main_state.tabs_and_create_ok(),
                TextEdit::singleline(&mut v.find.pattern)
                    .desired_width(SEARCH_TEXT_WIDTH)
                    .hint_text(format!("🔍 {}", t!("_find"))),
            );
            if v.find.request_focus {
                search_pattern_response.request_focus();
                v.find.request_focus = false;
            }
            if search_pattern_response.changed() {
                controller.set_action(Action::StartFilter);
            }

            if !v.find.pattern.is_empty() {
                ui.add_space(-27.);
                if ui
                    .add(
                        Button::image(IMG_ERASE)
                            .fill(Color32::WHITE)
                            .small()
                            .frame(false),
                    )
                    .clicked()
                {
                    v.find.pattern.clear();
                    controller.set_action(Action::StartFilter);
                }

                if ui
                    .add(
                        Slider::new(
                            &mut v.find.threshold,
                            Find::MIN_THRESHOLD..=Find::MAX_THRESHOLD,
                        )
                        .clamping(SliderClamping::Always)
                        .show_value(false)
                        .step_by(10_f64),
                    )
                    .changed()
                {
                    controller.set_action(Action::StartFilter);
                }

                ui.label("🔎");
            }
        });
        ui.add_space(4.);
    });
}
