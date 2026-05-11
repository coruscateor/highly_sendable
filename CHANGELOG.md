# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (11/05/2025)

### Added

- Added the BasicStatusWithItem struct.

- Added the rust-version entry to the package fields.

- Added documentation

- Added the BasicStatus and BasicStatusU32 enums.

- Made the as_str method public in the text::SendableText implementation.

- Added the WorkInProgressMessageOption and WorkInProgressMessageResult types.

- Added the accessorise, pastey and inc_dec dependencies.

- Added the ConnectionMessage struct.

- Added the ConnectionStateId struct.

- Added the EssentialStatus enum.

- Added the PauseableStatus enum.

- Added the WorkInProgressMessage struct.

- Added the SendableTextImmut enum to the text module.

- Added a package.metadata.docs.rs section to the cargo file with the necessary details for rustdoc to label optional features on docs.rs.

- Added the license metadata field.

- Added the authors field to the readme.



### Changed

- Updated the package description.

- Updated the readme.

- Updated the delegate dependency to version 0.13.5.

- Replaced the cfg_if macro call with cfg_select in the sendable_text module.
    
- Replaced the cfg_if macro call with cfg_select in the sendable_text_immut module

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the lib file.

- Various dependencies were updated via the “cargo update” command.

- Updated the package keywords.



### Removed

- Removed the cfg-if dependency.

- Removed the result module and its contents.

- Removed the cfg feature decoration from the sendable_text sub-module use statement in the text module mod file.



## Version 0.1.0 (01/04/2025)

- Initial release


