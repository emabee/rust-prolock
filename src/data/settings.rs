use anyhow::{Context, Result, anyhow};
use fd_lock::RwLock as FdRwLock;
use oxilangtag::LanguageTag;
use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::file_dialog::FileDialogLabels;

const PROD_PROLOCK_FOLDER: &str = ".prolock";
const TEST_PROLOCK_FOLDER: &str = ".prolock_test";
const SETTINGS_FILENAME: &str = "settings";
const DEFAULT_DOCUMENT_FILENAME: &str = "secrets";
const TEMP_DATA_FILE_SUFFIX: &str = "_temp_file_for_secure_storing_780987z543w";

const DEFAULT_LOCALE: &str = "en";

#[derive(Deserialize, Serialize)]
pub struct Settings {
    #[serde(skip)]
    pub prolock_folder: PathBuf,
    pub files: Vec<PathBuf>,
    pub current_file: usize,
    pub language: String,
    #[serde(default)]
    is_test: bool,
}

impl Settings {
    //
    pub fn new(is_test: bool) -> Result<Self> {
        let prolock_folder = Self::prolock_folder(is_test)?;
        Ok(Self {
            prolock_folder: prolock_folder.clone(),
            files: vec![Self::default_document_file(prolock_folder)],
            current_file: 0,
            language: {
                let locale = sys_locale::get_locale().unwrap_or(DEFAULT_LOCALE.to_string());
                LanguageTag::parse(locale)
                    .unwrap_or_else(
                        |_e| LanguageTag::parse(DEFAULT_LOCALE.to_string()).unwrap(/*OK*/),
                    )
                    .primary_language()
                    .to_string()
            },
            is_test,
        })
    }

    pub fn read_or_create(is_test: bool) -> Result<Self> {
        let prolock_folder = Self::prolock_folder(is_test)?;
        let my_file = Self::settings_file(prolock_folder.clone());
        let context = format!("reading {}", my_file.display());
        let mut settings = if std::fs::exists(&my_file).context(context.clone())? {
            Self::lock_and_read(&my_file).context(context.clone())?
        } else {
            create_dir_all(my_file.parent().context(t!("cannot_determine_folder"))?)?;
            let settings = Settings::new(is_test)?;
            settings.save()?;

            settings
        };

        rust_i18n::set_locale(&settings.language);
        rust_i18n::i18n!("locales", fallback = "en");
        settings.prolock_folder = prolock_folder;
        Ok(settings)
    }

    fn prolock_folder(is_test: bool) -> Result<PathBuf> {
        {
            let mut file_path = Self::home_dir()?;
            file_path.push(if is_test || cfg!(test) {
                TEST_PROLOCK_FOLDER
            } else {
                PROD_PROLOCK_FOLDER
            });
            Ok(file_path)
        }
    }
    fn home_dir() -> Result<PathBuf> {
        dirs::home_dir().context("Can't find home directory")
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
        let my_file = Self::settings_file(self.prolock_folder.clone());
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

    fn settings_file(mut prolock_folder: PathBuf) -> PathBuf {
        prolock_folder.push(SETTINGS_FILENAME);
        prolock_folder
    }

    fn default_document_file(mut prolock_folder: PathBuf) -> PathBuf {
        prolock_folder.push(DEFAULT_DOCUMENT_FILENAME);
        prolock_folder
    }

    pub fn temp_document_file(path: &Path) -> Result<PathBuf> {
        let mut file_path = PathBuf::from(path);
        let mut name = file_path.file_name().context("file name")?.to_owned();
        name.push(TEMP_DATA_FILE_SUFFIX);
        file_path.set_file_name(name);
        Ok(file_path)
    }

    pub fn default_export_file_path(&self) -> String {
        format!(
            "{home_dir}/{filename}",
            home_dir = self.prolock_folder.parent().unwrap(/*OK*/).display(),
            filename = Self::default_export_file_name()
        )
    }

    pub fn default_export_file_name() -> String {
        format!(
            "export_{infix}.prolock",
            infix = whoami::username().unwrap_or_else(|_| "0".to_string())
        )
    }

    pub fn get_file_dialog_labels(&self) -> FileDialogLabels {
        if &self.language == "de" {
            Self::get_german_file_dialog_labels()
        } else {
            Self::get_english_file_dialog_labels()
        }
    }

    fn get_english_file_dialog_labels() -> FileDialogLabels {
        FileDialogLabels::default()
    }

    fn get_german_file_dialog_labels() -> FileDialogLabels {
        FileDialogLabels {
            title_select_directory: "📁 Ordner auswählen".to_string(),
            title_select_file: "📂 Datei öffnen".to_string(),
            title_select_multiple: " Dateien auswählen".to_string(),
            title_save_file: "📥 Datei sichern".to_string(),

            cancel: "Abbrechen".to_string(),
            overwrite: "Überschreiben".to_string(),

            reload: "⟲  Neu laden".to_string(),
            working_directory: "↗  Zum Arbeitsordner wechseln".to_string(),
            select_all: "Alle auswählen".to_string(),
            show_hidden: "Versteckte Dateien anzeigen".to_string(),
            show_system_files: "System-Dateien anzeigen".to_string(),

            heading_pinned: "Befestigt".to_string(),
            heading_places: "Orte".to_string(),
            heading_devices: "Geräte".to_string(),
            heading_removable_devices: "Entfernbare Geräte".to_string(),

            home_dir: "🏠  Home".to_string(),
            desktop_dir: "🖵  Desktop".to_string(),
            documents_dir: "🗐  Dokumente".to_string(),
            downloads_dir: "📥  Downloads".to_string(),
            audio_dir: "🎵  Audio".to_string(),
            pictures_dir: "🖼  Bilder".to_string(),
            videos_dir: "🎞  Videos".to_string(),

            pin_folder: "📌 Befestigen".to_string(),
            unpin_folder: "✖ Lösen".to_string(),
            rename_pinned_folder: "✏ Umbenennen".to_string(),

            selected_directory: "Ausgewählter Ordner:".to_string(),
            selected_file: "Ausgewählte Datei:".to_string(),
            selected_items: "Ausgewählte Objekte:".to_string(),
            file_name: "Dateiname:".to_string(),
            file_filter_all_files: "Alle Dateien".to_string(),
            save_extension_any: "Alle".to_string(),

            open_button: "🗀  Öffnen".to_string(),
            save_button: "📥  Speichern".to_string(),
            cancel_button: "🚫 Abbrechen".to_string(),

            overwrite_file_modal_text: "existiert bereits. Überschreiben?".to_string(),

            err_empty_folder_name: "Der Ordnername darf nicht leer sein".to_string(),
            err_empty_file_name: "Der Dateiname darf nicht leer sein".to_string(),
            err_directory_exists: "Ein Ordner mit diesem Namen existiert bereits".to_string(),
            err_file_exists: "Eine Datei mit diesem Namen existiert bereits".to_string(),
        }
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
