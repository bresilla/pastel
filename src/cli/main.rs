use ansi_term::Color as TermColor;
use clap::{
    crate_description, crate_name, crate_version, App as ClapApp, AppSettings, Arg, SubCommand,
};

mod app;
mod error;
mod hdcanvas;
mod parser;
mod termcolor;
mod utility;
mod x11colors;

use app::{Command, Config};
use error::{PastelError, Result};

type ExitCode = i32;

fn run() -> Result<ExitCode> {
    let color_arg = Arg::with_name("color")
        .help(
            "Colors can be specified in many different formats, such as #RRGGBB, RRGGBB, \
             #RGB, 'rgb(…, …, …)', 'hsl(…, …, …)' or simply by the name of the color. \
             If the color argument is not specified, the color will be read \
             from standard input.\n\
             Examples:\
             \n  - cyan\
             \n  - salmon\
             \n  - skyblue\
             \n  - '#ff0077'\
             \n  - f07\
             \n  - 'rgb(216, 180, 140)'\
             \n  - 'hsl(128, 100%, 54%)'",
        )
        .required(false);
    let app = ClapApp::new(crate_name!())
        .version(crate_version!())
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .max_term_width(100)
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("show")
                .about("Display information about the given color on the terminal")
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("pick")
                .about("Print a spectrum of colors to choose from")
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("saturate")
                .long_about(
                    "Increase the saturation of a color by adding a certain amount to the HSL \
                     saturation channel (a number between 0.0 and 1.0). If the amount is negative, \
                     the color will be desaturated."
                )
                .about("Increase color saturation by a specified amount")
                .arg(Arg::with_name("amount").help("amount of saturation to add").required(true))
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("desaturate")
                .about("Decrease color saturation by a specified amount")
                .long_about(
                    "Decrease the saturation of a color by subtracting a certain amount from the \
                     HSL saturation channel (a number between 0.0 and 1.0). If the amount is negative, \
                     the color will be saturated.

                     See also: to-gray"
                )
                .arg(Arg::with_name("amount").help("amount of saturation to subtract").required(true))
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("lighten")
                .long_about(
                    "Lighten a color by adding a certain amount to the HSL lightness channel (a \
                     number between 0.0 and 1.0). If the amount is negative, the color will be \
                     darkened.",
                )
                .about("Lighten color by a specified amount")
                .arg(Arg::with_name("amount").help("amount of lightness to add").required(true))
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("darken")
                .long_about(
                    "Darken a color by subtracting a certain amount from the lightness channel (a \
                     number between 0.0 and 1.0). If the amount is negative, the color will be \
                     lightened."
                )
                .about("Darken color by a specified amount")
                .arg(Arg::with_name("amount").help("amount of lightness to subtract").required(true))
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("rotate")
                .about("Rotate the hue channel by a specified angle")
                .long_about("Rotate the HSL hue channel of a color by the specified angle (in \
                             degrees). A rotation by 180° returns the complementary color. A \
                             rotation by 360° returns to the original color.")
                .arg(Arg::with_name("degrees").help("angle by which to rotate (in degrees)").required(true))
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("complement")
                .about("Get the complementary color (hue rotated by 180°)")
                .long_about("Compute the complementary color by rotating the HSL hue channel by 180°.")
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("to-gray")
                .about("Completely desaturate a color (while preserving luminance)")
                .long_about(
                    "Completely desaturate the given color while preserving the luminance.\n\
                     \n\
                     For a definition of 'luminance', see:\n\n  \
                       https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef")
                .arg(color_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Print a list of available color names")
                .arg(Arg::with_name("sort").short("s").long("sort").help("Sort order").possible_values(&["name", "brightness", "luminance", "hue", "chroma"]).default_value("hue"))
        )
        .subcommand(
            SubCommand::with_name("format")
                .about("Print a list of available color names")
                .arg(Arg::with_name("type").help("Format type").possible_values(&["rgb", "hsl", "hex"]).required(true))
                .arg(color_arg.clone())
        );

    let global_matches = app.get_matches();

    if let (subcommand, Some(matches)) = global_matches.subcommand() {
        let config = Config::new();
        let command = Command::from_string(subcommand);
        command.execute(matches, &config)?;
    } else {
        unreachable!("Subcommand is required");
    }

    Ok(0)
}

fn main() {
    let result = run();
    match result {
        Err(err) => {
            eprintln!(
                "{}: {}",
                TermColor::Red.paint("[pastel error]"),
                err.message()
            );
            std::process::exit(1);
        }
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
    }
}
