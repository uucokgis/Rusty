#[derive(Debug)]
pub struct Triangle {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
    id: usize
}

impl Triangle {
    pub fn new(x: f64, y: f64, z: f64) -> Triangle {
        Triangle {
            x_coord: x,
            y_coord: y,
            z_coord: z,
            id: 0
        }
    }
}