//! Layout strategies.

use Result;

/// A layout strategy.
pub trait Layout {
    /// Perform the construction.
    fn construct(&self, &Configuration) -> Result<Vec<Element>>;
}

/// A configuration of a layout strategy.
pub struct Configuration {
    /// The number of cores.
    pub core_count: usize,
    /// The area of a core.
    pub core_area: f64,
    /// The area of an L3 cache.
    pub l3_area: f64,
}

/// A layout element.
pub struct Element {
    /// The name.
    pub name: String,
    /// The coordinates of the bottom-left corner.
    pub position: (f64, f64),
    /// The width and height.
    pub dimension: (f64, f64),
}

mod tiles;

pub use self::tiles::Tiles;
