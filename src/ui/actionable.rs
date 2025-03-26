mod bundle_buttons;
pub mod edit_bundle;
mod show_bundle;

use crate::{
    controller::{Action, Controller},
    data::{Bundle, Bundles, Transient},
    ui::{
        IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_ERASE,
        sizes::{
            BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT,
            SEARCH_TEXT_WIDTH,
        },
        viz::{Edit, V, VBundle},
    },
};
use bundle_buttons::{
    active_buttons_edit_and_delete, active_buttons_save_and_cancel,
    inactive_buttons_edit_and_delete,
};
use egui::{
    Button, CentralPanel, Color32, Context, Image, RichText, ScrollArea, TextEdit, TopBottomPanel,
    scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};

pub(super) fn panels_for_actionable_ui(
    bundles: &Bundles,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    top_panel_header(v, controller, ctx);

    central_panel_bundles(bundles, transient, v, controller, ctx);
}

fn top_panel_header(v: &mut V, controller: &mut Controller, ctx: &Context) {
    TopBottomPanel::top("header").show(ctx, |ui| {
        ui.add_space(4.);
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    v.edit.idx.is_none(),
                    Button::image(
                        Image::new(if v.edit.idx.is_none() {
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
                controller.set_action(Action::StartAdd);
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

fn central_panel_bundles(
    bundles: &Bundles,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    CentralPanel::default().show(ctx, |ui| {
        if bundles.is_empty() {
            ui.horizontal(|ui| {
                ui.label(RichText::from("⬆ ").color(Color32::DARK_GRAY).size(22.));
                ui.label(
                    RichText::from(t!("Press this button to create an entry"))
                        .color(Color32::DARK_GRAY)
                        .size(16.)
                        .italics(),
                );
            });
        } else {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    StripBuilder::new(ui)
                        .sizes(Size::exact(BUNDLE_HEIGHT), usize::max(1, v.visible_len()))
                        .vertical(|mut bundle_strip| {
                            for (index, (name, bundle)) in bundles.iter().enumerate() {
                                let v_bundle = &mut v.bundles[index];
                                if !v_bundle.suppressed {
                                    if v.edit.idx == Some(index) {
                                        bundle_strip.strip(|bundle_builder| {
                                            edit_a_bundle_with_buttons(
                                                bundle_builder,
                                                &mut v.edit,
                                                controller,
                                            );
                                        });
                                    } else {
                                        bundle_strip.strip(|bundle_builder| {
                                            show_a_bundle_with_buttons(
                                                ctx,
                                                bundle_builder,
                                                index,
                                                bundle,
                                                v_bundle,
                                                name,
                                                transient,
                                                v.edit.idx,
                                                controller,
                                            );
                                        });
                                    }
                                }
                            }
                        });
                });
        }
    });
}

fn edit_a_bundle_with_buttons(
    bundle_builder: StripBuilder<'_>,
    edit: &mut Edit,
    controller: &mut Controller,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                active_buttons_save_and_cancel(ui, controller);
            });
            edit_bundle::ui(edit, &mut inner_bundle_strip);
        });
}

#[allow(clippy::too_many_arguments)]
fn show_a_bundle_with_buttons(
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    index: usize,
    bundle: &Bundle,
    v_bundle: &mut VBundle,
    name: &str,
    transient: &Transient,
    edit_idx: Option<usize>,
    controller: &mut Controller,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                if edit_idx.is_none() {
                    active_buttons_edit_and_delete(ui, index, name, controller);
                } else {
                    inactive_buttons_edit_and_delete(ui);
                }
            });
            show_bundle::ui(
                ctx,
                index,
                bundle,
                v_bundle,
                name,
                transient,
                &mut inner_bundle_strip,
            );
        });
}
