mod buttons;
mod central_panel;
mod edit;
mod show;

pub use buttons::{
    active_buttons_edit_and_delete, active_buttons_save_and_cancel,
    inactive_buttons_edit_and_delete,
};
pub use central_panel::central_panel;
pub use edit::edit;
pub use show::show;
