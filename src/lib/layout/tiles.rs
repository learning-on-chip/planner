use Result;
use super::{Component, Configuration, Layout};

const CORE_LABEL: &'static str = "Core";
const L3_LABEL: &'static str = "L3";

const CORES_PER_L3: usize = 4;
const CORE_WIDTH_HEIGHT_RATIO: f64 = 0.5;
const SIZE_RESOLUTION: f64 = 1e6;

/// A layout strategy that builds a road of L3 caches with sidewalks of cores.
pub struct Tiles;

impl Layout for Tiles {
    fn construct(&self, &Configuration { core_count, core_area, l3_area }: &Configuration)
                 -> Result<Vec<Component>> {

        macro_rules! round(
            ($size:expr) => ({
                let mut size = (SIZE_RESOLUTION * $size).round() as usize;
                size += size % 2;
                (size as f64) / SIZE_RESOLUTION
            });
        );

        if core_count % CORES_PER_L3 != 0 {
            raise!("the number of cores should be a multiple of {}", CORES_PER_L3);
        }

        let core_width = round!((core_area * CORE_WIDTH_HEIGHT_RATIO).sqrt());
        let core_height = round!(core_area / core_width);

        let l3_width = round!((CORES_PER_L3 as f64) * core_width);
        let l3_height = round!(l3_area / l3_width);

        let mut components = Vec::new();
        for k in 0..core_count {
            let l = k / (2 * CORES_PER_L3);
            let i = (k % (2 * CORES_PER_L3)) % CORES_PER_L3 + l * CORES_PER_L3;
            let j = (k % (2 * CORES_PER_L3)) / CORES_PER_L3;
            if i % CORES_PER_L3 == 0 && j == 1 {
                components.push(Component {
                    name: format!("{}{}", L3_LABEL, 2 * l + 1),
                    position: ((l as f64) * l3_width, core_height + l3_height),
                    dimension: (l3_width, l3_height),
                });
            }
            components.push(Component {
                name: format!("{}{}", CORE_LABEL, k),
                position: ((i as f64) * core_width, (j as f64) * (core_height + 2.0 * l3_height)),
                dimension: (core_width, core_height),
            });
            if i % CORES_PER_L3 == CORES_PER_L3 - 1 && j == 0 {
                components.push(Component {
                    name: format!("{}{}", L3_LABEL, 2 * l),
                    position: ((l as f64) * l3_width, core_height),
                    dimension: (l3_width, l3_height),
                });
            }
        }
        Ok(components)
    }
}
