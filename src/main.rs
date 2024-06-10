use {
    crate::semver_inc::{Increase, INVALID_SEMVER_STRING},
    clap::Parser,
    semver::Version,
    semver_inc::IncreaseType,
    std::process,
};

mod semver_inc;

fn main() {
    let args = semver_inc::Cli::parse();

    // Unwrapping is safe, since we checked values beforehand.
    let mut input = String::new();
    let mut kind = IncreaseType::Unknown;

    if args.major.is_some() {
        input = args.major.unwrap();
        kind = IncreaseType::Major;
    } else if args.minor.is_some() {
        input = args.minor.unwrap();
        kind = IncreaseType::Minor;
    } else if args.patch.is_some() {
        input = args.patch.unwrap();
        kind = IncreaseType::Patch;
    } else if args.pre_release.is_some() {
        input = args.pre_release.unwrap();
        kind = IncreaseType::PreRelease;
    } else if args.pre_release_patch.is_some() {
        input = args.pre_release_patch.unwrap();
        kind = IncreaseType::PreReleasePatch;
    }

    let mut v_prefix = false;
    if input.starts_with('v') {
        v_prefix = true;
        input.remove(0);
    }

    let Ok(parsed) = Version::parse(&input) else {
        eprintln!("{INVALID_SEMVER_STRING}");
        process::exit(1);
    };

    if let Ok(inc) = parsed.increase(&kind) {
        if v_prefix {
            print!("v");
        }
        println!("{inc}");
    }
}
