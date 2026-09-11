# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (__/09/2025)

### Added

commit 89ec0f214e8af1a1a8185ae69b47956da8db33d0

- Added the clone_if_not_string method to the text::SendableText enum.

- Added documentation

commit 0d3546f39a12332d5c05d619be70ee5de0a2e452

- Conditionally added the SendableRefVisitor struct.

commit a03fa40cd433603f518d22df18f14eff85f6bee4

-- Added implementer constrains, removed the buffer method and added put_buffer, put_opt_buffer, take_buffer and take_buffer_with_capacity methods to the VecBufferInputOutput trait.

Added in this version.

- Added the EmptyVecBufferInputOutput struct.

commit 40b19e23662db6a80b07c4c104d56e4f37838b9d

-- Added the VecBufferRetriever trait.

- Added the VecBufferInputOutput trait.

Renamed

-- The progress_item_ref and progress_item_mut methods have been added to the BasicStatus implementation.

Incorrect

- The progress_item_ref and progress_item_mut methods have been added to the BasicStatusWithItem implementation.

- Conditionally implemented the Default, PartialEq, Eq, and PartialOrd traits for the BasicStatusWithItem enum.

commit fda310016d1e5e9bf052816d11fa6420c951d32d

- Added the CowableRef enum.
    
- Added the SendableRef enum.

commit 600055f66e3761af84462016b80ab11aaddc0d75

- Added the len, capacity and len_is_at_capacity methods to the text::SendableText implementation.

commit 404f0f90797cc818f71e731eb61d0a42cf492439

- Added the VariableStateMessage struct.
    
- Added the VariableStateNumber struct.

commit bb127f243527c9d8b24e1b622c8246e5040b2707

-- Added string, str and arc_str methods to the SendableBytes enum.

Added in this version.

- Conditionally added the SendableBytesVisitor struct.

commit 01bee7bb399bca6d6b0fd1a432d7f02a1945da76

- Added the bytes crate as an optional dependency.
    
- Added the SendableBytes enum.

e299294e913b09f6623291293343ada57d3e2021

- Added an “AI Contribution Policy” section  to the readme.



### Changed

commit 89ec0f214e8af1a1a8185ae69b47956da8db33d0

-- Updated the package version to 0.3.0-beta.
    
- Updated the package description.

- Updated the serde dependency to version 1.0.229.

- Updated the pastey dependency to version 0.2.3.
    
- Updated the inc_dec dependency to version 0.2.0.

-- Prepared the changelog.

- Updated the readme.

-- Continued work on CowableRef.
    
-- Continued work on SendableBytes.
    
-- Continued work on SendableRef.

-- Updated documentation

-- Changed the name of the variable_state_number field and getter method to “value” in the VariableStateNumber struct definition and implementation.

Added in this version.

-- Other minor changes.

commit c7ecb31521426336688f3fa839687f39e5065340

- Updated the readme.

-- Continued work on VecBufferInputOutput and EmptyVecBufferInputOutput.

-- Continued work on CowableRef.

-- Continued work on SendableBytes.

-- Continued work on SendableRef.

- The text::SendableText enum now implements the PartialOrd and Ord traits.

- The type of the st_queue field of the text::SendableTextLog struct has been changed to std::collections::VecDeque.

-- The VariableStateMessage struct now implements Clone.

Added in this version.

commit 9381a523fb400a127116ae0d1f3450c2564a39f9

- Updated the accessorise dependency to version 0.3.0.

-- Continued work on the CowableRef, SendableBytes and SendableRef implementations.

- Updated some documentation.

-- Other minor changes.

commit 0d3546f39a12332d5c05d619be70ee5de0a2e452

-- Conditionally implemented the Serialize and Deserialize traits for the SendableRef enum.

Added in this version.

commit a03fa40cd433603f518d22df18f14eff85f6bee4

-- Renamed the VecBufferRetriever trait to VecBufferInputOutput.

Renamed

-- Continued work on the CowableRef and SendableRef enums.

Added in this version.

commit 40b19e23662db6a80b07c4c104d56e4f37838b9d

- The EssentialStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.

- The PauseableStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.

- The BasicStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.

-- Renamed the “N” generic parameter to “M” in the same_number method definition in the VariableStateMessage implementation.

Added in this version.

- Other minor changes.

Put last

commit 29925f714300f8e3eb3a82f7bf0e99b4bbf92f3c

-- Continued work on the CowableRef and the SendableRef enums.

commit 600055f66e3761af84462016b80ab11aaddc0d75

-- Continued work on the SendableBytes struct.

commit 404f0f90797cc818f71e731eb61d0a42cf492439

- Disabled the ConnectionStateId struct.

- Disabled the ConnectionMessage struct.

commit bb127f243527c9d8b24e1b622c8246e5040b2707

-- Replaced the accessorise dependency with a local repository and updated the project accordingly.

-- Conditionally implemented the serde Serialize and Deserialize traits for the SendableBytes enum.

Added in this version.

commit 01bee7bb399bca6d6b0fd1a432d7f02a1945da76

- Updated the Corlib dependency to version 0.5.0.




### Deprecated



### Removed

commit bb127f243527c9d8b24e1b622c8246e5040b2707

- Removed the text::SendableTextImmut enum.



### Fixed



### Security



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


