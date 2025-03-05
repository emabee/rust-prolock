use crate::data::{Bundle, Bundles, FileList, Secret, Secrets, Transient};
use anyhow::{Context, Result, anyhow};
use fd_lock::RwLock as FdRwLock;
use oxilangtag::LanguageTag;
use rust_i18n::t;
use sequential::Sequence;
use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{Read, Write as _},
    path::{Path, PathBuf},
};

const CURRENT_FORMAT_VERSION: u8 = 0;
const DEFAULT_LOCALE: &str = "en";
const PREFACE: &str = "\
# DO NOT EDIT THIS FILE
#
# For security reasons, prolock verifies the integrity of the readable section
# of this file when decrypting the value of 'secret'.
# Editing any part of this file renders the file unusable for prolock!

";

// Describes the status and content of the prolock file
#[derive(Clone, Debug)]
pub struct PlFile {
    pub file_path: PathBuf,
    pub stored: Stored,
    o_transient: Option<Transient>,
}

// This is the structure that is serialized to the file (after the preface);
// it consists of a readable section and an encrypted section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stored {
    pub readable: Readable,
    pub cipher: String,
}

// The readable section is written in clear and also used as auth-tag for the encrypted section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Readable {
    pub header: FileHeader,
    pub bundles: Bundles,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct FileHeader {
    pub format_version: u8,
    pub language: String,
    pub update_counter: Sequence<usize>,
}

impl PlFile {
    pub fn read_or_create(file_path: &Path) -> Result<Self> {
        if file_path.exists() {
            let result = Self::lock_and_read(file_path)?.1;
            rust_i18n::set_locale(&result.stored.readable.header.language);
            Ok(result)
        } else {
            // first start: ensure the folder exists, and start with initial PlFile
            create_dir_all(
                file_path
                    .parent()
                    .context("cannot determine folder for storage")?,
            )?;

            let locale = sys_locale::get_locale().unwrap_or(DEFAULT_LOCALE.to_string());
            let language = LanguageTag::parse(locale)
                .unwrap_or_else(|_e| LanguageTag::parse(DEFAULT_LOCALE.to_string()).unwrap(/*OK*/))
                .primary_language()
                .to_string();
            rust_i18n::set_locale(&language);
            rust_i18n::i18n!("locales", fallback = "en");

            Ok(Self {
                file_path: file_path.to_path_buf(),
                o_transient: None,
                stored: Stored {
                    readable: Readable {
                        header: FileHeader {
                            update_counter: Sequence::new(),
                            language,
                            format_version: CURRENT_FORMAT_VERSION,
                        },
                        bundles: Bundles::new(),
                    },
                    cipher: String::new(),
                },
            })
        }
    }

