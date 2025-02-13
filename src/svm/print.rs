use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use semver::Version;

pub fn current_version(version: Option<Version>) {
    match version {
        Some(v) => {
            println!("{} (current)", style(v.to_string().as_str()).green());
        }
        None => {
            println!("Global version not set");
        }
    }
}

pub fn installed_versions(versions: Vec<Version>) {
    println!("\n{}", style("Installed Versions").bold());
    versions.iter().for_each(|v| {
        println!("{}", style(v.to_string().as_str()).yellow());
    });
}

pub fn available_versions(versions: Vec<Version>) {
    println!("\n{}", style("Available to Install").bold());
    let groups = versions
        .iter()
        .group_by(|v| v.minor)
        .into_iter()
        .map(|(_, g)| g.cloned().collect())
        .collect::<Vec<Vec<Version>>>();
    for group in groups {
        println!(
            "{:?}",
            group.iter().map(|v| v.to_string()).collect::<Vec<String>>()
        );
    }
}

pub fn installing_version(version: &Version) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(120);
    spinner.set_message(format!("Downloading Solc {}", version.to_string()));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️ ", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ",
                "⛈ ", "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️ ", "🌤 ", "☀️ ", "☀️ ",
            ])
            .template("{spinner:.green} {msg}"),
    );
    spinner
}

pub fn unsupported_version(version: &Version) {
    println!(
        "{}",
        style(format!("Version: {} unsupported", version.to_string())).red()
    );
}

pub fn set_global_version(version: &Version) {
    ProgressBar::new_spinner()
        .finish_with_message(format!("Global version set: {}", version.to_string()));
}

pub fn version_not_found(version: &Version) {
    println!(
        "{}",
        style(format!("Version: {} not found", version.to_string())).red()
    );
}
