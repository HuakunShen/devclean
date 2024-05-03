use std::path::Path;

pub mod general;
pub mod stop;
pub use stop::Stop;
pub mod languages;

pub trait Removable {
    fn is_removable(&self, path: &Path) -> bool;
}

pub trait Reportable {
    fn report(&self, path: &Path) -> bool;
}