    fn lock_and_create_empty(file_path: &Path) -> Result<FdRwLock<File>> {
        Ok(FdRwLock::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(file_path)
                .context("opening file")?,
        ))
    }

    fn lock_and_read(file_path: &Path) -> Result<(FdRwLock<File>, PlFile)> {
        {
            let file = File::open(file_path).context(t!("opening file"))?;
            let mut file_lock = FdRwLock::new(file);
            let stored = Self::read_stored(&mut file_lock, file_path)?;
            Ok((
                file_lock,
                Self {
                    file_path: file_path.to_path_buf(),
                    o_transient: None,
                    stored,
                },
            ))
        }
    }

    fn read_stored(file_lock: &mut FdRwLock<File>, file_path: &Path) -> Result<Stored> {
        let mut file_content = String::with_capacity(1024);
        let mut write_guard = file_lock
            .write()
            .context(format!("locking {}", file_path.display()))?;
        (*write_guard)
            .read_to_string(&mut file_content)
            .context(format!("reading {}", file_path.display()))?;
        let semantic_content = skip_over_comments_and_empty_lines(&file_content);
        serde_json::from_str(semantic_content).context(t!("parsing PlFile"))
    }

    // #[cfg(test)]
    // pub fn len(&self) -> usize {
    //     self.stored.readable.bundles.len()
    // }
    pub fn is_empty(&self) -> bool {
        self.stored.readable.bundles.is_empty()
    }

    pub fn transient(&self) -> Option<&Transient> {
        self.o_transient.as_ref()
    }
    // #[cfg(test)]
    // pub fn bundles(&self) -> Iter<'_, String, Bundle> {
    //     self.stored.readable.bundles.into_iter()
    // }
    pub fn has_bundle(&self, name: &str) -> bool {
        self.stored.readable.bundles.contains_key(name)
    }

    pub fn is_actionable(&self) -> bool {
        self.o_transient.is_some()
    }
    pub fn set_actionable(&mut self, password: String) -> Result<()> {
        if self.stored.cipher.is_empty() {
            self.o_transient = Some(Transient::new(password, Secrets::default()));
            self.save()?;
        } else {
            self.o_transient = Some(
                Transient::from_cipher(password, &self.stored.readable, &self.stored.cipher)
                    .context(t!("Password not correct"))?,
            );
        }

        Ok(())
    }

    pub fn language(&self) -> &str {
        &self.stored.readable.header.language
    }

    pub fn set_language(&mut self, new_lang: &str) -> Result<()> {
        if self.stored.readable.header.language != new_lang {
            self.stored.readable.header.language = new_lang.to_string();
            self.save()?;
            rust_i18n::set_locale(new_lang);
        }
        Ok(())
    }

    pub fn change_password(&mut self, old_pw: &str, new_pw: String) -> Result<()> {
        self.check_password(old_pw)?;
        if let Some(ref mut transient) = self.o_transient {
            transient.set_storage_password(new_pw);
            self.save()?;
        }
        Ok(())
    }

    pub fn check_password(&mut self, old_pw: &str) -> Result<()> {
        if self.transient().unwrap(/*ok*/).get_storage_password() == old_pw {
            Ok(())
        } else {
            Err(anyhow!(
                t!("The current password is not correct").to_string()
            ))
        }
    }

    fn add_bundle<S>(&mut self, key: S, bundle: Bundle) -> Result<()>
    where
        S: AsRef<str>,
    {
        if self.stored.readable.bundles.contains_key(key.as_ref()) {
            Err(anyhow!(t!(
                "add_bundle: bundle %{b} exists already",
                b = key.as_ref()
            )))
        } else {
            match self.o_transient {
                None => Err(anyhow!(t!("password_not_available"))),
                Some(ref mut transient) => self.stored.readable.bundles.add(key, bundle, transient),
            }
        }
    }

    // #[cfg(test)]
    // pub fn get_bundle(&self, key: &str) -> Result<Bundle> {
    //     self.stored
    //         .readable
    //         .bundles
    //         .get(key)
    //         .cloned()
    //         .ok_or(anyhow!("get_bundle: bundle '{key}' does not exist"))
    // }

    fn modify_bundle(&mut self, key: String, bundle: Bundle) -> Result<()> {
        if self.stored.readable.bundles.contains_key(&key) {
            match self.o_transient {
                None => Err(anyhow!(t!("password_not_available"))),
                Some(ref mut transient) => {
                    self.stored.readable.bundles.modify(key, bundle, transient)
                }
            }
        } else {
            Err(anyhow!("modify_bundle: bundle '{key}' does not exist"))
        }
    }

    fn delete_bundle(&mut self, key: String) -> Result<()> {
        if self.stored.readable.bundles.contains_key(&key) {
            match self.o_transient {
                None => Err(anyhow!(t!("password_not_available"))),
                Some(ref mut transient) => self
                    .stored
                    .readable
                    .bundles
                    .remove_bundle_with_refs(key, transient),
            }
        } else {
            Err(anyhow!(t!("delete_bundle: bundle '%{key}' does not exist")))
        }
    }

    fn refs(&self, key: &str) -> (Vec<u64>, bool) {
        self.stored
            .readable
            .bundles
            .0
            .get(key)
            .unwrap_or_else(|| panic!("no bundle for key {key}"))
            .refs()
    }

    fn save(&mut self) -> Result<()> {
        let used_refs = self.stored.readable.bundles.refs();
        let provided_refs = self.transient().as_ref().unwrap().refs();
        assert_eq!(used_refs, provided_refs);

        let mut prod_lock = if *self
            .stored
            .readable
            .header
            .update_counter
            .peek()
            .as_ref()
            .unwrap()
            == 0
        {
            Self::lock_and_create_empty(&self.file_path)?
        } else {
            let (prod_lock, old_pl_file) = Self::lock_and_read(&self.file_path)?;
            // assert that the update_counter is not changed
            if old_pl_file.stored.readable.header.update_counter.peek()
                != self.stored.readable.header.update_counter.peek()
            {
                return Err(anyhow!(t!(
                    "Cannot save because the file was updated concurrently"
                )));
            }
            prod_lock
        };

        let _prod_guard = prod_lock.write();

        if self.o_transient.is_none() {
            return Err(anyhow!(t!("Cannot save because the password is not set")));
        }

        if !self.stored.readable.bundles.is_storable() {
            return Err(anyhow!(t!(
                "Cannot save because not all secrets are in correct state"
            )));
        }

        // prepare for save
        self.stored.readable.header.update_counter.next();
        self.stored.cipher = self
        .o_transient
        .as_mut()
        .unwrap(/* cannot fail */)
        .as_cipher(&self.stored.readable)?;

        // store to temp file
        let temp_path = FileList::temp_document_path()?;
        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(temp_path.clone())?;
        temp_file.write_all(PREFACE.as_bytes())?;
        temp_file.write_all(serde_json::ser::to_string_pretty(&self.stored)?.as_bytes())?;
        temp_file.write_all(b"\n")?;
        temp_file.flush()?;

        // read temp file and compare
        let (_temp_lock, mut temp_pl_file) = PlFile::lock_and_read(&temp_path)?;
        temp_pl_file.set_actionable(
            self.o_transient
                .as_ref()
                .unwrap()
                .get_storage_password()
                .to_string(),
        )?;
        if !self.equals_logically(&temp_pl_file) {
            return Err(anyhow!("save: write/read cycle failed"));
        }

        // copy temp file over prod file
        match std::fs::rename(temp_path, &self.file_path) {
            Ok(()) => Ok(()),
            Err(e) => {
                // TODO rollback by discarding the temp file and re-reading the old file
                Err(e.into())
            }
        }
    }

    fn equals_logically(&self, other: &PlFile) -> bool {
        let my_transient = self.o_transient.as_ref().unwrap();
        let other_transient = other.o_transient.as_ref().unwrap();
        self.stored
            .readable
            .bundles
            .0
            .iter()
            .zip(&other.stored.readable.bundles.0)
            .all(|((s1, b1), (s2, b2))| {
                *s1 == *s2
                    && b1.description == b2.description
                    && b1.creds.iter().zip(&b2.creds).all(|(my, other)| {
                        my.name(my_transient) == other.name(other_transient)
                            && my.secret(my_transient) == other.secret(other_transient)
                    })
            })
    }

    pub fn save_with_added_bundle(&mut self, name: String, bundle: Bundle) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("internal error: can't save with empty name"));
        }
        if self.has_bundle(&name) {
            return Err(anyhow!("a bundle with name {name} exists already"));
        }
        self.add_bundle(name, bundle)?;

        self.save()?;
        Ok(())
    }

    pub fn save_with_deleted_bundle(&mut self, name: String) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("internal error: can't save with empty name"));
        }
        if !self.has_bundle(&name) {
            return Err(anyhow!("a bundle with name {name} does not exist"));
        }
        self.delete_bundle(name)?;

        self.save()?;
        Ok(())
    }

    pub fn save_with_updated_bundle(
        &mut self,
        orig_name: &str,
        name: String,
        bundle: &Bundle,
    ) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("internal error: can't save with empty name"));
        }

        // remember all previously used refs
        let (mut old_refs, found_non_reffed_secrets) = self.refs(orig_name);
        assert!(
            !found_non_reffed_secrets,
            "internal error: can't save non-reffed Secrets"
        );
        old_refs.sort_unstable();

        if name == orig_name {
            self.modify_bundle(name, bundle.clone())?;
        } else {
            self.add_bundle(name, bundle.clone())?;
            self.delete_bundle(orig_name.to_string())?;
        }

        // garbage-collect all now redundant secrets
        // - remove from old_refs all refs that are still in bundle
        for cred in &bundle.creds {
            if let Secret::Ref(reff) = &cred.name {
                if let Ok(index) = old_refs.binary_search(reff) {
                    old_refs.remove(index);
                }
            }
            if let Secret::Ref(reff) = &cred.secret {
                if let Ok(index) = old_refs.binary_search(reff) {
                    old_refs.remove(index);
                }
            }
        }

        // - remove all remaining old_refs from Secrets
        if let Some(transient) = &mut self.o_transient {
            for reff in old_refs {
                transient.remove_secret(reff);
            }
        }

        self.save()?;

        Ok(())
    }
}

