use std::io::Write;

use Result;
use layout::Element;

macro_rules! element_template(
    () => (
r#"{name}:
  position {x:.0}, {y:.0};
  dimension {width:.0}, {height:.0};

  power values 0;
"#
    );
);

/// The 3D-ICE format.
pub struct ThreeDICE;

impl super::Format for ThreeDICE {
    fn write(&self, elements: &[Element], writer: &mut Write) -> Result<()> {
        let mut first = true;
        for &Element { ref name, position: (x, y), dimension: (width, height) } in elements {
            if !first {
                ok!(writer.write(b"\n"));
            } else {
                first = false;
            }
            ok!(writer.write_fmt(format_args!(element_template!(),
                                              name = name,
                                              x = x * 1e6,
                                              y = y * 1e6,
                                              width = width * 1e6,
                                              height = height * 1e6)));
        }
        Ok(())
    }
}
