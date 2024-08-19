use {
    crate::semver_next::{Next, INVALID_SEMVER_STRING},
    clap::Parser,
    semver::Version,
    semver_next::IncreaseType,
    std::process,
};

mod semver_next;

fn main() {
    let args = semver_next::Cli::parse();

    let mut input = String::new();
    let mut kind = IncreaseType::PreReleasePatch;

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

    let mut v_prefix_char: Option<char> = None;
    if input.to_ascii_lowercase().starts_with('v') {
        v_prefix_char = Some(input.remove(0));
    }

    let Ok(parsed) = Version::parse(&input) else {
        eprintln!("{INVALID_SEMVER_STRING}");
        process::exit(1);
    };

    if let Ok(inc) = parsed.next(&kind) {
        if v_prefix_char.is_some() {
            print!("{}", v_prefix_char.unwrap());
        }
        println!("{inc}");
    }
}
