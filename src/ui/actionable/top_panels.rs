use crate::{
    data::Documents,
    ui::{
        IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_ERASE,
        controller::{Action, Controller},
        sizes::SEARCH_TEXT_WIDTH,
        viz::{BundleState, DocId, DocumentState, MainState, V},
    },
};
use egui::{Button, Color32, Context, Image, RichText, TextEdit, TopBottomPanel};

pub(super) fn panel_with_tabs(
    v: &mut V,
    documents: &Documents,
    controller: &mut Controller,
    ctx: &Context,
) {
    // two tabs: Bundles and Documents
    TopBottomPanel::top("panel_with_tabs").show(ctx, |ui| {
        ui.add_space(10.);
        ui.horizontal(|ui| {
            ui.add_space(14.);
            if ui
                .add(
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
                .add(
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
                        Some(DocId(0, documents.iter().next().unwrap().0.to_string()))
                    }));
                controller.set_action(Action::StartFilter);
            }
        });
        ui.add_space(-12.);
    });
}

pub(super) fn panel_with_create_and_filter(v: &mut V, controller: &mut Controller, ctx: &Context) {
    TopBottomPanel::top("header").show(ctx, |ui| {
        ui.add_space(16.);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    v.modal_state.no_modal_is_open(),
                    Button::image(
                        Image::new(if v.modal_state.no_modal_is_open() {
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

            ui.add_space(2.);
            let response = ui.add(
                TextEdit::singleline(&mut v.find.pattern)
                    .desired_width(SEARCH_TEXT_WIDTH)
                    .hint_text(format!("🔍 {}", t!("_find"))),
            );
            if v.find.request_focus {
                response.request_focus();
                v.find.request_focus = false;
            }
            if response.changed() {
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
            }
        });
        ui.add_space(4.);
    });
}
