use std::{fmt::Display, process::Command};

use crate::{
    AvailablePackageManager, Cmd, Package, PackageManager, PackageManagerCommands, PkgFormat,
};

/// Wrapper for the Homebrew package manager.
///
/// [Homebrew — The Missing Package Manager for macOS (or Linux)](https://brew.sh/)
#[derive(Debug, Default)]
pub struct Homebrew;

impl PackageManager for Homebrew {
    fn pkg_delimiter(&self) -> char {
        '@'
    }

    fn pkg_manager_name(&self) -> String {
        AvailablePackageManager::Brew.to_string().to_lowercase()
    }

    fn supported_pkg_formats(&self) -> Vec<PkgFormat> {
        vec![PkgFormat::Bottle]
    }
}

impl PackageManagerCommands for Homebrew {
    fn cmd(&self) -> Command {
        Command::new("brew")
    }

    fn get_cmds(&self, cmd: Cmd, _pkg: Option<&Package>) -> Vec<String> {
        match cmd {
            Cmd::Install => vec!["install"],
            Cmd::Uninstall => vec!["uninstall"],
            Cmd::Update | Cmd::UpdateAll => vec!["upgrade"],
            Cmd::List => vec!["list"],
            Cmd::Sync => vec!["update"],
            Cmd::AddRepo => vec!["tap"],
            Cmd::Search => vec!["search"],
            Cmd::Outdated => vec!["outdated"],
        }
        .iter()
        .map(|x| x.to_string())
        .collect()
    }
}

impl Display for Homebrew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Homebrew")
    }
}

#[cfg(test)]
mod tests {
    #![cfg(target_os = "macos")]
    use super::*;
    use crate::{Operation, PackageManager};

    #[test]
    fn test_homebrew() {
        let hb = Homebrew;
        // sync
        assert!(hb.sync().success());
        // search
        assert!(hb.search("hello").iter().any(|p| p.name() == "hello"));

        let mut pkg = "hello".into();
        // install
        assert!(hb
            .execute_pkg_command(&mut pkg, Operation::Install, false)
            .success());
        // list
        assert!(hb.list_installed().iter().any(|p| p.name() == "hello"));
        // update
        assert!(hb
            .execute_pkg_command(&mut pkg, Operation::Update, false)
            .success());
        // uninstall
        assert!(hb
            .execute_pkg_command(&mut pkg, Operation::Uninstall, false)
            .success());
        // TODO: Test AddRepo
    }
}
