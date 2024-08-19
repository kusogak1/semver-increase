use {
    clap::{ArgGroup, Parser},
    semver::{Prerelease, Version},
    std::{env, fmt::Display, str::FromStr},
};

pub(crate) const INVALID_SEMVER_STRING: &str = "Invalid semver string";
const INVALID_PRE_RELEASE_STRING: &str = "Invalid pre-release string";

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "A simple CLI tool to increase semver versions, including pre-release tags."
)]
#[clap(group = ArgGroup::new("increase").required(true).args(&["major", "minor", "patch", "pre_release", "pre_release_patch"]))]
pub(crate) struct Cli {
    /// Returns the next major version
    #[clap(long, short = 'x')]
    pub(crate) major: Option<String>,

    /// Returns the next minor version
    #[clap(long, short = 'y')]
    pub(crate) minor: Option<String>,

    /// Returns the next patch version
    #[clap(long, short = 'z')]
    pub(crate) patch: Option<String>,

    /// Returns the next pre-release version
    #[clap(long, short = 'r')]
    pub(crate) pre_release: Option<String>,

    /// Returns the next pre-release patch version.
    /// The next pre-release version after stable is alpha
    #[clap(long, short = 'p')]
    pub(crate) pre_release_patch: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum IncreaseType {
    Major,
    Minor,
    Patch,
    PreRelease,
    PreReleasePatch,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PrereleaseType {
    Alpha,
    Beta,
    ReleaseCandidate,
    Stable,
}

impl PrereleaseType {
    pub fn next_prerelease(&self) -> Self {
        match self {
            PrereleaseType::Alpha => PrereleaseType::Beta,
            PrereleaseType::Beta => PrereleaseType::ReleaseCandidate,
            PrereleaseType::ReleaseCandidate => PrereleaseType::Stable,
            PrereleaseType::Stable => PrereleaseType::Alpha,
        }
    }
}

impl FromStr for PrereleaseType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_ascii_lowercase();
        let tmp_str = binding.as_str();
        let dot_pos = tmp_str.find('.').unwrap_or(tmp_str.len());
        match tmp_str[..dot_pos].to_owned().as_str() {
            "alpha" => Ok(PrereleaseType::Alpha),
            "beta" => Ok(PrereleaseType::Beta),
            "releasecandidate" | "release-candidate" | "release_candidate" | "rc" => {
                Ok(PrereleaseType::ReleaseCandidate)
            }
            "stable" | "" => Ok(PrereleaseType::Stable),
            _ => Err(()),
        }
    }
}

impl Display for PrereleaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrereleaseType::Alpha => write!(f, "alpha"),
            PrereleaseType::Beta => write!(f, "beta"),
            PrereleaseType::ReleaseCandidate => write!(f, "rc"),
            PrereleaseType::Stable => write!(f, ""),
        }
    }
}

pub(crate) trait Next {
    fn next(&self, kind: &IncreaseType) -> Result<Version, ()>;
}

impl Next for Version {
    fn next(&self, kind: &IncreaseType) -> Result<Self, ()> {
        match kind {
            IncreaseType::Major => Ok(next_major(self)),
            IncreaseType::Minor => Ok(next_minor(self)),
            IncreaseType::Patch => Ok(next_patch(self)),
            IncreaseType::PreRelease => next_pre_release(self),
            IncreaseType::PreReleasePatch => next_pre_release_patch(self),
        }
    }
}

fn next_patch(version: &Version) -> Version {
    let mut next_version = version.clone();
    next_version.patch += 1;
    next_version.pre = Prerelease::EMPTY;
    next_version
}

fn next_minor(version: &Version) -> Version {
    let mut next_version = version.clone();
    next_version.minor += 1;
    next_version.patch = 0;
    next_version.pre = Prerelease::EMPTY;
    next_version
}

fn next_major(version: &Version) -> Version {
    let mut next_version = version.clone();
    next_version.major += 1;
    next_version.minor = 0;
    next_version.patch = 0;
    next_version.pre = Prerelease::EMPTY;
    next_version
}

fn next_pre_release(version: &Version) -> Result<Version, ()> {
    let mut next_version = version.clone();
    let pre_str = next_version.pre.as_str().to_string();
    let prerelease_dot_pos = pre_str.rfind('.').unwrap_or(pre_str.len());
    let compare_str = &pre_str[..prerelease_dot_pos];
    let prerelease_type = PrereleaseType::from_str(compare_str)?;
    match prerelease_type {
        PrereleaseType::Alpha | PrereleaseType::Beta => {
            next_version.pre =
                Prerelease::from_str(format!("{}.0", prerelease_type.next_prerelease()).as_str())
                    .expect("Could not create prelease tag.");
        }
        PrereleaseType::ReleaseCandidate => next_version.pre = Prerelease::EMPTY,
        PrereleaseType::Stable => {
            next_version.patch += 1;
            next_version.pre =
                Prerelease::from_str(format!("{}.0", prerelease_type.next_prerelease()).as_str())
                    .expect("Could not create prelease tag.");
        }
    }
    Ok(next_version)
}

