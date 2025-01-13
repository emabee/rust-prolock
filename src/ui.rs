use crate::{
    pl_file::PlFile,
    v_bundles::{NamedSecret, VBundle, VBundles},
};
use core::f32;
use eframe::{App, Frame};
use egui::{
    include_image, scroll_area::ScrollBarVisibility, Button, CentralPanel, Color32, Context,
    FontFamily, FontId, Image, Rgba, RichText, ScrollArea, TextEdit, Theme, TopBottomPanel,
};
use egui_extras::{Size, StripBuilder};

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
    save_modal_open: bool,
    save_progress: Option<f32>,
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
                    .map(|(name, secret)| NamedSecret {
                        name: name.clone(),
                        secret: secret.resolve(pl_file.o_transient.as_ref().unwrap()),
                        show_secret: false,
                        copied_at: None,
                    })
                    .collect(),
            })
            .collect();
        UiApp {
            v_bundles,
            pl_file,
            save_progress: None,
            save_modal_open: false,
        }
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
                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(10.);

                ui.label("stored/edited");

                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(250.);

                // TODO: Drei-Punkt Menu rechts oben
                ui.add(Image::new(include_image!("assets/burger.png")));
            });

            ui.add_space(10.);
        });

        // bundles
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    StripBuilder::new(ui)
                        .sizes(Size::exact(100.), self.v_bundles.len())
                        .vertical(|mut bundle_strip| {
                            for (index, v_bundle) in &mut self.v_bundles.iter_mut().enumerate() {
                                bundle_strip.strip(|bundle_builder| {
                                    show_bundle(
                                        ctx,
                                        color_user,
                                        color_secret,
                                        bundle_builder,
                                        index,
                                        v_bundle,
                                    );
                                });
                            }
                        });
                })
        });
    }
}

fn show_bundle(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    bundle_builder: StripBuilder<'_>,
    index: usize,
    v_bundle: &mut VBundle,
) {
    bundle_builder
        .size(Size::exact(20.))
        .size(Size::exact(400.))
        .size(Size::exact(500.))
        .horizontal(|mut inner_bundle_strip| {
            inner_bundle_strip.cell(|ui| {
                ui.add(
                    Button::image(
                        Image::new(include_image!("./assets/edit.png"))
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(0.5),
                    )
                    .fill(Color32::WHITE),
                );
            });
            inner_bundle_strip.strip(|left_builder| {
                show_left_bundle_part(index, v_bundle, left_builder);
            });
            inner_bundle_strip.strip(|right_builder| {
                show_right_bundle_part(
                    ctx,
                    color_user,
                    color_secret,
                    index,
                    v_bundle,
                    right_builder,
                );
            });
        });
}

fn show_right_bundle_part(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    index: usize,
    v_bundle: &mut VBundle,
    right_builder: StripBuilder<'_>,
) {
    right_builder
        .sizes(Size::exact(20.), v_bundle.named_secrets.len())
        .vertical(|mut right_strip| {
            for named_secret in &mut v_bundle.named_secrets {
                right_strip.strip(|cred_builder| {
                    show_cred(
                        ctx,
                        color_user,
                        color_secret,
                        index,
                        named_secret,
                        cred_builder,
                    );
                });
            }
        });
}

fn show_cred(
    ctx: &Context,
    color_user: Color32,
    color_secret: Color32,
    index: usize,
    named_secret: &mut NamedSecret,
    cred_builder: StripBuilder<'_>,
) {
    cred_builder
        .size(Size::exact(210.))
        .size(Size::exact(170.))
        .horizontal(|mut cred_strip| {
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                ui.add(
                    TextEdit::singleline(&mut named_secret.name.as_str())
                        .desired_width(200.)
                        .clip_text(true)
                        .text_color(color_user)
                        .interactive(true),
                );
            });
            cred_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                let response = ui
                    .add(
                        TextEdit::singleline(&mut named_secret.secret.as_str())
                            .desired_width(160.)
                            .clip_text(true)
                            .text_color(color_secret)
                            .password(!named_secret.show_secret)
                            .interactive(true),
                    )
                    .on_hover_ui(|ui| {
                        ui.style_mut().interaction.selectable_labels = true;
                        match named_secret.copied_at {
                            None => {
                                if ui
                                    .add(Button::new("  Copy").min_size([60., 10.].into()))
                                    .clicked()
                                {
                                    ctx.copy_text(named_secret.secret.clone());
                                    named_secret.copied_at = Some(std::time::Instant::now());
                                }
                            }
                            Some(instant) => {
                                ui.label("✔ Copied");
                                if std::time::Instant::now() - instant
                                    > std::time::Duration::from_millis(800)
                                {
                                    named_secret.copied_at = None;
                                }
                            }
                        }
                    });
                if response.hovered() {
                    named_secret.show_secret = true;
                } else {
                    named_secret.show_secret = false;
                };
            });
        });
}

fn show_left_bundle_part(index: usize, v_bundle: &mut VBundle, left_builder: StripBuilder<'_>) {
    left_builder
        .size(Size::exact(15.))
        .size(Size::exact(40.))
        .size(Size::exact(10.))
        .vertical(|mut left_strip| {
            //name
            left_strip.cell(|ui| {
                set_faded_bg_color(ui, 20., index);
                ui.add(
                    TextEdit::singleline(&mut v_bundle.name.as_str())
                        .desired_width(330.)
                        .clip_text(true)
                        .font(FontId {
                            size: 16.,
                            family: FontFamily::Proportional,
                        })
                        .interactive(true),
                );
            });

            // description
            left_strip.cell(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    set_faded_bg_color(ui, f32::INFINITY, index);
                    ui.add_sized(
                        [380., 80.],
                        TextEdit::multiline(&mut v_bundle.description.as_str()).interactive(true),
                    );
                });
            });
        });
}

fn set_faded_bg_color(ui: &mut egui::Ui, height: f32, index: usize) {
    let dark_mode = ui.visuals().dark_mode;
    let bg_color = ui.visuals().window_fill();
    let t = if index % 2 == 0 {
        if dark_mode {
            0.95
        } else {
            0.91
        }
    } else {
        if dark_mode {
            0.95
        } else {
            0.8
        }
    };

    let mut rect = ui.available_rect_before_wrap();
    rect.set_height(height);
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::lerp(Rgba::from(Color32::DARK_BLUE)..=Rgba::from(bg_color), t),
    );
}
