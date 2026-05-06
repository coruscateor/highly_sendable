# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (__/04/2025)

### Added

commit 1bac7a0ee5028be16683607af850321348936014

- Added the BasicStatusWithItem struct.

commit 6933c9676959046a230a79265b02c56b77138c6e

-- Added rust-version to the package fields.

- Added the rust-version entry to the package fields.

- Added documentation

- Added the BasicStatus and BasicStatusU32 enums.

commit 7ad030b33a8e795240f2f1acb6b815f099278944

- Made the as_str method public in the text::SendableText implementation.
    
-- Made the as_str method public in the text::SendableTextImmut implementation.

Added in this version.

commit b1bb9f994d62038efa205c5d7ce5298159b856f3

- Added the WorkInProgressMessageOption and WorkInProgressMessageResult types.

commit 6269a93830413deca4c8586fccd4f45635d21248

 -- Renamed the WorkInProgress struct to WorkInProgressMessage.

Renamed - Added in this version.

commit 0d241dfb105d31df8977ddbdf53dd3a73a25a743

- Added the accessorise, pastey and inc_dec dependencies.

- Added the ConnectionMessage struct.

- Added the ConnectionStateId struct.

- Added the EssentialStatus enum.

- Added the PauseableStatus enum.

-- Added the WorkInProgress struct.

Renamed - Added in this version.

- Added the WorkInProgressMessage struct.

commit 7264154696dfca38a58afa359ac7ec72292e04d2

- Added the SendableTextImmut enum to the text module.

-- Added the WorkInProgressResultIntPercentage and IdedWorkInProgressResultIntPercentage structs.

Then removed

commit 4f3f9d290368862d7bdf1be2acc53c6721fb0c9e

- Added a package.metadata.docs.rs section to the cargo file with the necessary details for rustdoc to label optional features on docs.rs.

commit d172d00a4f67512658d31ce846ff56a15dbc4bef

- Added the license metadata field.



### Changed

commit 1bac7a0ee5028be16683607af850321348936014

- Updated the package description.

- Updated the readme.
    
-- BasicStatusU32 is now a type alias.

Added in this version.

commit 6933c9676959046a230a79265b02c56b77138c6e

-- Updated the package version string to “0.2.0-beta".

- Updated the delegate dependency to version 0.13.5.

-- Updated the pastey dependency to version 0.2.2.

Added in this version.

-- ConnectionStateId now conditionally derives the serde Serialize and Deserialize traits.

Added in this version.

- Replaced the cfg_if macro call with cfg_select in the sendable_text module.
    
- Replaced the cfg_if macro call with cfg_select in the sendable_text_immut module

-- Replaced the EssentialStatus default type of the S generic parameter of the WorkInProgressMessage struct and related types with BasicStatus.

Added in this version.

commit b1bb9f994d62038efa205c5d7ce5298159b856f3

- Updated the minimum expected version of the serde dependency to 1.0.228.

- Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the lib file.

commit 7264154696dfca38a58afa359ac7ec72292e04d2

-- Renamed the result methods of WorkInProgressResult and IdedWorkInProgressResult to result_ref.

Added in this version.

-- WorkInProgressResult and IdedWorkInProgressResult now implement the Clone and Default traits.

Added in this version.

-- Disabled the result module.

Removed in this version.

-- Moved the WorkInProgressResult and IdedWorkInProgressResult structs to the crate level module.

Removed in this version.

commit 4f3f9d290368862d7bdf1be2acc53c6721fb0c9e

- Updated the crate version string to 0.2.0-alpha.

commit 4f3f9d290368862d7bdf1be2acc53c6721fb0c9e

-- Renamed the done_none method to none_done and the not_done_none method to none_not_done in the WorkInProgressResult and IdedWorkInProgressResult structs.

Removed in this version.



### Deprecated



### Removed

commit 6933c9676959046a230a79265b02c56b77138c6e

- Removed the cfg-if dependency.

commit 6269a93830413deca4c8586fccd4f45635d21248

- Removed the result module.

commit 0d241dfb105d31df8977ddbdf53dd3a73a25a743

-- Removed the WorkInProgressResult and IdedWorkInProgressResult structs.

-- Removed the WorkInProgressResult and IdedWorkInProgressResult structs that were part of the result module.

Added in this version.

commit 4f3f9d290368862d7bdf1be2acc53c6721fb0c9e

- Removed the cfg feature decoration from the sendable_text sub-module use statement in the text module mod file.



### Fixed

commit 7cf1b9d5beab7332329b73b3465b8a4442e3f96d

-- Fixed a build error.

No longer relevant.



### Security



## Version 0.1.0 (01/04/2025)

- Initial release


