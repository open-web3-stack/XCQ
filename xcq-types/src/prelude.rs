use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::{
            any,
            borrow,
            boxed,
            cmp,
            collections,
            fmt,
            format,
            hash,
            marker,
            mem,
            num,
            ops,
            string,
            sync,
            time,
            vec,
            rc,
            iter,
        };
    } else {
        pub use alloc::{
            borrow,
            boxed,
            collections,
            format,
            string,
            sync,
            vec,
            rc
        };

        pub use core::{
            any,
            cmp,
            fmt,
            hash,
            marker,
            mem,
            num,
            ops,
            time,
            iter,
        };
    }
}
