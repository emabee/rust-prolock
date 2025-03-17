mod bundle_buttons;
pub mod edit_bundle;
mod show_bundle;

use crate::{
    controller::{Action, Controller},
    data::{Bundle, Bundles, Transient},
    ui::{
        IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_SEARCH,
        sizes::{
            BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT,
            EGUI_DEFAULT_SPACE, SEARCH_TEXT_WIDTH, WIN_WIDTH,
        },
        viz::{V, VBundle, VEditBundle},
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
                    v.edit_idx.is_none(),
                    Button::image(
                        Image::new(if v.edit_idx.is_none() {
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

            ui.add_space(
                WIN_WIDTH
                    - 4.
                    - SEARCH_TEXT_WIDTH
                    - 16.
                    - (2. * EGUI_DEFAULT_SPACE)
                    - (2. * 26.)
                    - 58.,
            );
            ui.add(TextEdit::singleline(&mut v.search).desired_width(SEARCH_TEXT_WIDTH));
            if ui
                .add(
                    Button::image(
                        Image::new(IMG_SEARCH)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(0.22),
                    )
                    .fill(Color32::WHITE),
                )
                .clicked()
            {
                //
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
                        .sizes(Size::exact(BUNDLE_HEIGHT), usize::max(1, bundles.len()))
                        .vertical(|mut bundle_strip| {
                            for (index, (name, bundle)) in bundles.iter().enumerate() {
                                if v.edit_idx == Some(index) {
                                    bundle_strip.strip(|bundle_builder| {
                                        edit_a_bundle_with_buttons(
                                            bundle_builder,
                                            &mut v.edit_bundle,
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
                                            &mut v.bundles[index],
                                            name,
                                            transient,
                                            v.edit_idx,
                                            controller,
                                        );
                                    });
                                }
                            }
                        });
                });
        }
    });
}

fn edit_a_bundle_with_buttons(
    bundle_builder: StripBuilder<'_>,
    edit_bundle: &mut VEditBundle,
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
            edit_bundle::ui(edit_bundle, &mut inner_bundle_strip);
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