fn skip_over_comments_and_empty_lines(file_content: &str) -> &str {
    let mut start = 0;
    while let Some(next_line_break) = file_content[start..].find('\n') {
        let line = file_content[start..start + next_line_break].trim_start();
        if !line.is_empty() && !line.starts_with('#') {
            break;
        }
        start += next_line_break + 1;
    }
    &file_content[start..]
}

// #[cfg(test)]
// mod test {
//     use crate::data::bundle::Bundle;

//     use super::TEST_DOC_FOLDER;
//     use anyhow::Context;

//     #[test]
//     fn good_cycle() {
//         // ensure that the doc file does not exist (to not get confused with prev test runs)
//         let file = super::prod_document_path().unwrap();
//         let doc_dir = file.parent().unwrap();
//         if doc_dir.exists() {
//             assert!(
//                 doc_dir.ends_with(TEST_DOC_FOLDER),
//                 "doc_dir = {}",
//                 doc_dir.display()
//             );
//             std::fs::remove_dir(doc_dir).ok();
//         }

//         // test open then save
//         let mut f = super::PlFile::read_or_create().context("open").unwrap();
//         f.set_actionable("password".to_string()).unwrap();

//         let key = format!("dummy{}", f.len());
//         let mut bundle = Bundle::new(
//             "some longer description\n\
//         some longer description\n\
//         some longer description\n\
//         some longer description\n\
//         some longer description\nsome longer description\n"
//                 .to_string(),
//         );
//         bundle.add_cred("user1".to_string(), "SeCreT1".to_string());
//         bundle.add_cred("user2".to_string(), "SeCreT2".to_string());
//         f.add_bundle(&key, bundle).unwrap();
//         f.save().context("save").unwrap();
//         assert!(file.exists());

