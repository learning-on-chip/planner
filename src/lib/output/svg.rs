use std::io::Write;

use Result;
use layout::Component;

/// The SVG format.
pub struct SVG;

impl super::Output for SVG {
    fn write(&self, components: &[Component], writer: &mut Write) -> Result<()> {
        use std::f64::{INFINITY, NEG_INFINITY};

        let (mut min_x, mut min_y) = (INFINITY, INFINITY);
        let (mut max_x, mut max_y) = (NEG_INFINITY, NEG_INFINITY);
        for &Component { position: (x, y), dimension: (width, height), .. } in components {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }

        const MARGIN: f64 = 5.0;
        const WIDTH: f64 = 800.0;

        let scale = WIDTH / (max_x - min_x);
        let height = (max_y - min_y) * scale;

        let mut body = String::new();
        for &Component { ref name, position: (x, y), dimension: (width, height) } in components {
            body.push_str(&format!(
                "    <rect x=\"{:.0}\" \
                           y=\"{:.0}\" \
                           width=\"{:.0}\" \
                           height=\"{:.0}\" \
                           id=\"{}\" />\n",
                MARGIN + scale * (x - min_x), MARGIN + scale * (y - min_y),
                scale * width, scale * height, name,
            ));
        }

        macro_rules! template(
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

        ok!(writer.write_all(format!(template!(), width = WIDTH + 2.0 * MARGIN,
                                     height = height + 2.0 * MARGIN,
                                     body = body.trim()).as_bytes()));

        Ok(())
    }
}
