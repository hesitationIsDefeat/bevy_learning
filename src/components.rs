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
        pub value: Vec2,
    }

    impl Velocity {
        pub fn new(x: f32, y: f32) -> Self {
            Self { value: Vec2::new(x, y) }
        }
        pub fn empty() -> Self {
            Self::new(0., 0.)
        }
    }

    #[derive(Component, Debug)]
    pub struct MoveSpeed {
        pub value: f32,
    }

    impl MoveSpeed {
        pub fn new(value: f32) -> Self {
            Self { value }
        }
    }
}

pub mod marker {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Player;
}