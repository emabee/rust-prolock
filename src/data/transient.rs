use crate::data::{Readable, Secrets};
use anyhow::{Context, Result};
use pwsec::{ChachaB64, CipherB64};
use secstr::SecUtf8;
use sequential::Sequence;

const PBKDF2_ROUNDS: u32 = 91_232;

#[derive(Clone, Debug)]
pub struct Transient {
    storage_password: SecUtf8,
    seq_for_secret_refs: Sequence<u64>,
    secrets: Secrets,
}
impl Transient {
    pub fn new(password: String, secrets: Secrets) -> Self {
        Self {
            storage_password: SecUtf8::from(password),
            seq_for_secret_refs: Sequence::start_after_highest(&mut secrets.keys()),
            secrets,
        }
    }

    pub fn from_cipher(password: String, readable: &Readable, cipher: &str) -> Result<Transient> {
        let secrets = serde_json::from_slice(
            &ChachaB64::with_pbkdf2_rounds(PBKDF2_ROUNDS)
                .decrypt_auth(
                    CipherB64::parse(cipher).context("cipher")?,
                    serde_json::to_string(readable)
                        .context("parse header")?
                        .as_bytes(),
                    &password,
                )
                .context("decrypt")?,
        )
        .context("parse secrets")?;
        Ok(Transient::new(password, secrets))
    }

    pub fn add_secret(&mut self, s: String) -> u64 {
        let idx = self.seq_for_secret_refs.next().unwrap(/*ok*/);
        self.secrets.add(idx, s);
        idx
    }

    pub fn remove_secret(&mut self, idx: u64) {
        self.secrets.remove(idx);
    }

    pub fn get_secret(&self, idx: u64) -> Option<&str> {
        self.secrets.get(idx)
    }

    pub fn refs(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(self.secrets.keys().copied())
    }

    pub fn set_storage_password(&mut self, new_pw: String) {
        self.storage_password = SecUtf8::from(new_pw);
    }
    pub fn get_storage_password(&self) -> &str {
        self.storage_password.unsecure()
    }

    pub fn as_cipher(&mut self, auth_tag: &Readable) -> Result<String> {
        self.secrets.prepare();
        Ok(ChachaB64::with_pbkdf2_rounds(PBKDF2_ROUNDS)
            .encrypt_auth(
                serde_json::to_string(&self.secrets)?.as_bytes(),
                serde_json::to_string(auth_tag)?.as_bytes(),
                self.storage_password.unsecure(),
            )?
            .to_string())
    }
}
