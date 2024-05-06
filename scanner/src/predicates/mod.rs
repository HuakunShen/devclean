use std::path::Path;
pub mod general;
pub mod languages;
pub mod stop;
pub use stop::Stop;

pub trait Removable: Send + Sync {
    fn is_removable(&self, path: &Path) -> bool;
}

pub trait Reportable: Send + Sync {
    fn report(&self, path: &Path) -> bool;
}
