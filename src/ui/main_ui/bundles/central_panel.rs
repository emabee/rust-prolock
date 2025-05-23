use crate::{
    data::{Bundle, Bundles, Key, Transient},
    ui::{
        controller::Controller,
        main_ui::bundles::{
            active_buttons_edit_and_delete, active_buttons_save_and_cancel,
            inactive_buttons_edit_and_delete,
        },
        sizes::{BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT},
        viz::{BundleState, MainState, V, VBundle, VEditBundle},
    },
};
use egui::{
    CentralPanel, Color32, Context, RichText, ScrollArea, scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};

pub fn central_panel(
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
                        .sizes(
                            Size::exact(BUNDLE_HEIGHT),
                            usize::max(1, v.visible_bundles()),
                        )
                        .vertical(|mut bundle_strip| {
                            let mut alternate = false;
                            for (key, bundle) in bundles.iter() {
                                alternate = !alternate;
                                let v_bundle = v.bundles.get_mut(key).unwrap();
                                if !v_bundle.suppressed {
                                    let mut done = false;
                                    if let MainState::Bundles(BundleState::ModifyBundle {
                                        ref mut v_edit_bundle,
                                        ref error,
                                    }) = v.main_state
                                    {
                                        if *key == v_edit_bundle.orig_key {
                                            bundle_strip.strip(|bundle_builder| {
                                                edit_a_bundle_with_buttons(
                                                    bundle_builder,
                                                    v_edit_bundle,
                                                    error.as_deref(),
                                                    controller,
                                                );
                                            });
                                            done = true;
                                        }
                                    }
                                    if !done {
                                        bundle_strip.strip(|bundle_builder| {
                                            show_a_bundle_with_buttons(
                                                ctx,
                                                bundle_builder,
                                                bundle,
                                                v_bundle,
                                                key,
                                                alternate,
                                                transient,
                                                v.modal_state.no_modal_is_open(),
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
    v_edit_bundle: &mut VEditBundle,
    error: Option<&str>,
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
            super::edit(v_edit_bundle, error, &mut inner_bundle_strip);
        });
}

#[allow(clippy::too_many_arguments)]
fn show_a_bundle_with_buttons(
    ctx: &Context,
    bundle_builder: StripBuilder<'_>,
    bundle: &Bundle,
    v_bundle: &mut VBundle,
    key: &Key,
    alternate: bool,
    transient: &Transient,
    show_active_buttons: bool,
    controller: &mut Controller,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                if show_active_buttons {
                    active_buttons_edit_and_delete(ui, key, controller);
                } else {
                    inactive_buttons_edit_and_delete(ui);
                }
            });
            super::show_bundle(
                ctx,
                bundle,
                v_bundle,
                key,
                alternate,
                transient,
                &mut inner_bundle_strip,
            );
        });
}
