pub mod actions;
pub mod app;
pub mod bean;
pub mod existence;
pub mod icons;
pub mod universe_ctx;
use yew::prelude::*;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Dimensions {
    pub height: i32,
    pub width: i32,
}
