commit 89ec0f214e8af1a1a8185ae69b47956da8db33d0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Sep 9 20:39:25 2026 +1200

    - Updated the package version to 0.3.0-beta.
    
    - Updated the package description.
    
    - Updated the serde dependency to version 1.0.229.
    
    - Updated the pastey dependency to version 0.2.3.
    
    - Updated the inc_dec dependency to version 0.2.0.
    
    - Prepared the changelog.
    
    - Updated the readme.
    
    - Continued work on CowableRef.
    
    - Continued work on SendableBytes.
    
    - Continued work on SendableRef.
    
    - Added the clone_if_not_string method to the text::SendableText enum.
    
    - Updated documentation
    
    - Added documentation
    
    - Changed the name of the variable_state_number field and getter method to “value” in the VariableStateNumber struct definition and implementation.
    
    - Other minor changes.

commit c7ecb31521426336688f3fa839687f39e5065340 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Sep 8 19:13:35 2026 +1200

    - Updated the readme.
    
    - Continued work on VecBufferInputOutput and EmptyVecBufferInputOutput.
    
    - Continued work on CowableRef.
    
    - Continued work on SendableBytes.
    
    - Continued work on SendableRef.
    
    - The text::SendableText enum now implements the PartialOrd and Ord traits.
    
    - The type of the st_queue field of the text::SendableTextLog struct has been changed to std::collections::VecDeque.
    
    - The VariableStateMessage struct now implements Clone.

commit 9381a523fb400a127116ae0d1f3450c2564a39f9 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Sep 7 20:14:52 2026 +1200

    - Updated the accessorise dependency to version 0.3.0.
    
    - Continued work on the CowableRef, SendableBytes and SendableRef implementations.
    
    - Updated some documentation.
    
    - Other minor changes.

commit 0d3546f39a12332d5c05d619be70ee5de0a2e452 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Aug 13 19:41:53 2026 +1200

    - Conditionally implemented the Serialize and Deserialize traits for the SendableRef enum.
    
    - Conditionally added the SendableRefVisitor struct.

commit a03fa40cd433603f518d22df18f14eff85f6bee4 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Aug 10 18:34:52 2026 +1200

    - Renamed the VecBufferRetriever trait to VecBufferInputOutput.
    
    - Added implementer constrains, removed the buffer method and added put_buffer, put_opt_buffer, take_buffer and take_buffer_with_capacity methods to the VecBufferInputOutput trait.
    
    - Added the EmptyVecBufferInputOutput struct.
    
    - Continued work on the CowableRef and SendableRef enums.

commit 40b19e23662db6a80b07c4c104d56e4f37838b9d -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Aug 7 18:46:24 2026 +1200

    - Added the VecBufferRetriever trait.
    
    - The EssentialStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.
    
    - The PauseableStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.
    
    - The BasicStatus enum now derives the PartialEq, Eq, PartialOrd and Ord traits.
    
    - The progress_item_ref and progress_item_mut methods have been added to the BasicStatus implementation.
    
    - Conditionally implemented the Default, PartialEq, Eq, and PartialOrd traits for the BasicStatusWithItem enum.
    
    - Renamed the “N” generic parameter to “M” in the same_number method definition in the VariableStateMessage implementation.
    
    - Other minor changes.

commit 29925f714300f8e3eb3a82f7bf0e99b4bbf92f3c -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Aug 6 14:36:06 2026 +1200

    Continued work on the CowableRef and the SendableRef enums.

commit fda310016d1e5e9bf052816d11fa6420c951d32d -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Aug 5 21:59:35 2026 +1200

    - Added the CowableRef enum.
    
    - Added the SendableRef enum.

commit 600055f66e3761af84462016b80ab11aaddc0d75 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Aug 4 19:32:25 2026 +1200

    - Continued work on the SendableBytes struct.
    
    - Added the len, capacity and len_is_at_capacity methods to the text::SendableText implementation.

commit 8c91ac4b982047ce2fda803675e5fa95606a95c1 --
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Aug 1 20:44:33 2026 +1200

    Continued work on the SendableBytes enum.

commit 404f0f90797cc818f71e731eb61d0a42cf492439 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 29 12:57:19 2026 +1200

    - Disabled the ConnectionStateId struct.
    
    - Disabled the ConnectionMessage struct.
    
    - Added the VariableStateMessage struct.
    
    - Added the VariableStateNumber struct.

commit bb127f243527c9d8b24e1b622c8246e5040b2707 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 24 19:07:44 2026 +1200

    - Replaced the accessorise dependency with a local repository and updated the project accordingly.
    
    - Added string, str and arc_str methods to the SendableBytes enum.
    
    - Conditionally implemented the serde Serialize and Deserialize traits for the SendableBytes enum.
    
    - Conditionally added the SendableBytesVisitor struct.
    
    - Removed the text::SendableTextImmut enum.

commit 01bee7bb399bca6d6b0fd1a432d7f02a1945da76 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jul 23 19:31:02 2026 +1200

    - Updated the Corlib dependency to version 0.5.0.
    
    - Added the bytes crate as an optional dependency.
    
    - Added the SendableBytes enum.
