use crate::ui::sizes::MODAL_WIDTH;
use egui::{Align, Context, Pos2, ScrollArea, TextEdit, Window};
use egui_extras::{Size, StripBuilder};
use flexi_logger::{LoggerHandle, Snapshot};

pub fn show_log(
    logger_handle: &LoggerHandle,
    logger_snapshot: &mut Snapshot,
    open: &mut bool,
    ctx: &Context,
) {
    let updated = logger_handle.update_snapshot(logger_snapshot).unwrap();
    let text = &mut logger_snapshot.text;
    Window::new(t!("Action log"))
        .default_pos(Pos2 { x: 10_000., y: 0. })
        .open(open)
        .show(ctx, |ui| {
            ui.set_width(MODAL_WIDTH);
            let text1 = text;
            let height = 200.;
            ui.add_space(5.);

            StripBuilder::new(ui)
                .size(Size::exact(height))
                .vertical(|mut log_strip| {
                    log_strip.cell(|ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.add_sized(
                                [MODAL_WIDTH, height],
                                TextEdit::multiline(text1).interactive(true),
                            );
                            if updated {
                                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            }
                        });
                    });
                });

            ui.add_space(5.);
        });
}
