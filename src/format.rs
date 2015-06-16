use std::io::Write;

use Result;
use layout::Component;

pub trait Format {
    fn print<W: Write>(&self, component: &[Component], writer: W) -> Result<()>;
}

pub struct ThreeDICE;

impl ThreeDICE {
    #[inline]
    pub fn new() -> ThreeDICE {
        ThreeDICE
    }
}

impl Format for ThreeDICE {
    fn print<W: Write>(&self, components: &[Component], mut writer: W) -> Result<()> {
        let mut first = true;
        for &Component { ref name, position: (x, y), dimension: (width, height) } in components {
            if !first {
                ok!(writer.write(b"\n"));
            } else {
                first = false;
            }
            ok!(writer.write_all(format!(
"{}:
  position {}, {};
  dimension {}, {};

  power: values 0.0;
", name, x, y, width, height).as_bytes()));
        }
        Ok(())
    }
}