fn next_pre_release_patch(version: &Version) -> Result<Version, ()> {
    let mut next_version = version.clone();
    let mut pre_str = next_version.pre.as_str().to_string();
    if !pre_str.is_empty() {
        let dot_pos = if let Some(pos) = pre_str.rfind('.') {
            pos + 1
        } else {
            eprintln!("{INVALID_PRE_RELEASE_STRING}: '{pre_str}'.");
            return Err(());
        };
        let Ok(mut num) = pre_str[dot_pos..].parse::<usize>() else {
            eprintln!("{INVALID_PRE_RELEASE_STRING}: '{pre_str}'.");
            return Err(());
        };
        num += 1;
        let new_ver = num.to_string();
        pre_str.replace_range(dot_pos.., new_ver.as_str());
        next_version.pre = Prerelease::new(pre_str.as_str()).expect("");
        return Ok(next_version);
    }
    // This is the stable case, so we need to go to alpha and the next patch
    next_version.pre =
        Prerelease::from_str(format!("{}.0", PrereleaseType::Stable.next_prerelease()).as_str())
            .expect("Could not create prelease tag.");
    next_version.patch += 1;
    Ok(next_version)
}

#[cfg(test)]
mod test {
    use {
        crate::semver_next::{IncreaseType, Next, PrereleaseType},
        lazy_static::lazy_static,
        pretty_assertions::assert_eq,
        semver::{BuildMetadata, Prerelease, Version},
        std::str::FromStr,
    };

    lazy_static! {
        /// 1.2.3-alpha.4
        static ref TEST_STARTING_VERSION: Version = Version { major: 1, minor: 2, patch: 3, pre: Prerelease::new("alpha.4").unwrap(), build: BuildMetadata::EMPTY };
    }

    #[test]
    fn parsing() {
        let version = Version::parse("1.2.3.4");
        assert!(version.is_err());

        let version = Version::parse("1.2.3");
        assert!(version.is_ok());
        let version = Version::parse("1.2.3-4");
        assert!(version.is_ok());
        let version = Version::parse("1.2.3-alpha.4-");
        assert!(version.is_ok());

        let version = Version::parse("1.2.3-alpha.4-4.5");
        assert!(version.is_ok());
        let increase_result = version.unwrap().next(&IncreaseType::PreRelease);
        assert!(increase_result.is_ok());

        let version = Version::parse("1.2.3-alpha.4-4.5");
        assert!(version.is_ok());
        let increase_result = version.unwrap().next(&IncreaseType::PreReleasePatch);
        assert!(increase_result.is_ok());

        let version = Version::parse("1.2.3-alpha.4-4");
        assert!(version.is_ok());
        let increase_result = version.unwrap().next(&IncreaseType::PreReleasePatch);
        assert!(increase_result.is_err());
    }

    #[test]
    fn next_pre_patch() {
        let actual = TEST_STARTING_VERSION
            .next(&IncreaseType::PreReleasePatch)
            .unwrap();
        let expected = Version::parse("1.2.3-alpha.5").unwrap();
        assert_eq!(&expected, &actual);

        // invalid pre-release verion case fails
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Prerelease::new("--text--").unwrap(),
            build: BuildMetadata::EMPTY,
        };
        let result = version.next(&IncreaseType::PreReleasePatch);
        assert!(result.is_err());

        // creating new pre-release from stable
        let expected = Version::parse("1.2.4-alpha.0").unwrap();
        let actual = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
        .next(&IncreaseType::PreReleasePatch)
        .unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn next_pre() {
        let expected = Version::parse("1.2.3-beta.0").unwrap();
        let actual = TEST_STARTING_VERSION
            .next(&IncreaseType::PreRelease)
            .unwrap();
        assert_eq!(&expected, &actual);

        let expected = Version::parse("1.2.3-rc.0").unwrap();
        let actual = actual.next(&IncreaseType::PreRelease).unwrap();
        assert_eq!(&expected, &actual);

        let expected = Version::parse("1.2.3").unwrap();
        let actual = actual.next(&IncreaseType::PreRelease).unwrap();
        assert_eq!(&expected, &actual);

        let expected = Version::parse("1.2.4-alpha.0").unwrap();
        let actual = actual.next(&IncreaseType::PreRelease).unwrap();
        assert_eq!(&expected, &actual);

        // can not create pre-release, if pre-release tag is malformed.
        let invalid_version = Version::parse("1.2.4-foo").unwrap();
        let result = invalid_version.next(&IncreaseType::PreRelease);
        assert!(result.is_err());

        let version_str = "rc";
        let actual = PrereleaseType::from_str(version_str);
        assert!(actual.is_ok());

        let expected = PrereleaseType::Stable;
        let actual = actual.unwrap().next_prerelease();
        assert_eq!(expected, actual);

        let expected = "";
        let actual = actual.to_string();
        assert_eq!(expected, &actual);
    }

    #[test]
    fn next_patch() {
        let expected = Version::parse("1.2.4").unwrap();
        let actual = TEST_STARTING_VERSION.next(&IncreaseType::Patch).unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn next_minor() {
        let expected = Version::parse("1.3.0").unwrap();
        let actual = TEST_STARTING_VERSION.next(&IncreaseType::Minor).unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn next_major() {
        let expected = Version::parse("2.0.0").unwrap();
        let actual = TEST_STARTING_VERSION.next(&IncreaseType::Major).unwrap();
        assert_eq!(&expected, &actual);
    }
}
