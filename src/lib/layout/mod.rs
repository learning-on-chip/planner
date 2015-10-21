use Result;

pub trait Layout {
    fn construct(&self, &Config) -> Result<Vec<Component>>;
}

pub struct Config {
    pub core_count: usize,
    pub core_area: f64,
    pub l3_area: f64,
}

pub struct Component {
    pub name: String,
    pub position: (f64, f64),
    pub dimension: (f64, f64),
}

mod tiles;

pub use self::tiles::Tiles;
