#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(dead_code)] // FIXME

#[macro_use]
extern crate serde;

pub mod bundle;
pub mod bundles;
pub mod pl_file;
pub mod secrets;
pub mod transient;

use bundle::Bundle;
use pl_file::PlFile;

fn main() {
    // evaluate args

    let mut pl_file = PlFile::open().expect("File open error");

    //vv temp section (to get rid of "unused" messages vv//
    let o_storage_password: Option<String> = None;
    if let Some(storage_password) = o_storage_password {
        pl_file
            .set_password(storage_password)
            .expect("Wrong password");
    }

    let mut bundle = Bundle::new("bliblablub");
    bundle.add_cred("userx".to_string(), "passwordx".to_string());

    pl_file
        .add_bundle("test", bundle)
        .expect("add bundle failed");
    //^^ temp section (to get rid of "unused" messages ^^//

    // prepare ui

    // show ui

    // save
    pl_file.save().expect("save failed");
}
