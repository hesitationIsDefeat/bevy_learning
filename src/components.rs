pub mod normal {
    use bevy::prelude::{Component, Vec2};

    #[derive(Component, Debug)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
    }

    impl Position {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    #[derive(Component, Debug)]
    pub struct Velocity {
        pub direction: Vec2,
        pub speed: f32,
    }

    impl Velocity {
        pub fn new(x: f32, y: f32, speed: f32) -> Self {
            Self { direction: Vec2::new(x, y).normalize(), speed }
        }
        pub fn from_v2(v2: Vec2, speed: f32) -> Self {
            Self::new(v2.x, v2.y, speed)
        }
        pub fn no_direction(speed: f32) -> Self {
            Self::new(0., 0., speed)
        }
        pub fn empty() -> Self {
            Self::no_direction(0.)
        }
    }

    #[derive(Component, Debug)]
    pub struct Target {
        pub value: Vec2,
    }

    impl Target {
        pub fn new(x: f32, y: f32) -> Self {
            Self { value: Vec2::new(x, y) }
        }
        pub fn from_v2(v2: Vec2) -> Self {
            Self::new(v2.x, v2.y)
        }
        pub fn empty() -> Self {
            Self::new(0., 0.)
        }
    }
}

pub mod marker {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Player;
}