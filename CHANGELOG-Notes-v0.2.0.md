commit 1bac7a0ee5028be16683607af850321348936014 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 29 14:17:18 2026 +1200

    - Updated the package description.
    
    - Updated the readme.
    
    - BasicStatusU32 is now a type alias.
    
    - Added the BasicStatusWithItem struct.

commit 6933c9676959046a230a79265b02c56b77138c6e -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 28 16:02:22 2026 +1200

    - Updated the package version string to “0.2.0-beta".
    
    - Added rust-version to the package fields.
    
    - Removed the cfg-if dependency.
    
    - Updated the delegate dependency to version 0.13.5.
    
    - Updated the pastey dependency to version 0.2.2.
    
    - Added documentation
    
    - ConnectionStateId now conditionally derives the serde Serialize and Deserialize traits.
    
    - Added the BasicStatus and BasicStatusU32 enums.
    
    - Replaced the cfg_if macro call with cfg_select in the sendable_text module.
    
    - Replaced the cfg_if macro call with cfg_select in the sendable_text_immut module.
    
    - Replaced the EssentialStatus default type of the S generic parameter of the WorkInProgressMessage struct and related types with BasicStatus.

commit 7ad030b33a8e795240f2f1acb6b815f099278944 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Mar 31 18:32:43 2026 +1300

    - Made the as_str method public in the text::SendableText implementation.
    
    - Made the as_str method public in the text::SendableTextImmut implementation.

commit b1bb9f994d62038efa205c5d7ce5298159b856f3 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Mar 18 13:47:24 2026 +1300

    - Updated the minimum expected version of the serde dependency to 1.0.228.
    
    - Replaced doc_auto_cfg with doc_cfg in the docsrs cfg_attr in the lib file.
    
    - Added the WorkInProgressMessageOption and WorkInProgressMessageResult types.

commit 6269a93830413deca4c8586fccd4f45635d21248 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 13 17:35:19 2026 +1300

    - Removed the result module.
    
    - Renamed the WorkInProgress struct to WorkInProgressMessage.

commit 0d241dfb105d31df8977ddbdf53dd3a73a25a743
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Mar 12 19:29:31 2026 +1300

    - Added the accessorise, pastey and inc_dec dependencies.
    
    - Added the ConnectionMessage struct.
    
    - Added the ConnectionStateId struct.
    
    - Added the EssentialStatus enum.
    
    - Added the PauseableStatus enum.
    
    - Added the WorkInProgress struct.
    
    - Removed the WorkInProgressResult and IdedWorkInProgressResult structs.
    
    - Removed the WorkInProgressResultIntPercentage and IdedWorkInProgressResultIntPercentage structs.

commit 7264154696dfca38a58afa359ac7ec72292e04d2
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 2 17:16:24 2025 +1200

    - Renamed the result methods of WorkInProgressResult and IdedWorkInProgressResult to result_ref.
    
    - WorkInProgressResult and IdedWorkInProgressResult now implement the Clone and Default traits.
    
    - Disabled the result module.
    
    - Added the SendableTextImmut enum to the text module.
    
    - Moved the WorkInProgressResult and IdedWorkInProgressResult structs to the crate level module.
    
    - Added the WorkInProgressResultIntPercentage and IdedWorkInProgressResultIntPercentage structs.

commit 4f3f9d290368862d7bdf1be2acc53c6721fb0c9e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jul 1 18:40:47 2025 +1200

    - Updated the crate version string to 0.2.0-alpha.
    
    - Added a package.metadata.docs.rs section to the cargo file with the necessary details for rustdoc to label optional features on docs.rs.
    
    - Renamed the done_none method to none_done and the not_done_none method to none_not_done in the WorkInProgressResult and IdedWorkInProgressResult structs.
    
    - Removed the cfg feature decoration from the sendable_text sub-module use statement in the text module mod file.

commit d172d00a4f67512658d31ce846ff56a15dbc4bef
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 1 15:08:50 2025 +1300

    Added the license metadata field.

commit 7cf1b9d5beab7332329b73b3465b8a4442e3f96d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 1 15:06:07 2025 +1300

    Fixed a build error.

commit a7bda3ebc64d6472e22b359600dace13f54f56e8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 1 15:00:45 2025 +1300

    Initial commit
