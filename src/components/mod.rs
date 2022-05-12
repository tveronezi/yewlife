pub mod actions;
pub mod app;
pub mod bean;
pub mod existence;
pub mod icons;
pub mod msg_ctx;

#[derive(PartialEq, Clone)]
pub struct Dimensions {
    pub height: i32,
    pub width: i32,
}
