use egui::{
    Color32, Context, FontFamily, FontId, Pos2, ScrollArea, TextFormat, Window,
    scroll_area::ScrollBarVisibility,
    text::{LayoutJob, TextWrapping},
};
use flexi_logger::{LoggerHandle, Snapshot};

pub fn show_log(
    logger_handle: &LoggerHandle,
    logger_snapshot: &mut Snapshot,
    open: &mut bool,
    ctx: &Context,
) {
    logger_handle.update_snapshot(logger_snapshot).unwrap();

    Window::new(t!("Action log"))
        .default_pos(Pos2 { x: 800., y: 200. })
        .default_height(400.)
        .default_width(600.)
        .resizable(true)
        .open(open)
        .show(ctx, |ui| {
            ScrollArea::both()
                .id_salt("action log")
                .auto_shrink(false)
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .max_width(f32::INFINITY)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for line in logger_snapshot.text.lines() {
                        let mut job = LayoutJob {
                            wrap: TextWrapping::truncate_at_width(ui.available_width()),
                            ..Default::default()
                        };
                        job.append(
                            line,
                            0.0,
                            TextFormat {
                                font_id: FontId::new(11.0, FontFamily::Monospace),
                                color: match line.chars().next() {
                                    Some('E') => Color32::RED,
                                    Some('W') => Color32::YELLOW,
                                    Some('I') => Color32::DARK_GRAY,
                                    Some('D') => Color32::from_gray(120), // medium gray
                                    Some('T') => Color32::from_gray(150), // lighter gray
                                    _ => Color32::PURPLE,
                                },
                                ..Default::default()
                            },
                        );
                        ui.label(job);
                    }
                });
        });
}
