pub mod body {
    use crate::trajectory::trajectory::TrajectoryPoint;
    use rand::{thread_rng, Rng};
    use std::f32::consts::PI;
    #[derive(Debug)]
    pub struct Body {
        pub dot_position: TrajectoryPoint,
        pub mass: u32,
        pub trajectory: Vec<TrajectoryPoint>,
    }

    impl Body {
        pub fn new(x: f32, y: f32, z: f32, dot_x: f32, dot_y: f32, dot_z: f32, mass: u32) -> Self {
            Self {
                dot_position: TrajectoryPoint {
                    x: dot_x,
                    y: dot_y,
                    z: dot_z,
                },
                mass,
                trajectory: vec![TrajectoryPoint { x, y, z }],
            }
        }

        pub fn gen_random_body() -> Self {
            let mut rng = thread_rng();

            let mass: u32 = rng.gen_range(1..=100);
            let position: TrajectoryPoint = TrajectoryPoint {
                x: rng.gen_range(-10.0..=10.0),
                y: rng.gen_range(-10.0..=10.0),
                z: rng.gen_range(-10.0..=10.0),
            };

            let velocity: f32 = rng.gen_range(0.0..=1.0);
            let (theta, sigma): (f32, f32) =
                (rng.gen_range(0.0..=2.0 * PI), rng.gen_range(0.0..=2.0 * PI));

            let dot_x = velocity * theta.sin() * sigma.cos();
            let dot_y = velocity * theta.sin() * sigma.sin();
            let dot_z = velocity * theta.cos();

            Self::new(
                position.x, position.y, position.z, dot_x, dot_y, dot_z, mass,
            )
        }
    }
}