//         // test open then check creds
//         let mut f = super::PlFile::read_or_create().context("open").unwrap();
//         f.set_actionable("password".to_string()).unwrap();
//         let transient = f.o_transient.as_ref().unwrap();

//         for (_id, bundle) in f.bundles() {
//             assert_eq!(bundle.len(), 2);
//             assert_eq!(bundle.secret_value("user1", transient), "SeCreT1");
//             assert_eq!(bundle.secret_value("user2", transient), "SeCreT2");
//         }
//     }

//     #[test]
//     fn test_skip_over_comments_and_empty_lines() {
//         assert_eq!(
//             super::skip_over_comments_and_empty_lines(
//                 r"

//     # dasdsad
//     # dasdsadertdtr

//        # adasdsad

// Hello
// world!"
//             ),
//             "Hello\nworld!"
//         );

//         assert_eq!(
//             super::skip_over_comments_and_empty_lines("Hello\nworld!"),
//             "Hello\nworld!"
//         );
//     }

//     use crate::data::{secret::Secret, PlFile};
//     const TEST_PASSWORD: &str = "skudfh";

//     #[test]
//     fn test_modify_bundle() {
//         // initialize
//         let mut pl_file = PlFile::read_or_create().unwrap();
//         pl_file.set_actionable(TEST_PASSWORD.to_string()).unwrap();

//         // load test data
//         pl_file.add_test_bundles(false).unwrap();
//         let pl_file_clone = pl_file.clone();
//         assert!(pl_file.content_is_equal_to(&pl_file_clone));

//         let mut bundle = pl_file.get_bundle("ddd").unwrap();
//         {
//             // modify ddd bundle to have the same content as given by modified_bundle()
//             bundle.description = "modified description".to_string();
//             *bundle.creds.get_mut("ddd_cn1").unwrap() = Secret::New("ddd_cs1mod".to_string());
//             bundle.creds.remove("ddd_cn2");
//             bundle
//                 .creds
//                 .insert("ddd_cn4".to_string(), Secret::New("ddd_cs4".to_string()));
//         }
//         pl_file.modify_bundle("ddd".to_string(), bundle).unwrap();

//         // verify
//         let mut pl_file2 = PlFile::read_or_create().unwrap();
//         pl_file2.set_actionable(TEST_PASSWORD.to_string()).unwrap();

//         // load test data
//         pl_file2.add_test_bundles(true).unwrap();
//         if pl_file.print_content(false) != pl_file2.print_content(false) {
//             println!(
//                 "assert_eq failed, left: {}, \nright: {}",
//                 pl_file.print_content(false),
//                 pl_file2.print_content(false)
//             );
//             assert!(false, "Comparison failed");
//         }
//     }

//     #[test]
//     fn test_rename_bundle() {
//         // initialize
//         let mut pl_file = PlFile::read_or_create().unwrap();
//         pl_file.set_actionable(TEST_PASSWORD.to_string()).unwrap();

//         // load test data
//         pl_file.add_test_bundles(false).unwrap();
//         let pl_file_clone = pl_file.clone();
//         assert!(pl_file.content_is_equal_to(&pl_file_clone));

//         let mut bundle = pl_file.get_bundle("ddd").unwrap();
//         {
//             // modify ddd bundle to have the same content as given by modified_bundle()
//             bundle.description = "modified description".to_string();
//             *bundle.creds.get_mut("ddd_cn1").unwrap() = Secret::New("ddd_cs1mod".to_string());
//             bundle.creds.remove("ddd_cn2");
//             bundle
//                 .creds
//                 .insert("ddd_cn4".to_string(), Secret::New("ddd_cs4".to_string()));
//         }
//         pl_file.modify_bundle("ddd".to_string(), bundle).unwrap();

//         // verify
//         let mut pl_file2 = PlFile::read_or_create().unwrap();
//         pl_file2.set_actionable(TEST_PASSWORD.to_string()).unwrap();

//         // load test data
//         pl_file2.add_test_bundles(true).unwrap();
//         if pl_file.print_content(false) != pl_file2.print_content(false) {
//             println!(
//                 "assert_eq failed, left: {}, \nright: {}",
//                 pl_file.print_content(false),
//                 pl_file2.print_content(false)
//             );
//             assert!(false, "Comparison failed");
//         }
//     }
// }
