//! Formatting strategies.

use std::io::Write;

use Result;
use layout::Component;

/// A formating strategy.
pub trait Format {
    /// Perform the formatting.
    fn write(&self, &[Component], &mut Write) -> Result<()>;
}

mod svg;
mod threed_ice;

pub use self::svg::SVG;
pub use self::threed_ice::ThreeDICE;
