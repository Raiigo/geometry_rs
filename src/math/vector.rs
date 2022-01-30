pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new<T>(x: T, y: T, z: T) -> Self
    where
        f64: From<T>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}