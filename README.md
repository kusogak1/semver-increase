# This CLI tool returns strings, depending on the input.

Use `semver-increase --help` to see the options:
```
Usage: semver-increase <--major <MAJOR>|--minor <MINOR>|--patch <PATCH>|--pre-release <PRE_RELEASE>|--pre-release-patch <PRE_RELEASE_PATCH>>

Options:
      --major <MAJOR>                          Increase the major version
      --minor <MINOR>                          Increase the minor version
      --patch <PATCH>                          Increase the patch version
      --pre-release <PRE_RELEASE>              Increase the pre-release version
      --pre-release-patch <PRE_RELEASE_PATCH>  Increase the pre-release patch version
  -h, --help                                   Print help
  -V, --version                                Print version
```

## Examples
```bash
./semver-increase --major "1.2.3-alpha.4" # Expected: 2.0.0

./semver-increase --minor "1.2.3-alpha.4" # Expected: 1.3.0

./semver-increase --patch "1.2.3-alpha.4" # Expected: 1.2.4

./semver-increase --pre-release "1.2.3-alpha.4" # Expected: 1.2.3-beta.0

./semver-increase --pre-release "1.2.3-beta.0" # Expected: 1.2.3-rc.0

./semver-increase --pre-release "1.2.3-rc.0" # Expected: 1.2.3

./semver-increase --pre-release "1.2.3" # Expected: Invalid pre-release string

./semver-increase --pre-release-patch "1.2.3-alpha.4" # Expected: 1.2.3-alpha.5

./semver-increase --pre-release-patch "1.2.4.42" # Expected: Invalid semver string

```