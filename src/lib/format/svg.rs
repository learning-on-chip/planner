use std::io::Write;

use Result;
use layout::Element;

const MARGIN: f64 = 5.0;
const WIDTH: f64 = 800.0;

macro_rules! element_template(
    () => (
r#"    <rect id="{id}" x="{x:.0}" y="{y:.0}" width="{width:.0}" height="{height:.0}" />
"#
    );
);

macro_rules! document_template(
    () => (
r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="{width:.0}" height="{height:.0}">
  <style type="text/css">
    rect {{
        fill: none;
        stroke: black;
        stroke-width: 2;
    }}
    rect[id^="Core"] {{
        fill: #0071bc;
    }}
    rect[id^="L3"] {{
        fill: #ecb01f;
    }}
  </style>
  <g transform="translate(0, {height:.0}) scale(1, -1)">
    {body}
  </g>
</svg>
"#
    );
);

/// The SVG format.
pub struct SVG;

impl super::Format for SVG {
    fn write(&self, elements: &[Element], writer: &mut Write) -> Result<()> {
        use std::f64::{INFINITY, NEG_INFINITY};

        let (mut min_x, mut min_y) = (INFINITY, INFINITY);
        let (mut max_x, mut max_y) = (NEG_INFINITY, NEG_INFINITY);
        for &Element { position: (x, y), dimension: (width, height), .. } in elements {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }

        let scale = WIDTH / (max_x - min_x);
        let height = (max_y - min_y) * scale;

        let mut body = String::new();
        for &Element { ref name, position: (x, y), dimension: (width, height) } in elements {
            body.push_str(&format!(element_template!(),
                                   id = name,
                                   x = MARGIN + scale * (x - min_x),
                                   y = MARGIN + scale * (y - min_y),
                                   width = scale * width,
                                   height = scale * height));
        }

        ok!(writer.write_fmt(format_args!(document_template!(),
                                          width = WIDTH + 2.0 * MARGIN,
                                          height = height + 2.0 * MARGIN,
                                          body = body.trim())));

        Ok(())
    }
}
