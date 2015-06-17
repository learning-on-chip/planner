use std::io::Write;

use Result;
use layout::Component;

pub trait Format {
    fn print(&self, &[Component], &mut Write) -> Result<()>;
}

mod svg;
mod threed_ice;

pub use self::svg::SVG;
pub use self::threed_ice::ThreeDICE;
