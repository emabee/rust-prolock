# Secure and comfortable management of secrets

ProLock is a small utility to manage secrets.

### File access

ProLock only reads and writes to a single, user-specific file (`~/.prolock/secrets`).
It does not interact with any cloud service etc.

### UI

ProLock comes with a UI to manage the secrets conveniently.
the ProLock UI supports currently English and German; other languages can easily be added.

### Data model

The data model consists of `Entry`s, each of which has

- an unprotected section consisting of the unique name and a description
- a protected section with 1 to 4 credentials, each of which consists of a name and a secret.

### File format

The file contains

- some readable file header which helps managing the file format correctly and detecting
  concurrent changes
- a readable data part showing the **unprotected sections** of the Entries
- the readable part of the file is also used as authentication tag for the encryption of the
  protected part (see below), which ensures that opening the file is only possible if the clear
  text was not edited concurrently.
- some ciphertext encoding the
[ChaCha20Poly1305](https://crates.io/crates/chacha20poly1305/0.10.1)-encrypted content
of the **protected sections** of the Entries.
  - the key for the encryption is derived from a user-given password using `pbkdf2`.
  - a new initialization vector for the encryption is diced with every file update
  - the encrypted data starts additionally with some random one-off String,
    to avoid any attack surface even if the protected data set is very small.
