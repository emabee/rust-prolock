# Secure and Comfortable Management of Secrets

ProLock is a small utility to manage secrets, storing them in a password-secured file.

## Motivation

There are some secrets that you might not want to manage in any browser's
password store, e.g. the passwords for your bank accounts.

ProLock allows managing secrets in a minimalistic and secure fashion:

- users interact with ProLock via the UI
- ProLock stores the secrets safely encrypted in a file.
  ProLock uses ChaCha20-Poly1305 as encryption algorithm, which is also used in important
  protocols and SW stacks.
- ProLock interacts only with the file system, it has no functionality to open or use
  network connections or to interact with other programs.
  The user has full control of what is happening.
- ProLock is completely written in [rust](www.rust-lang.org),
  and open source.

## Encryption

ProLock protects the sensitive part of the data with [ChaCha20-Poly1305](https://en.wikipedia.org/wiki/ChaCha20-Poly1305),
an AEAD (authenticated encryption with associated data) algorithm
that combines the ChaCha20 stream cipher with the Poly1305 message authentication code.

ChaCha20-Poly1305 takes as input a 256-bit key and a 96-bit nonce to encrypt a plaintext.
ProLock uses PBKDF2 (password-based key derivation function 2)
with 91,232 rounds to derive the key from a user-provided password,
and generates new values for the salt (for PBKDF2) and for the nonce
with every update to the file.

## UI

ProLock provides a UI to manage the data conveniently.
The UI is written in rust, with the `egui` framework.
It supports currently two languages, English and German; other languages can easily be added.

## File access

ProLock only reads and writes to files in the local host's file system.
By default, ProLock uses the user-specific file `~/.prolock/secrets`,
but you can use any other file name and location.

ProLock does not interact with any cloud service etc.

ProLock detects concurrent changes to the file and refuses to overwrite them.

### Data model

The data model consists of `Entry`s, each of which has

- an *unprotected* section consisting of a unique name and an optional description
- a *protected* section with up to 4 credentials, each of which consists of a name and a value.

### File format

The file contains

- a **readable file header**
  - helping correctly managing the file format (in case we need to evolve the file format
    in future versions of ProLock) and detecting concurrent changes.
- a **readable data part**
  - showing the **unprotected sections** of the Entries
  - being also used as authentication tag for the encryption of the
    protected part (see below), which ensures that decrypting the protected part
    is only possible if the readable data part was not modified.
- some **ciphertext**, which is a serialization of the
[ChaCha20Poly1305](https://crates.io/crates/chacha20poly1305/0.10.1)-encrypted content
of the **protected sections** of the Entries.
  - the **key** for the encryption is derived from a user-given password using `pbkdf2`.
  - a new initialization vector for the encryption is diced with every file update
  - the encrypted data starts additionally with some random one-off String,
    to avoid any attack surface if the protected data set is very small.

The file format allows sneaking into the file with a plain text editor
to have a glimpse on the unprotected part, as you can see the names and the
descriptions of the contained entries, but the protected part is safely encrypted.

Note that decrypting the encrypted part requires not only the right passphrase
as input, but also the unmodified content of the readable part.
Every modification of the unprotected part prevents the decryption of the protected part.
