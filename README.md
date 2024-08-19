# This CLI tool returns strings, depending on the input.

Use `semver-next --help` to see the options:
```
Usage: semver-next <--major <MAJOR>|--minor <MINOR>|--patch <PATCH>|--pre-release <PRE_RELEASE>|--pre-release-patch <PRE_RELEASE_PATCH>>

Options:
  -x, --major <MAJOR>
          Returns the next major version
  -y, --minor <MINOR>
          Returns the next minor version
  -z, --patch <PATCH>
          Returns the next patch version
  -r, --pre-release <PRE_RELEASE>
          Returns the next pre-release version
  -p, --pre-release-patch <PRE_RELEASE_PATCH>
          Returns the next pre-release patch version. The next pre-release version after stable is alpha
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples
```bash
./semver-next --major "1.2.3-alpha.4" # Expected: 2.0.0

./semver-next --minor "1.2.3-alpha.4" # Expected: 1.3.0

./semver-next --patch "1.2.3-alpha.4" # Expected: 1.2.4

./semver-next --pre-release "1.2.3-alpha.4" # Expected: 1.2.3-beta.0

./semver-next --pre-release "1.2.3-beta.0" # Expected: 1.2.3-rc.0

./semver-next --pre-release "1.2.3-rc.0" # Expected: 1.2.3

./semver-next --pre-release "1.2.3" # Expected: 1.2.4-alpha.0

./semver-next --pre-release-patch "1.2.3-alpha.4" # Expected: 1.2.3-alpha.5

./semver-next --pre-release-patch "1.2.4.42" # Expected: Invalid semver string

```