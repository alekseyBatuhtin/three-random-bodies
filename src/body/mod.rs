pub mod body {
    use crate::trajectory::trajectory::TrajectoryPoint;

    #[derive(Debug)]
    pub struct Body {
        pub dot_position: TrajectoryPoint,
        pub mass: u32,
        pub trajectory: Vec<TrajectoryPoint>,
    }

    impl Body {
        pub fn new(x: f32, y: f32, z: f32, mass: u32) -> Self {
            Body {
                dot_position: TrajectoryPoint { x, y, z },
                mass,
                trajectory: vec![TrajectoryPoint { x, y, z }],
            }
        }
    }
}
