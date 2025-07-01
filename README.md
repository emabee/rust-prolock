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

## How-to-Install

### Build it yourself

If you're working with rust yourself, then it might be the easiest for you to
download this repo and build the program yourself:

1. Ensure you have a recent [rust compiler](https://www.rust-lang.org/) installed.
The MSRV of ProLock is currently 1.85.

2. Clone the project to you local disk.

3. cd into the project's root folder and call

    `cargo build --release`

    This produces the desired binary, to be found in folder `./target/release/`.
    The binary's name is `prolock` or `prolock.exe`, depending on your platform.

    This binary is all you need to run ProLock.

4. On Mac, you might want to go one step further and install it in your
   `Applications` folder, so that it can appear in the dock. For that purpose, we need
   to create an app folder that combines the binary with some necessary metadata.
   This is most easily done with [`Cargo bundle`](https://crates.io/crates/cargo-bundle)
   (which you might need to install separately first):

    4.1 `cargo bundle --release`

    This command produces a folder `ProLock.app` in
    `./target/release/bundle/osx/` with the necessary content.

    4.2 Copy this folder `ProLock.app` into your Applications folder (`/Applications`).

    Prolock can now be started with a single click from the Applications folder
    and pulled permanently into the dock, if you want.

### Use pre-built results from github

WORK IN PROGRESS, SUPPORT WELCOME!

In its current state, the project uses github actions to build the project,
and then it loads the results up into github.

Current state is:
I can't get the result running on my Mac (M4 Pro), an error report is found in `../deployment_errors`.

<!-- To find these results, open the [Actions tab](https://github.com/emabee/rust-prolock/actions)
and choose the latest action for your platform.

#### Mac (M processors)

Open e.g. `[1.1.1] Release // MacOS Build (M) #8: Commit 25d5616 pushed by emabee`
and scroll down to the *Artifacts* section. It contains the desired `ProLock.App` that
you can download and copy into the `Applications` folder.

As your Mac rejects unknown binaries, for security reasons, the app will not yet run.
This [Apple support page](https://support.apple.com/de-de/guide/mac-help/mh40616/15.0/mac/15.0)
describes a procedure for permanently allowing a concrete program.

You need to open a settings page, try starting the program (prolock),
find a notice in the open settings section, and confirm that you want this program to be opened. -->
