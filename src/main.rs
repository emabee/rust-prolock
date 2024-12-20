#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(dead_code)] // FIXME

#[macro_use]
extern crate serde;

pub mod bundle;
pub mod bundles;
pub mod prv_file;
pub mod s_idx;
pub mod secrets;
pub mod sequence;
pub mod transient;

use bundle::Bundle;
use prv_file::PrvFile;

fn main() {
    // evaluate args

    let mut prv_file = PrvFile::open().expect("File open error");

    //vv temp section (to get rid of "unused" messages vv//
    let o_storage_password: Option<String> = None;
    if let Some(storage_password) = o_storage_password {
        prv_file
            .set_password(storage_password)
            .expect("Wrong password");
    }

    let mut bundle = Bundle::new("bliblablub");
    bundle.add_cred("userx".to_string(), "passwordx".to_string());

    prv_file
        .add_bundle("test", bundle)
        .expect("add bundle failed");

    //^^ temp section (to get rid of "unused" messages ^^//

    // prepare ui

    // show ui

    // save
    prv_file.save().expect("save failed");
}
