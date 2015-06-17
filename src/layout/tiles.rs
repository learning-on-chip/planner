use Result;
use super::{Component, Layout, Spec};

const CORE_LABEL: &'static str = "Core";
const L3_LABEL: &'static str = "L3";

const CORE_WIDTH_HEIGHT_RATIO: f64 = 0.5;
const CORES_PER_L3: usize = 4;

pub struct Tiles;

impl Layout for Tiles {
    fn construct(&self, &Spec { core_count, core_area, l3_area }: &Spec)
                 -> Result<Vec<Component>> {

        if core_count % CORES_PER_L3 != 0 {
            raise!("the number of cores should be a multiple of {}", CORES_PER_L3);
        }

        let core_width = (core_area * CORE_WIDTH_HEIGHT_RATIO).sqrt();
        let core_height = core_area / core_width;

        let l3_width = (CORES_PER_L3 as f64) * core_width;
        let l3_height = l3_area / l3_width;

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
