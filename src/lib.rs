pub mod std {
    pub mod zig {
        pub use crate::zig;
    }
}

#[path = "../lib/std/zig/mod.rs"]
pub mod zig;
