use anyhow::{Context, Result};
use fd_lock::RwLock as FdRwLock;
use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{Read, Write},
    path::{Path, PathBuf},
};

const FILE_LIST_FILE: &str = "files";
const PROD_DOC_FOLDER: &str = ".prolock";
const TEST_DOC_FOLDER: &str = ".prolock_test";
const PROD_DOC_FILE: &str = "secrets";
const TEMP_DOC_FILE: &str = "secrets_temp";

#[derive(Deserialize, Serialize)]
pub struct FileList {
    pub files: Vec<PathBuf>,
    pub current_file: usize,
}

impl FileList {
    pub fn default() -> Result<Self> {
        Ok(Self {
            files: vec![Self::prod_document_path()?],
            current_file: 0,
        })
    }
    pub fn read_or_create() -> Result<Self> {
        let flf = Self::my_file()?;
        if std::fs::exists(&flf)? {
            Self::lock_and_read(&flf)
        } else {
            create_dir_all(
                flf.parent()
                    .context("cannot determine folder for storage")?,
            )?;

            let file_list = FileList::default()?;
            FileList::lock_for_write(&flf)?
                .write()?
                .write_all(serde_json::ser::to_string_pretty(&file_list)?.as_bytes())?;
            Ok(file_list)
        }
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
        let flf = Self::my_file()?;
        FileList::lock_for_write(&flf)?
            .write()?
            .write_all(serde_json::ser::to_string_pretty(&self)?.as_bytes())?;
        Ok(())
    }

    fn lock_and_read(file_path: &Path) -> Result<Self> {
        {
            let file = File::open(file_path).context(t!("opening file"))?;
            let mut file_lock = FdRwLock::new(file);
            Self::read_stored(&mut file_lock, file_path)
        }
    }

    fn read_stored(file_lock: &mut FdRwLock<File>, file_path: &Path) -> Result<FileList> {
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
    pub fn add_and_set_file(&mut self, file: PathBuf) -> Result<()> {
        self.files.push(file);
        self.current_file = self.files.len() - 1;
        self.save()
    }

    fn my_file() -> Result<PathBuf> {
        let mut file_path = Self::document_folder()?;
        file_path.push(FILE_LIST_FILE);
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
