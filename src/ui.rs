mod active_bundle;
mod buttons;
mod inactive_bundle;
mod v;

use super::PlFile;
use crate::sizes::{
    BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT, EGUI_DEFAULT_SPACE,
    SEARCH_TEXT_WIDTH, WIN_WIDTH,
};
use v::{VBundle, VEditBundle, VNamedSecret, V};

use eframe::{App, Frame};
use egui::{
    include_image, scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, Context,
    FontFamily, Image, RichText, ScrollArea, TextEdit, Theme, TopBottomPanel,
};
use egui_extras::{Size, StripBuilder};

/* TODOs *************************************

Backlog:
- Header visuell abheben
- Suchfeld, um Einträge schnell finden zu können
    - Konkretere Beispiele erzeugen
    - sehr viele Beispiele erzeugen

- Drei-Punkt Menu rechts oben
    - Passwort ändern…
    - Über prolock…
    - Druck-Option: alles als Text serialisieren und anzeigen, den man drucken kann

- Passwort-Handling
    - beim ersten Start: Abfrage erst, wenn Daten erfasst wurden und gespeichert werden soll
    - beim Start mit existierendem File direkt abfragen, dann normales Fenster anzeigen
    - Passwort ändern (-> Menü-Eintrag?)
    - demo für Modals könnte interessant sein

- Änderungsprozess:
    - Nach Start ist alles sichtbar bzw auf Wunsch lesbar, aber nicht änderbar
    - Neuen Eintrag hinzufügen
    - Bestehenden Eintrag ändern
    - Bestehenden Eintrag löschen

- About + Hilfe
- Mehrsprachigkeit?

******************************************* */

pub struct UiApp {
    v: V,
    pl_file: PlFile,
}
impl UiApp {
    pub fn new(pl_file: PlFile) -> Self {
        let v = V {
            search: String::new(),
            edit_idx: None,
            bundles: pl_file
                .bundles()
                .map(|(name, bundle)| VBundle {
                    name: name.to_string(),
                    description: bundle.description.clone(),
                    v_named_secrets: bundle
                        .named_secrets
                        .iter()
                        .map(|(name, secret)| VNamedSecret {
                            name: name.clone(),
                            secret: secret.disclose(pl_file.o_transient.as_ref().unwrap()),
                            show_secret: false,
                            copied_at: None,
                        })
                        .collect(),
                })
                .collect(),
            edit_bundle: VEditBundle::default(),
        };
        UiApp { v, pl_file }
    }
}

impl App for UiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // header
        let (color_user, color_secret) = match ctx.theme() {
            Theme::Dark => (Color32::LIGHT_BLUE, Color32::LIGHT_RED),
            Theme::Light => (Color32::DARK_BLUE, Color32::DARK_RED),
        };

        TopBottomPanel::top("file").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::LIGHT_GRAY,
                    RichText::new(self.pl_file.file_path.display().to_string())
                        .family(FontFamily::Monospace),
                );
                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(10.);

                ui.label(format!(
                    "{} entries with {} secrets",
                    self.pl_file.stored.readable.bundles.len(),
                    self.pl_file.stored.readable.bundles.count_secrets(),
                ));

                ui.add_space(ui.available_width() - 80.);

                // TODO: Drei-Punkt Menu rechts oben
                ui.add(Image::new(include_image!("assets/burger.png")));
            });

            ui.add_space(10.);
        });

        TopBottomPanel::top("buttons").show(ctx, |ui| {
            ui.add_space(4.);
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        self.v.edit_idx.is_none(),
                        Button::image(
                            Image::new(if self.v.edit_idx.is_none() {
                                include_image!("./assets/add_entry.png")
                            } else {
                                include_image!("./assets/add_entry inactive.png")
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
                    //
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
                            Image::new(include_image!("assets/search.png"))
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

        // bundles
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    StripBuilder::new(ui)
                        .sizes(Size::exact(BUNDLE_HEIGHT), self.v.bundles.len())
                        .vertical(|mut bundle_strip| {
                            for (index, v_bundle) in &mut self.v.bundles.iter_mut().enumerate() {
                                bundle_strip.strip(|bundle_builder| {
                                    visualize_bundle(
                                        &mut self.pl_file,
                                        &mut self.v.edit_bundle,
                                        ctx,
                                        color_user,
                                        color_secret,
                                        bundle_builder,
                                        index,
                                        v_bundle,
                                        &mut self.v.edit_idx,
                                    );
                                });
                            }
                        });
                })
        });
    }
}

fn visualize_bundle(
    pl_file: &mut PlFile,
    v_edit_bundle: &mut VEditBundle,
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    bundle_builder: StripBuilder<'_>,
    index: usize,
    v_bundle: &mut VBundle,
    edit_idx: &mut Option<usize>,
) {
    bundle_builder
        .size(Size::exact(BUNDLE_WIDTH_BUTTONS))
        .size(Size::exact(BUNDLE_WIDTH_LEFT))
        .size(Size::exact(BUNDLE_WIDTH_RIGHT))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                buttons::show_bundle_buttons(v_edit_bundle, pl_file, index, edit_idx, ui);
            });

            if Some(index) == *edit_idx {
                active_bundle::show(
                    ctx,
                    color_user,
                    color_secret,
                    index,
                    v_edit_bundle,
                    inner_bundle_strip,
                );
            } else {
                inactive_bundle::show(
                    ctx,
                    color_user,
                    color_secret,
                    index,
                    v_bundle,
                    inner_bundle_strip,
                );
            }
        });
}
