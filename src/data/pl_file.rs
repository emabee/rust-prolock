use super::{Bundle, Bundles, Secrets, Transient};
use crate::data::Secret;
use anyhow::{anyhow, Context, Result};
use fd_lock::RwLock as FdRwLock;
use oxilangtag::LanguageTag;
use rust_i18n::t;
use sequential::Sequence;
#[cfg(test)]
use std::collections::btree_map::Iter;
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Write as _},
    path::{Path, PathBuf},
};

const PROD_DOC_FOLDER: &str = ".prolock";
const TEST_DOC_FOLDER: &str = ".prolock_test";
const PROD_DOC_FILE: &str = "secrets";
const TEMP_DOC_FILE: &str = "secrets_temp";
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
pub(crate) struct PlFile {
    pub(crate) file_path: PathBuf,
    pub(crate) stored: Stored,
    o_transient: Option<Transient>,
}

// This is the structure that is serialized to the file (after the preface);
// it consists of a readable section and an encrypted section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Stored {
    pub(crate) readable: Readable,
    pub(crate) cipher: String,
}

// The readable section is written in clear and also used as auth-tag for the encrypted section
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Readable {
    pub(crate) header: FileHeader,
    pub(crate) bundles: Bundles,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct FileHeader {
    pub(crate) format_version: u8,
    pub(crate) language: String,
    pub(crate) update_counter: Sequence<usize>,
}

impl PlFile {
    pub(crate) fn read_or_create() -> Result<Self> {
        let file_path = prod_document_path()?;
        if file_path.exists() {
            let result = Self::lock_and_read(&file_path)?.1;
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
                file_path,
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
        serde_json::from_str(semantic_content).context(t!("parsing"))
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.stored.readable.bundles.len()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.stored.readable.bundles.is_empty()
    }

    pub(crate) fn transient(&self) -> Option<&Transient> {
        self.o_transient.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn bundles(&self) -> Iter<'_, String, Bundle> {
        self.stored.readable.bundles.into_iter()
    }
    pub(crate) fn has_bundle(&self, name: &str) -> bool {
        self.stored.readable.bundles.contains_key(name)
    }

    pub(crate) fn is_actionable(&self) -> bool {
        self.o_transient.is_some()
    }
    pub(crate) fn set_actionable(&mut self, password: String) -> Result<()> {
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

    pub(crate) fn change_password(&mut self, old_pw: &str, new_pw: String) -> Result<()> {
        self.check_password(old_pw)?;
        if let Some(ref mut transient) = self.o_transient {
            transient.set_storage_password(new_pw);
            self.save()?;
        }
        Ok(())
    }

    pub(crate) fn check_password(&mut self, old_pw: &str) -> Result<()> {
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

    #[cfg(test)]
    pub(crate) fn get_bundle(&self, key: &str) -> Result<Bundle> {
        self.stored
            .readable
            .bundles
            .get(key)
            .cloned()
            .ok_or(anyhow!("get_bundle: bundle '{key}' does not exist"))
    }

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
        .as_ref()
        .unwrap(/* cannot fail */)
        .as_cipher(&self.stored.readable)?;

        // store to temp file
        let temp_path = temp_document_path()?;
        let prod_path = prod_document_path()?;
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
        match std::fs::rename(temp_path, prod_path) {
            Ok(()) => Ok(()),
            Err(e) => {
                // FIXME rollback by discarding the temp file and re-reading the old file
                Err(e.into())
            }
        }
    }

    fn equals_logically(&self, other: &PlFile) -> bool {
        let result = self
            .stored
            .readable
            .bundles
            .0
            .iter()
            .zip(&other.stored.readable.bundles.0)
            .all(|((s1, b1), (s2, b2))| {
                *s1 == *s2
                    && b1.description == b2.description
                    && b1.named_secrets.keys().zip(b2.named_secrets.keys()).all(
                        |(my_key, other_key)| {
                            my_key == other_key
                                && b1.secret_value(my_key, self.o_transient.as_ref().unwrap())
                                    == b2.secret_value(
                                        other_key,
                                        other.o_transient.as_ref().unwrap(),
                                    )
                        },
                    )
            });
        if !result {
            println!("self = {self:?}");
            println!("other = {other:?}");
        }
        result
    }

    pub(crate) fn save_with_added_bundle(&mut self, name: String, bundle: Bundle) -> Result<()> {
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

    pub(crate) fn save_with_deleted_bundle(&mut self, name: String) -> Result<()> {
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

    pub(crate) fn save_with_updated_bundle(
        &mut self,
        orig_name: &str,
        name: String,
        bundle: &Bundle,
    ) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("internal error: can't save with empty name"));
        }

        if name == orig_name {
            // remember all previously used refs
            let (mut old_refs, found_non_reffed_secrets) = self.refs(&name);
            assert!(
                !found_non_reffed_secrets,
                "internal error: can't save non-reffed Secrets"
            );
            old_refs.sort_unstable();

            self.modify_bundle(name, bundle.clone())?;

            // garbage-collect all now redundant secrets
            // - remove from old_refs all refs that are still in bundle
            for secret in bundle.named_secrets.values() {
                if let Secret::Ref(reff) = secret {
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
        } else {
            // if name was changed
            //      remember all previously used refs
            //      add the new value to pl_file
            //      remove the previously used key-value pair
            //      make sure all Secrets are Ref'ed
            //      garbage-collect all now redundant secrets
            unimplemented!("bundle name was changed");
        }

        self.save()?;

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn content_is_equal_to(&self, other: &PlFile) -> bool {
        // TODO does not compare everything, esp. not the secret values!
        self.file_path == other.file_path
            && self
                .stored
                .readable
                .bundles
                .0
                .iter()
                .zip(&other.stored.readable.bundles.0)
                .map(|((s1, b1), (s2, b2))| {
                    *s1 == *s2
                        && b1.description == b2.description
                        && b1.named_secrets.keys().collect::<Vec<&String>>()
                            == b2.named_secrets.keys().collect::<Vec<&String>>()
                })
                .fold(true, |a, b| a && b)
    }

    #[cfg(test)]
    pub(crate) fn print_content(&self, with_transient: bool) -> String {
        use std::fmt::Write as _;
        let spc = |n: usize| -> String { " ".repeat(4 * n) };

        // We skip
        //     self.file_path: PathBuf,
        //     self.stored.cipher: String,
        //     in self.o_transient: Option<Transient>,
        //          transient.storage_password: SecUtf8,
        //          transient.seq_for_secret_refs: Sequence<u64>,
        let mut output = String::with_capacity(1000);

        macro_rules! wrt {
            ($n:expr, $($args:tt)*) => {
                write!(&mut output, "{}", spc($n)).ok();
                writeln!(&mut output, $($args)*).ok();
            };
        }

        let r = &self.stored.readable;
        wrt!(0, "PlFile: {{");
        wrt!(1, "FileHeader: {{");
        wrt!(2, "format_version: {}", &r.header.format_version);
        wrt!(2, "update_counter: {:?}", r.header.update_counter);
        wrt!(1, "}},");
        wrt!(1, "Bundles: ({{");
        for (name, bundle) in &self.stored.readable.bundles.0 {
            wrt!(2, "{:?}: Bundle {{", name);
            wrt!(3, "description: {:?}", bundle.description);
            wrt!(3, "named_secrets: {{");
            if let Some(ref transient) = self.o_transient {
                for (name, secret) in &bundle.named_secrets {
                    wrt!(4, "{}: ({})", name, secret.disclose(transient));
                }
            }
            wrt!(3, "}},");
            wrt!(2, "}},");
        }
        if with_transient {
            if let Some(ref transient) = self.o_transient {
                wrt!(1, "}},");
                wrt!(1, "{}", transient.as_string());
            } else {
                wrt!(1, "}}");
            }
        } else {
            wrt!(1, "}}");
        }
        wrt!(0, "}}");
        output
    }

    #[cfg(test)]
    pub(crate) fn add_test_bundles(&mut self, modified: bool) -> Result<(), anyhow::Error> {
        self.add_bundle(
            "Bank of North America",
            Bundle::new_with_creds(&"aaa_dscr", &[("aaa_cn", "aaa_cs")]),
        )?;
        self.add_bundle(
            "Bank of South America",
            Bundle::new_with_creds(
                &"http://one_bank.de\n\n\
                Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
                Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
                Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
                Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
                Hello world! Hello world! Hello world! Hello world! Hello world! Hello world! \
                Hello world! Hello world! ",
                &[("aaa_cn", "aaa_cs"), ("asdaqweqweqwe", "rtzrtzfhfghgfh")],
            ),
        )?;
        self.add_bundle(
            "ccc",
            Bundle::new_with_creds(
                &"ccc_dscr1\n\
                ccc_dscr2\n\
                ccc_dscr3\n\
                ccc_dscr4\n\
                ccc_dscr5",
                &[
                    ("ccc_cn1", "ccc_cs"),
                    ("ccc_cn2", "ccc_cs"),
                    ("ccc_cn3", "ccc_cs"),
                ],
            ),
        )?;
        if modified {
            self.add_bundle("ddd", modified_bundle())?;
        } else {
            self.add_bundle("ddd", unmodified_bundle())?;
        }
        self.add_bundle(
            "eee",
            Bundle::new_with_creds(&"eee_dscr", &[("eee_cn", "eee_cs")]),
        )?;
        self.add_bundle(
            "fff",
            Bundle::new_with_creds(&"fff_dscr", &[("fff_cn", "fff_cs")]),
        )?;
        Ok(())
    }
}

#[cfg(test)]
fn unmodified_bundle() -> Bundle {
    Bundle::new_with_creds(
        &"ddd_dscr",
        &[
            ("ddd_cn1", "ddd_cs"),
            ("ddd_cn2", "ddd_cs"),
            ("ddd_cn3", "ddd_cs"),
        ],
    )
}
#[cfg(test)]
fn modified_bundle() -> Bundle {
    Bundle::new_with_creds(
        &"modified description",
        &[
            ("ddd_cn1", "ddd_cs1mod"),
            ("ddd_cn3", "ddd_cs"),
            ("ddd_cn4", "ddd_cs4"),
        ],
    )
}

fn document_folder() -> Result<PathBuf> {
    let mut file_path = dirs::home_dir().context("Can't find home directory")?;
    file_path.push(if cfg!(test) {
        TEST_DOC_FOLDER
    } else {
        PROD_DOC_FOLDER
    });
    Ok(file_path)
}
fn prod_document_path() -> Result<PathBuf> {
    let mut file_path = document_folder()?;
    file_path.push(PROD_DOC_FILE);
    Ok(file_path)
}
fn temp_document_path() -> Result<PathBuf> {
    let mut file_path = document_folder()?;
    file_path.push(TEMP_DOC_FILE);
    Ok(file_path)
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

#[cfg(test)]
mod test {
    use crate::data::bundle::Bundle;

    use super::TEST_DOC_FOLDER;
    use anyhow::Context;

    #[test]
    fn good_cycle() {
        // ensure that the doc file does not exist (to not get confused with prev test runs)
        let file = super::prod_document_path().unwrap();
        let doc_dir = file.parent().unwrap();
        if doc_dir.exists() {
            assert!(
                doc_dir.ends_with(TEST_DOC_FOLDER),
                "doc_dir = {}",
                doc_dir.display()
            );
            std::fs::remove_dir(doc_dir).ok();
        }

        // test open then save
        let mut f = super::PlFile::read_or_create().context("open").unwrap();
        f.set_actionable("password".to_string()).unwrap();

        let key = format!("dummy{}", f.len());
        let mut bundle = Bundle::new(
            "some longer description\n\
        some longer description\n\
        some longer description\n\
        some longer description\n\
        some longer description\nsome longer description\n"
                .to_string(),
        );
        bundle.add_cred("user1".to_string(), "SeCreT1".to_string());
        bundle.add_cred("user2".to_string(), "SeCreT2".to_string());
        f.add_bundle(&key, bundle).unwrap();
        f.save().context("save").unwrap();
        assert!(file.exists());

        // test open then check creds
        let mut f = super::PlFile::read_or_create().context("open").unwrap();
        f.set_actionable("password".to_string()).unwrap();
        let transient = f.o_transient.as_ref().unwrap();

        for (_id, bundle) in f.bundles() {
            assert_eq!(bundle.len(), 2);
            assert_eq!(bundle.secret_value("user1", transient), "SeCreT1");
            assert_eq!(bundle.secret_value("user2", transient), "SeCreT2");
        }
    }

    #[test]
    fn test_skip_over_comments_and_empty_lines() {
        assert_eq!(
            super::skip_over_comments_and_empty_lines(
                r"

    # dasdsad
    # dasdsadertdtr
    
       # adasdsad

Hello
world!"
            ),
            "Hello\nworld!"
        );

        assert_eq!(
            super::skip_over_comments_and_empty_lines("Hello\nworld!"),
            "Hello\nworld!"
        );
    }

    use crate::data::{secret::Secret, PlFile};
    const TEST_PASSWORD: &str = "skudfh";

    #[test]
    fn test_modify_bundle() {
        // initialize
        let mut pl_file = PlFile::read_or_create().unwrap();
        pl_file.set_actionable(TEST_PASSWORD.to_string()).unwrap();

        // load test data
        pl_file.add_test_bundles(false).unwrap();
        let pl_file_clone = pl_file.clone();
        assert!(pl_file.content_is_equal_to(&pl_file_clone));

        let mut bundle = pl_file.get_bundle("ddd").unwrap();
        {
            // modify ddd bundle to have the same content as given by modified_bundle()
            bundle.description = "modified description".to_string();
            *bundle.named_secrets.get_mut("ddd_cn1").unwrap() =
                Secret::New("ddd_cs1mod".to_string());
            bundle.named_secrets.remove("ddd_cn2");
            bundle
                .named_secrets
                .insert("ddd_cn4".to_string(), Secret::New("ddd_cs4".to_string()));
        }
        pl_file.modify_bundle("ddd".to_string(), bundle).unwrap();

        // verify
        let mut pl_file2 = PlFile::read_or_create().unwrap();
        pl_file2.set_actionable(TEST_PASSWORD.to_string()).unwrap();

        // load test data
        pl_file2.add_test_bundles(true).unwrap();
        if pl_file.print_content(false) != pl_file2.print_content(false) {
            println!(
                "assert_eq failed, left: {}, \nright: {}",
                pl_file.print_content(false),
                pl_file2.print_content(false)
            );
            assert!(false, "Comparison failed");
        }
    }

    #[test]
    fn test_rename_bundle() {
        // initialize
        let mut pl_file = PlFile::read_or_create().unwrap();
        pl_file.set_actionable(TEST_PASSWORD.to_string()).unwrap();

        // load test data
        pl_file.add_test_bundles(false).unwrap();
        let pl_file_clone = pl_file.clone();
        assert!(pl_file.content_is_equal_to(&pl_file_clone));

        let mut bundle = pl_file.get_bundle("ddd").unwrap();
        {
            // modify ddd bundle to have the same content as given by modified_bundle()
            bundle.description = "modified description".to_string();
            *bundle.named_secrets.get_mut("ddd_cn1").unwrap() =
                Secret::New("ddd_cs1mod".to_string());
            bundle.named_secrets.remove("ddd_cn2");
            bundle
                .named_secrets
                .insert("ddd_cn4".to_string(), Secret::New("ddd_cs4".to_string()));
        }
        pl_file.modify_bundle("ddd".to_string(), bundle).unwrap();

        // verify
        let mut pl_file2 = PlFile::read_or_create().unwrap();
        pl_file2.set_actionable(TEST_PASSWORD.to_string()).unwrap();

        // load test data
        pl_file2.add_test_bundles(true).unwrap();
        if pl_file.print_content(false) != pl_file2.print_content(false) {
            println!(
                "assert_eq failed, left: {}, \nright: {}",
                pl_file.print_content(false),
                pl_file2.print_content(false)
            );
            assert!(false, "Comparison failed");
        }
    }
}
