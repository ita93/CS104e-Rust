use atags::raw;

pub use atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(value) => Some(value),
            _ => None
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(value) => Some(value),
            _ => None
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self{
            Atag::Cmd(value) => Some(value),
            _ => None
        }
    }
}

// FIXME: Implement `From<raw::Core>`, `From<raw::Mem>`, and `From<&raw::Cmd>`
// for `Atag`. These implementations should be used by the `From<&raw::Atag> for
// Atag` implementation below.

impl<'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        // FIXME: Complete the implementation below.

        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => unimplemented!(),
                (raw::Atag::MEM, &raw::Kind { mem }) => unimplemented!(),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => unimplemented!(),
                (raw::Atag::NONE, _) => unimplemented!(),
                (id, _) => unimplemented!()
            }
        }
    }
}
