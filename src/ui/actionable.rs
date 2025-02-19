use super::{
    sizes::{
        BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT,
        EGUI_DEFAULT_SPACE, SEARCH_TEXT_WIDTH, WIN_WIDTH,
    },
    viz::{EditIdx, PlModal, VBundle, VEditBundle},
    Colors, Ui, IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_SEARCH,
};
use crate::PlFile;
use bundle_buttons::{
    active_buttons_edit_and_delete, active_buttons_save_and_cancel,
    inactive_buttons_edit_and_delete,
};
use egui::{
    scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, Context, Image, ScrollArea,
    TextEdit, TopBottomPanel,
};
use egui_extras::{Size, StripBuilder};

mod bundle_buttons;
pub(crate) mod edit_bundle;
mod show_bundle;

impl Ui {
    pub(super) fn panels_for_actionable_ui(&mut self, ctx: &Context) {
        self.top_panel_header(ctx);

        self.central_panel_bundles(ctx);
    }

    fn top_panel_header(&mut self, ctx: &Context) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(4.);
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        self.v.edit_idx.is_none(),
                        Button::image(
                            Image::new(if self.v.edit_idx.is_none() {
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
                    self.v.pl_modal = PlModal::CreateBundle;
                    self.v.edit_bundle.clear();
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
                ui.add(TextEdit::singleline(&mut self.v.search).desired_width(SEARCH_TEXT_WIDTH));
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

    fn central_panel_bundles(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    StripBuilder::new(ui)
                        .sizes(Size::exact(BUNDLE_HEIGHT), self.v.bundles.len())
                        .vertical(|mut bundle_strip| {
                            for (index, v_bundle) in &mut self.v.bundles.iter_mut().enumerate() {
                                let edit_idx = self.v.edit_idx;
                                if edit_idx.is_mod_with(index) {
                                    bundle_strip.strip(|bundle_builder| {
                                        edit_a_bundle_with_buttons(
                                            ctx,
                                            bundle_builder,
                                            &mut self.pl_file,
                                            &mut self.v.edit_idx,
                                            &mut self.v.edit_bundle,
                                            &mut self.v.need_refresh,
                                            &self.colors,
                                        );
                                    });
                                }
                                if edit_idx.is_none() || edit_idx.is_mod_not_with(index) {
                                    bundle_strip.strip(|bundle_builder| {
                                        show_a_bundle_with_buttons(
                                            ctx,
                                            bundle_builder,
                                            &mut self.pl_file,
                                            index,
                                            v_bundle,
                                            &mut self.v.edit_idx,
                                            &mut self.v.edit_bundle,
                                            &mut self.v.need_refresh,
                                            &self.colors,
                                        );
                                    });
                                }
                            }
                        });
                })
        });
    }
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
    pl_file: &mut PlFile,
    index: usize,
    v_bundle: &mut VBundle,
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
                if edit_idx.is_none() {
                    active_buttons_edit_and_delete(
                        ui,
                        pl_file,
                        index,
                        v_bundle,
                        edit_idx,
                        edit_bundle,
                        need_refresh,
                    );
                } else {
                    inactive_buttons_edit_and_delete(ui);
                }
            });
            show_bundle::ui(ctx, colors, index, v_bundle, &mut inner_bundle_strip);
        });
}
