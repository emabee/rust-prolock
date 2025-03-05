mod bundle_buttons;
pub mod edit_bundle;
mod show_bundle;

use crate::{
    PlFile,
    ui::{
        Colors, IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_SEARCH,
        sizes::{
            BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT,
            EGUI_DEFAULT_SPACE, SEARCH_TEXT_WIDTH, WIN_WIDTH,
        },
        viz::{EditIdx, PlModal, V, VBundle, VEditBundle},
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
    pl_file: &mut PlFile,
    v: &mut V,
    colors: &Colors,
    ctx: &Context,
) {
    top_panel_header(v, ctx);

    central_panel_bundles(pl_file, v, colors, ctx);
}

fn top_panel_header(v: &mut V, ctx: &Context) {
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
                v.pl_modal = PlModal::CreateBundle;
                v.edit_bundle.prepare_for_create();
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

fn central_panel_bundles(pl_file: &mut PlFile, v: &mut V, colors: &Colors, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if v.bundles.is_empty() {
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
                        .sizes(Size::exact(BUNDLE_HEIGHT), usize::max(1, v.bundles.len()))
                        .vertical(|mut bundle_strip| {
                            for (index, v_bundle) in &mut v.bundles.iter_mut().enumerate() {
                                let edit_idx = v.edit_idx;
                                if edit_idx.is_mod_with(index) {
                                    bundle_strip.strip(|bundle_builder| {
                                        edit_a_bundle_with_buttons(
                                            ctx,
                                            bundle_builder,
                                            pl_file,
                                            &mut v.edit_idx,
                                            &mut v.edit_bundle,
                                            &mut v.need_refresh,
                                            colors,
                                        );
                                    });
                                }
                                if edit_idx.is_none() || edit_idx.is_mod_not_with(index) {
                                    bundle_strip.strip(|bundle_builder| {
                                        show_a_bundle_with_buttons(
                                            ctx,
                                            bundle_builder,
                                            &mut v.pl_modal,
                                            index,
                                            v_bundle,
                                            &mut v.edit_idx,
                                            &mut v.edit_bundle,
                                            colors,
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
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    pl_file: &mut PlFile,
    edit_idx: &mut EditIdx,
    edit_bundle: &mut VEditBundle,
    need_refresh: &mut bool,
    colors: &Colors,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                active_buttons_save_and_cancel(ui, pl_file, edit_bundle, edit_idx, need_refresh);
            });
            edit_bundle::ui(ctx, colors, edit_bundle, &mut inner_bundle_strip);
        });
}

#[allow(clippy::too_many_arguments)]
fn show_a_bundle_with_buttons(
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    pl_modal: &mut PlModal,
    index: usize,
    v_bundle: &mut VBundle,
    edit_idx: &mut EditIdx,
    edit_bundle: &mut VEditBundle,
    colors: &Colors,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                if edit_idx.is_none() {
                    active_buttons_edit_and_delete(
                        ui,
                        index,
                        pl_modal,
                        v_bundle,
                        edit_idx,
                        edit_bundle,
                    );
                } else {
                    inactive_buttons_edit_and_delete(ui);
                }
            });
            show_bundle::ui(ctx, colors, index, v_bundle, &mut inner_bundle_strip);
        });
}
