pub mod trajectory {
    use crate::body::body::Body;
    #[derive(Debug)]
    pub struct TrajectoryPoint {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    pub fn calc_center_of_trajectory_mass(bodies: &[Body; 3]) -> (f32, f32, f32) {
        let sum_mass = bodies
            .iter()
            .fold(0.0, |acc, body| acc + (body.mass as f32));
        let center_tr_x = bodies.iter().fold(0.0, |acc, body| {
            acc + body
                .trajectory
                .iter()
                .fold(0.0, |acc, tr| acc + (body.mass as f32) * tr.x)
        }) / sum_mass;

        let center_tr_y = bodies.iter().fold(0.0, |acc, body| {
            acc + body
                .trajectory
                .iter()
                .fold(0.0, |acc, tr| acc + (body.mass as f32) * tr.y)
        }) / sum_mass;

        let center_tr_z = bodies.iter().fold(0.0, |acc, body| {
            acc + body
                .trajectory
                .iter()
                .fold(0.0, |acc, tr| acc + (body.mass as f32) * tr.z)
        }) / sum_mass;

        (center_tr_x, center_tr_y, center_tr_z)
    }
}

#[cfg(test)]
mod trajectory_test {
    use super::*;
    use crate::body::body::Body;

    #[test]
    fn center_of_mass() {
        let bodies = [
            Body::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1),
            Body::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1),
            Body::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1),
        ];

        assert_eq!(
            trajectory::calc_center_of_trajectory_mass(&bodies),
            (1.0, 1.0, 1.0)
        );
    }
}
