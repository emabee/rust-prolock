use eframe::{App, Frame};
use egui::{
    Button, Color32, Context, FontFamily, FontId, Image, RichText, TextEdit, Theme, TopBottomPanel,
};

use crate::{
    pl_file::PlFile,
    v_bundles::{VBundle, VBundles},
};

/* TODOs *************************************

Änderungen
    Änderungen wie
    - Entry hinzufügen
    - Entry-Text ändern
    - Entry umbenennen
    - Entry löschen
    - Cred hinzufügen
    - Cred-Text ändern
    - Cred umbenennen
    - Cred löschen
    jederzeit zulassen, aber explizite Bestätigung einfordern:
    - bearbeitetes Entry farblich hervorheben, Verwerfen und Save Buttons einblenden
    - expliziter Lösch-Dialog

    -> wie detektiert man eine Änderung in einem Textfeld?

    action: Action;
    None => Normaler Modus
    Modified(n) => if n==current {
    in Arbeit
    } else {
    nicht änderbar
    }

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

- Icon entwerfen
- About + Hilfe
- Mehrsprachigkeit?

******************************************* */

pub struct UiApp {
    v_bundles: VBundles,
    //    edit_v_bundle: VBundle,
    pl_file: PlFile,
}
impl UiApp {
    pub fn new(pl_file: PlFile) -> Self {
        let v_bundles = pl_file
            .bundles()
            .map(|(name, bundle)| VBundle {
                name: name.to_string(),
                description: bundle.description.clone(),
                named_secrets: bundle
                    .named_secrets
                    .iter()
                    .map(|(name, secret)| {
                        (
                            name.clone(),
                            false,
                            secret.resolve(pl_file.o_transient.as_ref().unwrap()),
                        )
                    })
                    .collect(),
            })
            .collect();
        UiApp { v_bundles, pl_file }
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
                        .family(egui::FontFamily::Monospace),
                );
                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(10.);

                ui.label(format!(
                    "{} entries with {} secrets",
                    self.pl_file.stored.readable.bundles.len(),
                    self.pl_file.stored.readable.bundles.count_secrets(),
                ));
                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(10.);

                ui.label("stored/edited");

                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(250.);

                // TODO: Drei-Punkt Menu rechts oben
                ui.add(egui::Image::new(egui::include_image!(
                    "assets/three_dots.png"
                )));
            });

            ui.add_space(10.);
        });

        // bundles
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                for v_bundle in &mut self.v_bundles {
                    ui.add_enabled_ui(true, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.button("save");
                            });
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut v_bundle.name.as_str())
                                            .desired_width(300.)
                                            .font(FontId {
                                                size: 16.,
                                                family: FontFamily::Proportional,
                                            })
                                            .interactive(true),
                                    );
                                });
                                ui.add(
                                    TextEdit::multiline(&mut v_bundle.description.as_str())
                                        .desired_width(400.)
                                        .interactive(true),
                                );
                            });
                            ui.add_space(10.);
                            ui.vertical(|ui| {
                                // secrets
                                for (description, show_secret, secret) in
                                    &mut v_bundle.named_secrets
                                {
                                    ui.horizontal(|ui| {
                                        ui.add(
                                            TextEdit::singleline(description)
                                                .desired_width(200.)
                                                // .min_size((100., 5.).into())
                                                .text_color(color_user)
                                                .interactive(true),
                                        );
                                        ui.add(
                                            TextEdit::singleline(&mut secret.as_str())
                                                .desired_width(160.)
                                                .text_color(color_secret)
                                                .password(!*show_secret)
                                                .interactive(true),
                                        );

                                        if ui
                                            .add(Button::image(Image::new(egui::include_image!(
                                                "assets/copy.png"
                                            ))))
                                            .on_hover_ui(|ui| {
                                                ui.label("Copy the secret");
                                            })
                                            .clicked()
                                        {
                                            ctx.copy_text(secret.to_string());
                                        }
                                        // 📋 = \u{1F4CB}
                                        // 👓 = \u{1F453}
                                        // 👁 = \u{1F441}
                                        // 🤫, 🔐
                                        if ui
                                            .add(Button::new(
                                                RichText::new(if *show_secret {
                                                    "🔐"
                                                } else {
                                                    "👁"
                                                })
                                                .color(color_secret)
                                                .strong(),
                                            ))
                                            .on_hover_ui(|ui| {
                                                ui.label(if *show_secret {
                                                    "Hide the secret"
                                                } else {
                                                    "Reveal the secret"
                                                });
                                            })
                                            .clicked()
                                        {
                                            *show_secret = !*show_secret;
                                        }
                                    });
                                    ui.add_space(3.);
                                }
                            });
                        });
                    });
                    // ui.add_space(10.);
                    ui.separator();
                }
            });
        });
    }
}
