
# Status of Github Actions

## Qualification of every push

With every push to the main branch, **qualify the code**:

- ensure formatting is correct
- run clippy in appropriate configuration (nightly)
- run the tests.

This is achieved with `build_and_test.yml`.

## Release a version

This is currently a manual process.

An (empty) new release is created manually from the github UI.

The version should be named as `v<cargo version>`,
e.g., if the Cargo version is 0.3.5, then the release must be named `v0.3.5`.

The release creation triggers an action that does, for each platform
(macOS-13 and macOS-latest, later also windows-latest, ubuntu-latest) the following:

- make sure the tag name and the cargo version match as described above
- **build a platform-specific deployable archive in release mode**
- attach the archive with the bundle to the release.

### Status (for Mac): it does not work correctly

The archive is built as expected. We use `cargo bundle` to create a bundle,
and the bundle is put into a new archive.
**BUT**: the bundle is not accepted by MacOS, when you try e.g. to install it
into your Mac's Applications folder. An error appears,
stating that "ProLock.app" is damaged and cannot be opened
(this is *not* a permission problem, there is no option to overcome this).

In contrast, when I use `cargo bundle` locally, then I do get a bundle that works!
It is unclear to me how to fix the github action. :-(
