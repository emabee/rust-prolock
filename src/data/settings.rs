use anyhow::{Context, Result, anyhow};
use fd_lock::RwLock as FdRwLock;
use oxilangtag::LanguageTag;
use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{Read, Write},
    path::{Path, PathBuf},
};

const SETTINGS_FILE: &str = "settings";
const PROD_DOC_FOLDER: &str = ".prolock";
const TEST_DOC_FOLDER: &str = ".prolock_test";
const PROD_DOC_FILE: &str = "secrets";
const TEMP_DOC_FILE: &str = "secrets_temp";
const DEFAULT_LOCALE: &str = "en";

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub files: Vec<PathBuf>,
    pub current_file: usize,
    pub language: String,
}

fn default_language() -> String {
    let locale = sys_locale::get_locale().unwrap_or(DEFAULT_LOCALE.to_string());
    LanguageTag::parse(locale)
        .unwrap_or_else(|_e| LanguageTag::parse(DEFAULT_LOCALE.to_string()).unwrap(/*OK*/))
        .primary_language()
        .to_string()
}
impl Settings {
    pub fn default() -> Result<Self> {
        Ok(Self {
            files: vec![Self::prod_document_path()?],
            current_file: 0,
            language: default_language(),
        })
    }
    pub fn read_or_create() -> Result<Self> {
        let my_file = Self::my_file()?;
        let context = format!("reading {}", my_file.display());
        let settings = if std::fs::exists(&my_file).context(context.clone())? {
            Self::lock_and_read(&my_file).context(context.clone())?
        } else {
            create_dir_all(
                my_file
                    .parent()
                    .context("cannot determine folder for storage")?,
            )?;

            let settings = Settings::default()?;
            settings.save()?;
            settings
        };

        rust_i18n::set_locale(&settings.language);
        rust_i18n::i18n!("locales", fallback = "en");
        Ok(settings)
    }

    fn lock_for_write(file_path: &Path) -> Result<FdRwLock<File>> {
        Ok(FdRwLock::new(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(file_path)
                .context("opening file")?,
        ))
    }
    fn save(&self) -> Result<()> {
        let my_file = Self::my_file()?;
        let mut file_guard = Settings::lock_for_write(&my_file)?;
        let mut locked_file = file_guard.write()?;
        locked_file.write_all(serde_json::ser::to_string_pretty(&self)?.as_bytes())?;
        locked_file.write_all(b"\n")?;
        Ok(())
    }

    fn lock_and_read(file_path: &Path) -> Result<Self> {
        {
            let file = File::open(file_path).context(t!("opening file"))?;
            let mut file_lock = FdRwLock::new(file);
            Self::read_stored(&mut file_lock, file_path)
        }
    }

    fn read_stored(file_lock: &mut FdRwLock<File>, file_path: &Path) -> Result<Settings> {
        let mut file_content = String::with_capacity(1024);
        let mut write_guard = file_lock
            .write()
            .context(format!("locking {}", file_path.display()))?;
        (*write_guard)
            .read_to_string(&mut file_content)
            .context(format!("reading {}", file_path.display()))?;
        serde_json::from_str(&file_content).context(t!("parsing FileList"))
    }

    pub fn current_file(&self) -> &Path {
        debug_assert!(
            self.current_file < self.files.len(),
            "FileList broken (1): index ({}) >= len ({})",
            self.current_file,
            self.files.len()
        );
        self.files[self.current_file].as_path()
    }
    pub fn set_current_file(&mut self, idx: usize) -> Result<()> {
        debug_assert!(
            self.current_file < self.files.len(),
            "FileList broken (2): index ({}) >= len ({})",
            self.current_file,
            self.files.len()
        );
        self.current_file = idx;
        self.save()
    }

    pub fn add_and_set_file(&mut self, file: &Path) -> Result<()> {
        let canonfile = canonicalize(file)?;
        if let Some(pos) = self.files.iter().position(|f| f == &canonfile) {
            self.current_file = pos;
            self.save()
        } else {
            self.files.push(canonfile);
            self.current_file = self.files.len() - 1;
            self.save()
        }
    }
    pub fn forget_file(&mut self, file: &Path) -> Result<()> {
        if let Some(pos) = self.files.iter().position(|f| f == file) {
            if pos == 0 {
                return Err(anyhow!("Cannot forget default file"));
            }
            self.files.remove(pos);
            if pos <= self.current_file {
                self.current_file = self.current_file.saturating_sub(1);
            }
            self.save()
        } else {
            Ok(())
        }
    }

    #[allow(clippy::unused_self)]
    pub fn default_marker(&self, idx: usize) -> &'static str {
        if idx == 0 { "(default)" } else { "" }
    }
    pub fn current_marker(&self, idx: usize) -> &'static str {
        if idx == self.current_file {
            "(current)"
        } else {
            ""
        }
    }

    pub fn set_language(&mut self, lang: &str) -> Result<()> {
        self.language.clear();
        self.language.push_str(lang);
        self.save()?;

        rust_i18n::set_locale(&self.language);
        rust_i18n::i18n!("locales", fallback = "en");
        Ok(())
    }

    fn my_file() -> Result<PathBuf> {
        let mut file_path = Self::document_folder()?;
        file_path.push(SETTINGS_FILE);
        Ok(file_path)
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
        let mut file_path = Self::document_folder()?;
        file_path.push(PROD_DOC_FILE);
        Ok(file_path)
    }
    pub fn temp_document_path() -> Result<PathBuf> {
        let mut file_path = Self::document_folder()?;
        file_path.push(TEMP_DOC_FILE);
        Ok(file_path)
    }
}

fn canonicalize(file: &Path) -> Result<PathBuf> {
    let mut parent = file.parent().context("parent folder")?.to_owned();
    if parent.to_string_lossy().is_empty() {
        parent = PathBuf::from(".");
    }
    if !parent.exists() {
        create_dir_all(parent.clone()).context("creating parent folder")?;
    }

    Ok(parent
        .canonicalize()
        .context("canonicalizing parent folder")?
        .join(file.file_name().context("file name")?))
}
