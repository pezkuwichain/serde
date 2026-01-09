macro_rules! crate_root {
    () => {
        /// A facade around all the types we need from the `std`, `core`, and `alloc`
        /// crates. This avoids elaborate import wrangling having to happen in every
        /// module.
        ///
        /// NOTE: On targets where std is not available (target_os = "none", e.g. wasm32v1-none),
        /// we always use core/alloc even if the "std" feature is enabled due to Cargo feature
        /// unification. This is handled by checking `target_os = "none"` in all cfg conditions.
        mod lib {
            mod core {
                // On wasm32v1-none (target_os = "none"), always use ::core even if std feature is enabled
                // This handles Cargo feature unification where std gets enabled for no_std targets
                #[cfg(any(not(feature = "std"), target_os = "none"))]
                pub use ::core::*;
                #[cfg(all(feature = "std", not(target_os = "none")))]
                pub use ::std::*;
            }

            // Re-export the full prelude for wasm32v1-none and other no_std targets
            // On target_os = "none", always use ::core::prelude even with std feature
            #[cfg(any(not(feature = "std"), target_os = "none"))]
            pub use ::core::prelude::rust_2021::*;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use ::std::prelude::rust_2021::*;

            // Prelude items that may not be re-exported by glob import
            #[cfg(any(not(feature = "std"), target_os = "none"))]
            pub use ::core::option::Option::{self, None, Some};
            #[cfg(any(not(feature = "std"), target_os = "none"))]
            pub use ::core::result::Result::{self, Err, Ok};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use ::std::option::Option::{self, None, Some};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use ::std::result::Result::{self, Err, Ok};

            pub use self::core::{f32, f64};
            pub use self::core::{iter, num, str};

            #[cfg(any(feature = "std", feature = "alloc"))]
            pub use self::core::{cmp, mem};

            pub use self::core::cell::{Cell, RefCell};
            pub use self::core::cmp::Reverse;
            pub use self::core::fmt::{self, Debug, Display, Write as FmtWrite};
            pub use self::core::marker::{PhantomData, Sized};
            pub use self::core::clone::Clone;
            pub use self::core::marker::{Copy, Send, Sync, Unpin};
            pub use self::core::default::Default;
            pub use self::core::cmp::{Eq, Ord, PartialEq, PartialOrd};
            pub use self::core::convert::{AsMut, AsRef, From, Into};
            pub use self::core::iter::{
                DoubleEndedIterator, ExactSizeIterator, Extend, IntoIterator, Iterator,
            };
            pub use self::core::ops::{Drop, Fn, FnMut, FnOnce};
            pub use self::core::num::Wrapping;
            pub use self::core::ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo};
            pub use self::core::result;
            pub use self::core::time::Duration;

            // For alloc types: use alloc when std is not available OR on target_os = "none"
            #[cfg(all(feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::borrow::{Cow, ToOwned};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::borrow::{Cow, ToOwned};

            #[cfg(all(feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::string::{String, ToString};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::string::{String, ToString};

            #[cfg(all(feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::vec::Vec;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::vec::Vec;

            #[cfg(all(feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::boxed::Box;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::boxed::Box;

            #[cfg(all(feature = "rc", feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::rc::{Rc, Weak as RcWeak};
            #[cfg(all(feature = "rc", feature = "std", not(target_os = "none")))]
            pub use std::rc::{Rc, Weak as RcWeak};

            #[cfg(all(feature = "rc", feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::sync::{Arc, Weak as ArcWeak};
            #[cfg(all(feature = "rc", feature = "std", not(target_os = "none")))]
            pub use std::sync::{Arc, Weak as ArcWeak};

            #[cfg(all(feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

            #[cfg(all(not(no_core_cstr), any(not(feature = "std"), target_os = "none")))]
            pub use self::core::ffi::CStr;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::ffi::CStr;

            #[cfg(all(not(no_core_cstr), feature = "alloc", any(not(feature = "std"), target_os = "none")))]
            pub use alloc::ffi::CString;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::ffi::CString;

            #[cfg(all(not(no_core_net), any(not(feature = "std"), target_os = "none")))]
            pub use self::core::net;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::net;

            // std-only types - not available on target_os = "none"
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::error;

            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::collections::{HashMap, HashSet};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::ffi::{OsStr, OsString};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::hash::{BuildHasher, Hash};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::io::Write;
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::path::{Path, PathBuf};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::sync::{Mutex, RwLock};
            #[cfg(all(feature = "std", not(target_os = "none")))]
            pub use std::time::{SystemTime, UNIX_EPOCH};

            #[cfg(all(feature = "std", not(target_os = "none"), no_target_has_atomic, not(no_std_atomic)))]
            pub use std::sync::atomic::{
                AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
                AtomicU8, AtomicUsize, Ordering,
            };
            #[cfg(all(feature = "std", not(target_os = "none"), no_target_has_atomic, not(no_std_atomic64)))]
            pub use std::sync::atomic::{AtomicI64, AtomicU64};

            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic)))]
            pub use std::sync::atomic::Ordering;
            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic), target_has_atomic = "8"))]
            pub use std::sync::atomic::{AtomicBool, AtomicI8, AtomicU8};
            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic), target_has_atomic = "16"))]
            pub use std::sync::atomic::{AtomicI16, AtomicU16};
            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic), target_has_atomic = "32"))]
            pub use std::sync::atomic::{AtomicI32, AtomicU32};
            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic), target_has_atomic = "64"))]
            pub use std::sync::atomic::{AtomicI64, AtomicU64};
            #[cfg(all(feature = "std", not(target_os = "none"), not(no_target_has_atomic), target_has_atomic = "ptr"))]
            pub use std::sync::atomic::{AtomicIsize, AtomicUsize};

            #[cfg(not(no_core_num_saturating))]
            pub use self::core::num::Saturating;
        }

        // None of this crate's error handling needs the `From::from` error conversion
        // performed implicitly by the `?` operator or the standard library's `try!`
        // macro. This simplified macro gives a 5.5% improvement in compile time
        // compared to standard `try!`, and 9% improvement compared to `?`.
        macro_rules! tri {
            ($expr:expr) => {
                match $expr {
                    Ok(val) => val,
                    Err(err) => return Err(err),
                }
            };
        }

        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/de/mod.rs")]
        pub mod de;
        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/ser/mod.rs")]
        pub mod ser;

        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/format.rs")]
        mod format;

        #[doc(inline)]
        pub use crate::de::{Deserialize, Deserializer};
        #[doc(inline)]
        pub use crate::ser::{Serialize, Serializer};

        // Used by generated code. Not public API.
        #[doc(hidden)]
        #[cfg_attr(
            all(docsrs, if_docsrs_then_no_serde_core),
            path = "core/private/mod.rs"
        )]
        mod private;

        // Used by declarative macro generated code. Not public API.
        #[doc(hidden)]
        pub mod __private {
            #[doc(hidden)]
            pub use crate::private::doc;
            #[doc(hidden)]
            pub use core::result::Result;
        }

        include!(concat!(env!("OUT_DIR"), "/private.rs"));

        #[cfg(all(any(not(feature = "std"), target_os = "none"), no_core_error))]
        #[cfg_attr(all(docsrs, if_docsrs_then_no_serde_core), path = "core/std_error.rs")]
        mod std_error;
    };
}
