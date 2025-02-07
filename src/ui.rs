mod bundle_buttons;
mod edit_bundle;
mod show_bundle;
pub mod sizes;
mod v;

use super::PlFile;
use bundle_buttons::{
    active_buttons_edit_and_delete, active_buttons_save_and_cancel,
    inactive_buttons_edit_and_delete,
};
use sizes::{
    BUNDLE_HEIGHT, BUNDLE_WIDTH_BUTTONS, BUNDLE_WIDTH_LEFT, BUNDLE_WIDTH_RIGHT, EGUI_DEFAULT_SPACE,
    SEARCH_TEXT_WIDTH, WIN_WIDTH,
};
use v::{EditIdx, VBundle, VEditBundle, V};

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

pub struct Ui {
    v: V,
    pl_file: PlFile,
    colors: Colors,
}
pub struct Colors {
    pub user: Color32,
    pub secret: Color32,
}
impl Ui {
    pub fn new(pl_file: PlFile) -> Self {
        Ui {
            v: V::new(),
            pl_file,
            colors: Colors {
                user: Color32::DARK_BLUE,
                secret: Color32::DARK_RED,
            },
        }
    }
}

impl App for Ui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        (self.colors.user, self.colors.secret) = match ctx.theme() {
            Theme::Dark => (Color32::LIGHT_BLUE, Color32::LIGHT_RED),
            Theme::Light => (Color32::DARK_BLUE, Color32::DARK_RED),
        };
        if self.v.need_refresh {
            self.v.reset_bundles(
                &self.pl_file.stored.readable.bundles,
                self.pl_file.transient().unwrap(/*should never fail*/),
            );
            self.v.need_refresh = false;
        }

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
                ui.add(Image::new(include_image!("ui/assets/burger.png")));
            });

            ui.add_space(10.);
        });

        if self.pl_file.is_actionable() {
            self.actionable_ui(ctx);
        } else {
            self.ask_for_password(ctx);
        }
    }
}

impl Ui {
    fn ask_for_password(&mut self, ctx: &Context) {
        TopBottomPanel::top("pw error").show(ctx, |ui| {
            if let Some(e) = &self.v.pw.error {
                ui.label(RichText::new(e).color(Color32::RED));
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.pl_file.stored.readable.header.update_counter.peek() == Some(0) {
                // this is the first start, so ask twice
                ui.add_space(15.);
                ui.label(
                    RichText::new("Creating the prolock file")
                        .size(16.)
                        .color(Color32::DARK_BLUE),
                );
                ui.add_space(15.);
                ui.label("Specify the password to secure your prolock file:");
                ui.add_space(15.);
                ui.horizontal(|ui| {
                    ui.add_space(50.);
                    ui.add(TextEdit::singleline(&mut "Password:").desired_width(80.));
                    ui.add(
                        TextEdit::singleline(&mut self.v.pw.pw1)
                            .desired_width(120.)
                            .password(true),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_space(50.);
                    ui.add(TextEdit::singleline(&mut "Repeat:").desired_width(80.));
                    ui.add(
                        TextEdit::singleline(&mut self.v.pw.pw2)
                            .desired_width(120.)
                            .password(true),
                    );
                    if ui.button("OK").clicked() {
                        if self.v.pw.pw1 == self.v.pw.pw2 {
                            // try using the password
                            match self.pl_file.set_actionable(self.v.pw.pw1.clone()) {
                                Ok(()) => {
                                    self.v.pw.error = None;
                                    self.v.reset_bundles(
                                        &self.pl_file.stored.readable.bundles,
                                        self.pl_file.transient().unwrap(/*should never fail*/),
                                    );
                                }
                                Err(e) => {
                                    println!("{e:?}");
                                    self.v.pw.error = Some(format!("{e:?}"));
                                }
                            }
                        } else {
                            self.v.pw.error = Some("The passwords don't match".to_string());
                        }
                    }
                });
            } else {
                // ask once
                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut self.v.pw.pw1);
                    if ui.button("OK").clicked() {
                        // try using the password
                        match self.pl_file.set_actionable(self.v.pw.pw1.clone()) {
                            Ok(()) => {
                                self.v.pw.error = None;
                                self.v.reset_bundles(
                                    &self.pl_file.stored.readable.bundles,
                                    self.pl_file.transient().unwrap(/*should never fail*/),
                                );
                            }
                            Err(e) => {
                                self.v.pw.error = Some(format!("{e:?}"));
                            }
                        }
                    }
                });
            }
        });
    }

    fn actionable_ui(&mut self, ctx: &Context) {
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
                            self.v.bundles.len() + usize::from(self.v.edit_idx.is_new()),
                        )
                        .vertical(|mut bundle_strip| {
                            // loop over bundles: FIXME show new bundle if bundles is empty
                            if self.v.bundles.is_empty() && self.v.edit_idx == EditIdx::New(0) {
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
                                include_image!("./ui/assets/add_entry.png")
                            } else {
                                include_image!("./ui/assets/add_entry inactive.png")
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
                            Image::new(include_image!("./ui/assets/search.png"))
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
