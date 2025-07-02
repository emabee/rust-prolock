# What I want to achieve

## Qualification of every push

With every push to the master branch **qualify the code**:

- run clippy in appropriate configuration (nightly)
- run the tests.

This is achieved with `build_and_test.yml`.

## Releasing a version

With every push to the master branch that is tagged with a version,
**additionally build it and wrap it into a platform-specific deployable archive**,
for four platforms (macOS-13 and macOS-latest, windows-latest, ubuntu-latest).

- build the program in release mode

- encapsulate the built binary so that it can most easily be deployed
  - on Mac (macOS-13 and macOS-latest), create an app bundle that can be deployed as
  application
  - on windows and ubuntu, whatever similar approach makes sense there

- make all platform-specific encapsulations available as releases on github so that they can easily
  be retrieved by customers.
