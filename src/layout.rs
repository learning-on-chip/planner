use Result;

const CORE_WIDTH_HEIGHT_RATIO: f64 = 2.0;
const CORE_LABEL: &'static str = "Core";
const L3_LABEL: &'static str = "L3";

pub trait Layout {
    fn construct(&self, spec: &Spec) -> Result<Vec<Component>>;
}

pub struct Spec {
    pub core_count: usize,
    pub core_area: f64,
    pub l3_area: f64,
}

pub struct Component {
    pub name: String,
    pub position: (f64, f64),
    pub dimension: (f64, f64),
}

pub struct Tiles;

impl Tiles {
    #[inline]
    pub fn new() -> Tiles {
        Tiles
    }
}

impl Layout for Tiles {
    fn construct(&self, &Spec { core_count, .. }: &Spec) -> Result<Vec<Component>> {
        let mut components = Vec::new();
        for i in 0..core_count {
            components.push(Component {
                name: format!("{}{}", CORE_LABEL, i),
                position: (0.0, 0.0),
                dimension: (0.0, 0.0),
            });
        }
        Ok(components)
    }
}
