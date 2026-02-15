//! Generate a Markdown-compatible reference for the kount command-line interface.
use std::cmp::max;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Command as ClapCommand, CommandFactory};
use itertools::Itertools;
use kount_cli::Args as KountArgs;
use pretty_assertions::StrComparison;

use crate::{Mode, ROOT_DIR};

#[derive(clap::Args)]
pub struct CliArgs {
    #[arg(long, default_value_t, value_enum)]
    pub(crate) mode: Mode,
}

pub(crate) fn main(args: &CliArgs) -> Result<()> {
    let reference_string = generate();
    let filename = "cli.md";
    let reference_path = PathBuf::from(ROOT_DIR)
        .join("docs")
        .join("reference")
        .join(filename);

    if !reference_path.exists() {
        fs_err::write(&reference_path, "")?;
    }

    match args.mode {
        Mode::DryRun => {
            println!("{reference_string}");
        }
        Mode::Check => match fs_err::read_to_string(reference_path) {
            Ok(current) => {
                if current == reference_string {
                    println!("Up-to-date: {filename}");
                } else {
                    let comparison = StrComparison::new(&current, &reference_string);
                    bail!(
                        "{filename} changed, please run `{}`:\n{comparison}",
                        crate::REGENERATE_ALL_COMMAND
                    );
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                bail!(
                    "{filename} not found, please run `{}`",
                    crate::REGENERATE_ALL_COMMAND
                );
            }
            Err(err) => {
                bail!(
                    "{filename} changed, please run `{}`:\n{err}",
                    crate::REGENERATE_ALL_COMMAND
                );
            }
        },
        Mode::Write => match fs_err::read_to_string(&reference_path) {
            Ok(current) => {
                if current == reference_string {
                    println!("Up-to-date: {filename}");
                } else {
                    println!("Updating: {filename}");
                    fs_err::write(reference_path, reference_string.as_bytes())?;
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!("Updating: {filename}");
                fs_err::write(reference_path, reference_string.as_bytes())?;
            }
            Err(err) => {
                bail!(
                    "{filename} changed, please run `{}`:\n{err}",
                    crate::REGENERATE_ALL_COMMAND
                );
            }
        },
    }

    Ok(())
}

fn generate() -> String {
    let mut output = String::new();

    let mut kount = KountArgs::command();
    kount.build();

    let mut parents = Vec::new();

    output.push_str("# CLI Reference\n\n");
    generate_command(&mut output, &kount, &mut parents);

    output
}

#[allow(clippy::format_push_string)]
fn generate_command<'a>(
    output: &mut String,
    command: &'a ClapCommand,
    parents: &mut Vec<&'a ClapCommand>,
) {
    if command.is_hide_set() {
        return;
    }

    let name = if parents.is_empty() {
        command.get_name().to_string()
    } else {
        format!(
            "{} {}",
            parents.iter().map(|cmd| cmd.get_name()).join(" "),
            command.get_name()
        )
    };

    let level = max(2, parents.len() + 1);
    output.push_str(&format!("{} {name}\n\n", "#".repeat(level)));

    if let Some(about) = command.get_long_about().or_else(|| command.get_about()) {
        output.push_str(&about.to_string());
        output.push_str("\n\n");
    }

    // Usage
    {
        let mut command = command.clone();
        output.push_str("<h3 class=\"cli-reference\">Usage</h3>\n\n");
        output.push_str(&format!(
            "```\n{}\n```",
            command
                .render_usage()
                .to_string()
                .trim_start_matches("Usage: "),
        ));
        output.push_str("\n\n");
    }

    let name_key = name.replace(' ', "-");

    // Positional arguments
    let mut arguments = command
        .get_positionals()
        .filter(|arg| !arg.is_hide_set())
        .peekable();

    if arguments.peek().is_some() {
        output.push_str("<h3 class=\"cli-reference\">Arguments</h3>\n\n");
        output.push_str("<dl class=\"cli-reference\">");

        for arg in arguments {
            let id = format!("{name_key}--{}", arg.get_id());
            output.push_str(&format!("<dt id=\"{id}\">"));
            output.push_str(&format!(
                "<a href=\"#{id}\"><code>{}</code></a>",
                arg.get_id().to_string().to_uppercase(),
            ));
            output.push_str("</dt>");
            if let Some(help) = arg.get_long_help().or_else(|| arg.get_help()) {
                output.push_str("<dd>");
                output.push_str(&format!("{}\n", markdown::to_html(&help.to_string())));
                output.push_str("</dd>");
            }
        }

        output.push_str("</dl>\n\n");
    }

    // Options and flags
    let mut options = command
        .get_arguments()
        .filter(|arg| !arg.is_positional())
        .filter(|arg| !arg.is_hide_set())
        .sorted_by_key(|arg| arg.get_id())
        .peekable();

    if options.peek().is_some() {
        output.push_str("<h3 class=\"cli-reference\">Options</h3>\n\n");
        output.push_str("<dl class=\"cli-reference\">");
        for opt in options {
            let Some(long) = opt.get_long() else { continue };
            let id = format!("{name_key}--{long}");

            output.push_str(&format!("<dt id=\"{id}\">"));
            output.push_str(&format!("<a href=\"#{id}\"><code>--{long}</code></a>"));
            if let Some(short) = opt.get_short() {
                output.push_str(&format!(", <code>-{short}</code>"));
            }

            if opt
                .get_num_args()
                .unwrap_or_else(|| 1.into())
                .takes_values()
            {
                if let Some(values) = opt.get_value_names() {
                    for value in values {
                        output.push_str(&format!(
                            " <i>{}</i>",
                            value.to_lowercase().replace('_', "-")
                        ));
                    }
                }
            }

            output.push_str("</dt>");
            if let Some(help) = opt.get_long_help().or_else(|| opt.get_help()) {
                output.push_str("<dd>");
                output.push_str(&format!("{}\n", markdown::to_html(&help.to_string())));
                emit_default_option(opt, output);
                emit_possible_options(opt, output);
                output.push_str("</dd>");
            }
        }

        output.push_str("</dl>");
    }

    output.push_str("\n\n");

    parents.push(command);

    for subcommand in command.get_subcommands() {
        generate_command(output, subcommand, parents);
    }

    parents.pop();
}

fn emit_default_option(opt: &clap::Arg, output: &mut String) {
    if opt.is_hide_default_value_set() || !opt.get_num_args().expect("built").takes_values() {
        return;
    }

    let values = opt.get_default_values();
    if !values.is_empty() {
        let value = format!(
            "\n[default: {}]",
            opt.get_default_values()
                .iter()
                .map(|s| s.to_string_lossy())
                .join(",")
        );
        output.push_str(&markdown::to_html(&value));
    }
}

fn emit_possible_options(opt: &clap::Arg, output: &mut String) {
    if opt.is_hide_possible_values_set() {
        return;
    }

    let values = opt.get_possible_values();
    if !values.is_empty() {
        let value = format!(
            "\nPossible values:\n{}",
            values
                .into_iter()
                .filter(|value| !value.is_hide_set())
                .map(|value| {
                    let name = value.get_name();
                    value.get_help().map_or_else(
                        || format!(" - `{name}`"),
                        |help| format!(" - `{name}`:  {help}"),
                    )
                })
                .collect_vec()
                .join("\n"),
        );
        output.push_str(&markdown::to_html(&value));
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{main, CliArgs, Mode};

    #[test]
    #[cfg(unix)]
    fn cli_reference_is_up_to_date() -> Result<()> {
        main(&CliArgs { mode: Mode::Check })
    }
}
