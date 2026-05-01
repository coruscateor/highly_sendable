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



### Deprecated



### Removed

commit 6933c9676959046a230a79265b02c56b77138c6e

- Removed the cfg-if dependency.

commit 6269a93830413deca4c8586fccd4f45635d21248

- Removed the result module.



### Fixed



### Security



## Version 0.1.0 (01/04/2025)

- Initial release


