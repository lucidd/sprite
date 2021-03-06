
#![crate_name = "sprite"]
#![deny(missing_doc)]
#![warn(dead_code)]
#![feature(globs)]

//! A library for sprite hierarchy and scene management

extern crate uuid;
extern crate event;
extern crate graphics;

pub use action::{
    Action,
    ActionState,

    MoveTo,
    MoveBy,
    RotateTo,
    RotateBy,
    ScaleTo,
    ScaleBy,

    FlipX,
    FlipY,

    Show,
    Hide,
    ToggleVisibility,
    Blink,
    FadeIn,
    FadeOut,
    FadeTo,

    Ease,
};
pub use scene::Scene;
pub use sprite::Sprite;
pub use ease::{
    EaseFunction,

    EaseQuadraticIn,
    EaseQuadraticOut,
    EaseQuadraticInOut,

    EaseCubicIn,
    EaseCubicOut,
    EaseCubicInOut,

    EaseQuarticIn,
    EaseQuarticOut,
    EaseQuarticInOut,

    EaseQuinticIn,
    EaseQuinticOut,
    EaseQuinticInOut,

    EaseSineIn,
    EaseSineOut,
    EaseSineInOut,

    EaseCircularIn,
    EaseCircularOut,
    EaseCircularInOut,

    EaseExponentialIn,
    EaseExponentialOut,
    EaseExponentialInOut,

    EaseElasticIn,
    EaseElasticOut,
    EaseElasticInOut,

    EaseBackIn,
    EaseBackOut,
    EaseBackInOut,

    EaseBounceIn,
    EaseBounceOut,
    EaseBounceInOut,
};

mod action;
mod ease;
mod scene;
mod sprite;

