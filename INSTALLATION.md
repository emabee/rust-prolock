
# How to Install `Prolock`

## Build it yourself

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

## Future: Use pre-built releases from github

WORK IN PROGRESS. SUPPORT NEEDED to fix issues with bundle creation for Mac! See [Status of github actions](https://github.com/emabee/rust-prolock/blob/main/status_of_github_actios.md>)
for details of the current issues.

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
