use bevy::prelude::*;

#[derive(Component)]
pub struct Country {
    pub id: u32,
}

#[derive(Component)]
pub struct Nation {
    pub name: String,
}

#[derive(Component)]
pub struct Policy {
    pub democracy: f32,
    pub monarchy: f32,
    pub nationalism: f32,
    pub communism: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GovernmentType {
    Democracy,
    Monarchy,
    Nationalism,
    Communism,
}

#[derive(Component)]
pub struct Government {
    pub kind: GovernmentType,
}
