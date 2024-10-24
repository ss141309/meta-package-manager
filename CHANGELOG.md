
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.5] - 2024-10-18

- Adds MSRV to 1.70.0

## [0.7.4] - 2024-10-07

- Adds `--install-default` option to `managers` command. 
- Removes unneccessary use of `sudo` (#54).

## [0.7.3] - 2024-07-31

- Maintenance release.
- Adds CI to release binary.

## [0.7.2] - 2024-07-22

- Hotfix: Fix broken install of local deb packages

## [0.7.1] - 2024-07-12

- Update to dependencies.

## [0.7.0] - 2024-04-13

- Improves support for installing from URL (apt, dnf, zypper, yum)
- Improves code architecture (mostly refactor).

## [0.6.0] - 2024-04-07

- Adds support for installing from URL

## [0.5.0] - 2024-04-06

- Rewrite and fixes to tests and CICD pipeline.
- Adds support for writing output in `--json` format. 

## [0.4.0] - 2024-04-05

- Adds support for zypper (openSUSE).
- ManHandler renamed to PkgManagerHandler 

## [0.3.1] - 2024-01-04

- refactor: Merged workspace into single crate

## [0.3.0] - 2024-01-04

- Refactor: renamed mpm to libmpm and mpm-cli to mpm

## [0.2.0] - 2024-01-04

- Initial release
