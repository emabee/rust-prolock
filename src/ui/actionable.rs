mod bundle_buttons;
mod edit_bundle;
mod show_bundle;

use super::{
    sizes::{
        BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT,
        EGUI_DEFAULT_SPACE, SEARCH_TEXT_WIDTH, WIN_WIDTH,
    },
    viz::{EditIdx, VBundle, VEditBundle},
    Colors, Ui,
};
use crate::PlFile;
use bundle_buttons::{
    active_buttons_edit_and_delete, active_buttons_save_and_cancel,
    inactive_buttons_edit_and_delete,
};
use egui::{
    include_image, scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, Context, Image,
    ScrollArea, TextEdit, TopBottomPanel,
};
use egui_extras::{Size, StripBuilder};

impl Ui {
    pub(super) fn panels_for_actionable_ui(&mut self, ctx: &Context) {
        self.top_panel_header(ctx);

        self.central_panel_bundles(ctx);
    }

    fn central_panel_bundles(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    StripBuilder::new(ui)
                        .sizes(
                            Size::exact(BUNDLE_HEIGHT),
                            usize::max(self.v.bundles.len(), 1)
                                + usize::from(self.v.edit_idx.is_new()),
                        )
                        .vertical(|mut bundle_strip| {
                            if self.v.bundles.is_empty() {
                                self.v.edit_idx = EditIdx::New(0);
                                bundle_strip.strip(|bundle_builder| {
                                    edit_a_bundle_with_buttons(
                                        ctx,
                                        bundle_builder,
                                        &mut self.pl_file,
                                        &mut self.v.edit_idx,
                                        &mut self.v.need_refresh,
                                        &mut self.v.edit_bundle,
                                        &self.colors,
                                    );
                                });
                            } else {
                                for (index, v_bundle) in &mut self.v.bundles.iter_mut().enumerate()
                                {
                                    bundle_strip.strip(|bundle_builder| {
                                        let edit = match self.v.edit_idx {
                                            EditIdx::None => false,
                                            EditIdx::Mod(idx) | EditIdx::New(idx) => idx == index,
                                        };

                                        if edit {
                                            edit_a_bundle_with_buttons(
                                                ctx,
                                                bundle_builder,
                                                &mut self.pl_file,
                                                &mut self.v.edit_idx,
                                                &mut self.v.need_refresh,
                                                &mut self.v.edit_bundle,
                                                &self.colors,
                                            );
                                        } else {
                                            show_a_bundle_with_buttons(
                                                ctx,
                                                bundle_builder,
                                                index,
                                                v_bundle,
                                                &mut self.v.edit_idx,
                                                &mut self.v.edit_bundle,
                                                &self.colors,
                                            );
                                        }
                                    });
                                }
                            }
                        });
                })
        });
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
                                include_image!("./../ui/assets/add_entry.png")
                            } else {
                                include_image!("./../ui/assets/add_entry inactive.png")
                            })
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(0.22),
                        )
                        .fill(Color32::WHITE),
                    )
                    .on_hover_ui(|ui| {
                        ui.label("New entry");
                    })
                    .clicked()
                {
                    // TODO use index that is currently visible
                    self.v.edit_idx = EditIdx::New(0);
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
                            Image::new(include_image!("./../ui/assets/search.png"))
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
}

fn edit_a_bundle_with_buttons(
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    pl_file: &mut PlFile,
    edit_idx: &mut EditIdx,
    need_refresh: &mut bool,
    v_edit_bundle: &mut VEditBundle,
    colors: &Colors,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                active_buttons_save_and_cancel(pl_file, v_edit_bundle, edit_idx, need_refresh, ui);
            });
            edit_bundle::ui(ctx, colors, v_edit_bundle, &mut inner_bundle_strip);
        });
}

fn show_a_bundle_with_buttons(
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    index: usize,
    v_bundle: &mut VBundle,
    edit_idx: &mut EditIdx,
    v_edit_bundle: &mut VEditBundle,
    colors: &Colors,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                if edit_idx.is_none() {
                    active_buttons_edit_and_delete(index, v_bundle, edit_idx, v_edit_bundle, ui);
                } else {
                    inactive_buttons_edit_and_delete(ui);
                }
            });
            show_bundle::ui(ctx, colors, index, v_bundle, &mut inner_bundle_strip);
        });
}
