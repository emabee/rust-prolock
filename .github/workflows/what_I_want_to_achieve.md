# What I want to achieve

## Qualification of every push

With every push to the main branch, **qualify the code**:

- ensure formatting is correct
- run clippy in appropriate configuration (nightly)
- run the tests.

This is achieved with `build_and_test.yml`.

## Releasing a version

With every push to the main branch that is tagged with a version,
**build it in release mode and wrap it into a platform-specific deployable archive**,
for four platforms (macOS-13 and macOS-latest, windows-latest, ubuntu-latest).

- build the program in release mode

- encapsulate the built binary so that it can most easily be deployed
  - on Mac (macOS-13 and macOS-latest), create an app bundle that can be deployed as
  application
  - on windows and ubuntu, whatever similar approach makes sense there

- make all platform-specific encapsulations available as releases on github so that they can easily
  be retrieved by customers.

### cargo bundle

See <https://github.com/burtonageo/cargo-bundle>.

To install `cargo bundle` locally, run `cargo install cargo-bundle`.
