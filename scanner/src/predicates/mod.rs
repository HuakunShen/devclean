use std::path::Path;

pub mod general;
pub mod stop;
use stop::Stop;
pub mod languages;

pub trait Removable {
    fn is_removable(&self, path: &Path) -> bool;
}
