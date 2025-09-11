# Change Log
All notable changes to this project will be documented in this file.

## [Unreleased]
### Added
- Ability to change the application's accent color
- Ability to change row height

## [1.4.0] - 2025-09-09
### Added
- Parse hosts from `Include` directive.
- Parse additional ssh config options: `identitiesonly`, `localforward`, `remoteforward`, `dynamicforward`, `clearallforwardings`, `exitonforwardfailure`, `forwardagent`, `forwardx11`, `forwardx11timeout`, `forwardx11trusted`, `serveralivecountmax`, `serveraliveinterval`, `gatewayports`, `proxyjump`, `passwordauthentication`, `pubkeyauthentication`, `stricthostkeychecking`, `connecttimeout`, `controlmaster`, `controlpath`, `controlpersist`, `compression`.
### Fixed
- Options order
- Table constraints

## [1.3.0] - 2025-08-28
### Added
- Search
### Changed
- Reorganized menu
### Fixed
- Minor bugs

## [1.2.0] - 2025-08-09
### Added
- Connection copying
- Remote command execution

## [1.1.0] - 2025-08-03
### Added
- Support for importing from ~/.ssh/config
### Fixed
- Configuration file paths on Windows
- Terminal cursor visibility

## [1.0.1] - 2025-07-29
### Changed
- Table constraints
### Fixed
- Error handling

## [1.0.0] - 2025-07-24
Initial release
