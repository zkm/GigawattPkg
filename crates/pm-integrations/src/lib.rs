mod detect;
mod process;
mod router;

mod arch;
mod fedora;

pub use detect::{detect_distro, executable_exists};
pub use router::{BackendOptions, resolve_backend};
