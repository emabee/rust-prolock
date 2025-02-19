use egui::{include_image, ImageSource};

// Provide unique access to the images in this folder
macro_rules! img {
    ($img_name:ident, $img_file:literal) => {
        pub(super) const $img_name: ImageSource = include_image!($img_file);
    };
}

img!(IMG_ADD_ENTRY_INACTIVE, "add_entry inactive.png");
img!(IMG_ADD_ENTRY, "add_entry.png");
img!(IMG_BURGER, "burger.png");
img!(IMG_CANCEL, "cancel.png");
img!(IMG_DELETE_INACTIVE, "delete inactive.png");
img!(IMG_DELETE, "delete.png");
img!(IMG_EDIT_INACTIVE, "edit inactive.png");
img!(IMG_EDIT, "edit.png");
img!(IMG_LOGO, "logo.png");
img!(IMG_OK, "ok.png");
img!(IMG_SAVE, "save.png");
img!(IMG_SEARCH, "search.png");
