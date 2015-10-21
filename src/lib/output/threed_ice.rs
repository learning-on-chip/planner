use std::io::Write;

use Result;
use layout::Component;

/// The 3D-ICE format.
pub struct ThreeDICE;

impl super::Output for ThreeDICE {
    fn write(&self, components: &[Component], writer: &mut Write) -> Result<()> {
        let mut first = true;
        for &Component { ref name, position: (x, y), dimension: (width, height) } in components {
            if !first {
                ok!(writer.write(b"\n"));
            } else {
                first = false;
            }
            ok!(writer.write_all(format!(
"{}:
  position {:.0}, {:.0};
  dimension {:.0}, {:.0};

  power values 0;
", name, x * 1e6, y * 1e6, width * 1e6, height * 1e6).as_bytes()));
        }
        Ok(())
    }
}
