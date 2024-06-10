use {
    clap::{ArgGroup, Parser},
    semver::{Prerelease, Version},
    std::{fmt::Display, process, str::FromStr},
};

pub(crate) const INVALID_SEMVER_STRING: &str = "Invalid semver string";
const INVALID_PRE_RELEASE_STRING: &str = "Invalid pre-release string";

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[clap(name = "Semver Increaser", version = "0.2.0", about = "A tool to increase version numbers")]
#[clap(group = ArgGroup::new("increase").required(true).args(&["major", "minor", "patch", "pre_release", "pre_release_patch"]))]
pub(crate) struct Cli {
    /// Increase the major version
    #[clap(long)]
    pub(crate) major: Option<String>,

    /// Increase the minor version
    #[clap(long)]
    pub(crate) minor: Option<String>,

    /// Increase the patch version
    #[clap(long)]
    pub(crate) patch: Option<String>,

    /// Increase the pre-release version
    #[clap(long)]
    pub(crate) pre_release: Option<String>,

    /// Increase the pre-release patch version.
    /// The next pre-release version after stable is alpha
    #[clap(long)]
    pub(crate) pre_release_patch: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum IncreaseType {
    Unknown,
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
            "releasecandidate" | "release-candidate" | "release_candidate" | "rc" => Ok(PrereleaseType::ReleaseCandidate),
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

pub(crate) trait Increase {
    fn increase(&self, kind: &IncreaseType) -> Result<Version, ()>;
}

impl Increase for Version {
    fn increase(&self, kind: &IncreaseType) -> Result<Version, ()> {
        match kind {
            IncreaseType::Unknown => {
                eprintln!("Unknown increase type");
                process::exit(1);
            }
            IncreaseType::Major => Ok(increase_major(self)),
            IncreaseType::Minor => Ok(increase_minor(self)),
            IncreaseType::Patch => Ok(increase_patch(self)),
            IncreaseType::PreRelease => increase_pre_release(self),
            IncreaseType::PreReleasePatch => increase_pre_release_patch(self),
        }
    }
}

fn increase_patch(version: &Version) -> Version {
    let mut inc = version.clone();
    inc.patch += 1;
    inc.pre = Prerelease::EMPTY;
    inc
}

fn increase_minor(version: &Version) -> Version {
    let mut inc = version.clone();
    inc.minor += 1;
    inc.patch = 0;
    inc.pre = Prerelease::EMPTY;
    inc
}

fn increase_major(version: &Version) -> Version {
    let mut inc = version.clone();
    inc.major += 1;
    inc.minor = 0;
    inc.patch = 0;
    inc.pre = Prerelease::EMPTY;
    inc
}

fn increase_pre_release(version: &Version) -> Result<Version, ()> {
    let mut inc = version.clone();
    let pre_str = inc.pre.as_str().to_string();
    let dot_pos = pre_str.rfind('.').unwrap_or(pre_str.len());
    let compare_str = &pre_str[..dot_pos];
    let p_type = PrereleaseType::from_str(compare_str)?;
    match p_type {
        PrereleaseType::Alpha | PrereleaseType::Beta => {
            inc.pre = Prerelease::from_str(format!("{}.0", p_type.next_prerelease()).as_str()).expect("Could not create prelease tag.");
        }
        PrereleaseType::ReleaseCandidate => inc.pre = Prerelease::EMPTY,
        PrereleaseType::Stable => {
            inc.patch += 1;
            inc.pre = Prerelease::from_str(format!("{}.0", p_type.next_prerelease()).as_str()).expect("Could not create prelease tag.");
        }
    }
    Ok(inc)
}

fn increase_pre_release_patch(version: &Version) -> Result<Version, ()> {
    let mut inc = version.clone();
    let mut pre_str = inc.pre.as_str().to_string();
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
        inc.pre = Prerelease::new(pre_str.as_str()).expect("");
        return Ok(inc);
    }
    // This is the stable case, so we need to go to alpha and the next patch
    inc.pre = Prerelease::from_str(format!("{}.0", PrereleaseType::Stable.next_prerelease()).as_str()).expect("Could not create prelease tag.");
    inc.patch += 1;
    Ok(inc)
}

#[cfg(test)]
mod test {
    use {
        crate::semver_inc::{Increase, IncreaseType},
        lazy_static::lazy_static,
        pretty_assertions::assert_eq,
        semver::{BuildMetadata, Prerelease, Version},
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
        let increase_result = version.unwrap().increase(&IncreaseType::PreRelease);
        assert!(increase_result.is_ok());

        let version = Version::parse("1.2.3-alpha.4-4.5");
        assert!(version.is_ok());
        let increase_result = version.unwrap().increase(&IncreaseType::PreReleasePatch);
        assert!(increase_result.is_ok());

        let version = Version::parse("1.2.3-alpha.4-4");
        assert!(version.is_ok());
        let increase_result = version.unwrap().increase(&IncreaseType::PreReleasePatch);
        assert!(increase_result.is_err());
    }

    #[test]
    fn inc_pre_patch() {
        let actual = TEST_STARTING_VERSION.increase(&IncreaseType::PreReleasePatch).unwrap();
        let expected = Version::parse("1.2.3-alpha.5").unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn inc_pre() {
        let actual = TEST_STARTING_VERSION.increase(&IncreaseType::PreRelease).unwrap();
        let expected = Version::parse("1.2.3-beta.0").unwrap();
        assert_eq!(&expected, &actual);

        let actual = actual.increase(&IncreaseType::PreRelease).unwrap();
        let expected = Version::parse("1.2.3-rc.0").unwrap();
        assert_eq!(&expected, &actual);

        let actual = actual.increase(&IncreaseType::PreRelease).unwrap();
        let expected = Version::parse("1.2.3").unwrap();
        assert_eq!(&expected, &actual);

        let actual = actual.increase(&IncreaseType::PreRelease).unwrap();
        let expected = Version::parse("1.2.4-alpha.0").unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn inc_patch() {
        let actual = TEST_STARTING_VERSION.increase(&IncreaseType::Patch).unwrap();
        let expected = Version::parse("1.2.4").unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn inc_minor() {
        let actual = TEST_STARTING_VERSION.increase(&IncreaseType::Minor).unwrap();
        let expected = Version::parse("1.3.0").unwrap();
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn inc_major() {
        let actual = TEST_STARTING_VERSION.increase(&IncreaseType::Major).unwrap();
        let expected = Version::parse("2.0.0").unwrap();
        assert_eq!(&expected, &actual);
    }
}
