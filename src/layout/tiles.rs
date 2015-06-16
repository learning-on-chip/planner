use Result;
use super::{Component, Layout, Spec};

const CORE_LABEL: &'static str = "Core";
const L3_LABEL: &'static str = "L3";

const CORE_WIDTH_HEIGHT_RATIO: f64 = 0.5;
const CORES_PER_ROW: usize = 4;

pub struct Tiles;

impl Layout for Tiles {
    fn construct(&self, &Spec { core_count, core_area, l3_area }: &Spec)
                 -> Result<Vec<Component>> {

        let core_width = (core_area * CORE_WIDTH_HEIGHT_RATIO).sqrt();
        let core_height = core_area / core_width;

        let l3_width = (CORES_PER_ROW as f64) * core_width;
        let l3_height = l3_area / l3_width;

        let mut offset = 0.0;
        let mut components = Vec::new();
        for k in 0..core_count {
            let i = k % CORES_PER_ROW;
            components.push(Component {
                name: format!("{}{}", CORE_LABEL, k),
                position: ((i as f64) * core_width, offset),
                dimension: (core_width, core_height),
            });
            if i != CORES_PER_ROW - 1 {
                continue;
            }
            let i = k / CORES_PER_ROW;
            offset += core_height;
            components.push(Component {
                name: format!("{}{}", L3_LABEL, i),
                position: (0.0, offset),
                dimension: (l3_width, l3_height),
            });
            offset += l3_height;
        }
        Ok(components)
    }
}
