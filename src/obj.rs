// this is the obj which is used by everything
#[derive(Clone)] 
pub struct obj{
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub size:f32,
    pub velx: f32,
    pub vely: f32,
    pub bounce: f32,
}