use clap::{Parser, ValueEnum};
use std::{env, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Generate a complete and ready-to-use favicons for your websites
pub struct Args {
    // Image source
    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath, help = "Path to the source image")]
    pub source: PathBuf,

    // Platforms that should be supported
    #[arg(
        short = 'p',
        long,
        value_name = "platforms",
        value_enum,
        help = "Platforms that should be supported"
    )]
    pub platforms: Option<Vec<Platform>>,

    // Output folder
    #[arg(value_name = "output", short = 'o', long, value_hint = clap::ValueHint::DirPath, help = "Output folder destination, will be created if it does not exist")]
    pub output: Option<PathBuf>,

    // Generate HTML template
    #[arg(
        short = 't',
        long,
        default_value_t = false,
        help = "Generate a quick-start HTML template"
    )]
    pub template: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum Platform {
    // Favicons that are compatible with almost all web browsers
    Web,
    // Include favicons features that are supported by major modern browsers
    Modern,
    // Enable Android-based favicon support. Includes manifest.
    Android,
    // Enable Apple-based device favicon support.
    Apple,
}

// Validate shell arguments and assign defaults if `None`
pub fn validate_args(mut args: Args) -> Result<Args, String> {
    if !args.source.exists() {
        return Err("Source file does not exists".to_string());
    }

    if args.platforms.is_none() {
        args.platforms = Option::from(Vec::from([
            Platform::Web,
            Platform::Modern,
            Platform::Android,
            Platform::Apple,
        ]));
    }

    if args.output.is_none() {
        let cwd = env::current_dir().unwrap();
        let mut output_path = PathBuf::new();

        output_path.push(cwd);
        output_path.push("output");

        args.output = Option::from(output_path);
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_default_platforms() {
        let args = Args {
            source: PathBuf::from("samples/sample.svg"),
            platforms: Option::None,
            output: Option::from(PathBuf::from("here")),
            template: false,
        };
        let result = validate_args(args);

        assert_eq!(result.is_err(), false);
        assert_eq!(
            result.unwrap().platforms.unwrap(),
            Vec::from([
                Platform::Web,
                Platform::Modern,
                Platform::Android,
                Platform::Apple,
            ])
        );
    }

    #[test]
    fn test_assign_default_output() {
        let args = Args {
            source: PathBuf::from("samples/sample.svg"),
            platforms: Option::from(Vec::from([Platform::Web, Platform::Modern])),
            output: Option::None,
            template: false,
        };
        let result = validate_args(args);

        let cwd = env::current_dir().unwrap();
        let mut path = PathBuf::new();

        path.push(cwd);
        path.push("output");

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap().output.unwrap().to_str(), path.to_str());
    }
}
