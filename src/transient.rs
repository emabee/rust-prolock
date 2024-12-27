use crate::{pl_file::Readable, secrets::Secrets};
use anyhow::{Context, Result};
use pwsec::{ChachaB64, CipherB64};
use secstr::SecUtf8;
use sequential::Sequence;

const PBKDF2_ROUNDS: u32 = 91_232;

#[derive(Clone, Debug)]
pub(crate) struct Transient {
    storage_password: SecUtf8,
    seq_for_secret_refs: Sequence<u64>,
    secrets: Secrets,
}
impl Transient {
    pub(crate) fn new(password: String, secret: Secrets) -> Self {
        Self {
            storage_password: SecUtf8::from(password),
            seq_for_secret_refs: Sequence::start_after_highest(&mut secret.keys()),
            secrets: secret,
        }
    }

    pub(crate) fn from_cipher(
        password: String,
        readable: &Readable,
        cipher: &str,
    ) -> Result<Transient> {
        let secret =
            serde_json::from_slice(&ChachaB64::with_pbkdf2_rounds(PBKDF2_ROUNDS).decrypt_auth(
                CipherB64::parse(cipher).context("parse")?,
                serde_json::to_string(readable)?.as_bytes(),
                &password,
            )?)?;
        Ok(Transient::new(password, secret))
    }

    pub(crate) fn add_secret_value(&mut self, s: String) -> u64 {
        let idx = self.seq_for_secret_refs.next().unwrap(/* TODO */);
        self.secrets.add(idx, s);
        idx
    }

    pub(crate) fn get_secret_value(&self, idx: u64) -> Option<&String> {
        self.secrets.get(idx)
    }

    pub(crate) fn get_storage_password(&self) -> &str {
        self.storage_password.unsecure()
    }

    pub(crate) fn as_cipher(&self, auth_tag: &Readable) -> Result<String> {
        Ok(ChachaB64::with_pbkdf2_rounds(PBKDF2_ROUNDS)
            .encrypt_auth(
                serde_json::to_string(&self.secrets)?.as_bytes(),
                serde_json::to_string(auth_tag)?.as_bytes(),
                self.storage_password.unsecure(),
            )?
            .to_string())
    }
}
