// Copyright 2026 Henrik Dick. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Clear a surface of the vlogger, including the messages that have been sent to it.
///
/// # Examples
///
/// ```
/// use v_log::clear;
///
/// clear!("main_surface");
/// ```
#[macro_export]
macro_rules! clear {
    // clear!(vlogger: my_vlogger, target: "my_target", "my_surface")
    (vlogger: $vlogger:expr, $surface:expr) => {
        $crate::__private_api::clear(
            $crate::__vlog_vlogger!($vlogger),
            $crate::__private_api::module_path!(),
            $surface,
        )
    };

    // clear!(vlogger: my_vlogger, "my_surface")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr) => {
        $crate::__private_api::clear($crate::__vlog_vlogger!($vlogger), $target, $surface)
    };

    // clear!(target: "my_target", "my_surface")
    (target: $target:expr, $surface:expr) => {
        $crate::__private_api::clear(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $target,
            $surface,
        )
    };

    // clear!("my_surface")
    ($surface:expr) => {
        $crate::__private_api::clear(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $crate::__private_api::module_path!(),
            $surface,
        )
    };
}

/// Logs a message to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::message;
///
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// message!("main_surface", color: Healthy, "Correct position");
/// message!("main_surface", "Position is: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! message {
    // message!(vlogger: my_vlogger, target: "my_target", "my_surface", color: Base, "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // message!(vlogger: my_vlogger, "my_surface", color: Base, "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // message!(target: "my_target", "my_surface", color: Base, "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // message!("my_surface", color: Base, "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__message!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    )
}

/// Sends a point to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::point;
///
/// let pos1 = [3.234, -1.223];
/// let pos2 = [2.713, 0.577];
///
/// point!("main_surface", pos1, 5.0, Base, "o", "Position is: x: {}, y: {}", pos1[0], pos1[1]);
/// point!("main_surface", pos2, 5.0, Base);
/// ```
#[macro_export]
macro_rules! point {
    // point!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], 5.0, "o", "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // point!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], 5.0, "o", "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // point!(target: "my_target", "my_surface", [1.0, 2.0], 5.0, "o", "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // point!("my_surface", [1.0, 2.0], 5.0, "o", "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__point!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    )
}

/// Sends a label/text annotation to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::label;
///
/// let pos = [3.234, -1.223];
///
/// label!("main_surface", pos, (12.0, Base, "<"), "Position is: x: {}, y: {}", pos[0], pos[1]);
/// label!("main_surface", pos, "Flexible position"); // with size 12.0, flexible alignment and "Base" color
/// ```
#[macro_export]
macro_rules! label {
    // label!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], 12.0, Base, "<", "a {} label", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // label!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], 12.0, Base, "<", "a {} label", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // label!(target: "my_target", "my_surface", [1.0, 2.0], 12.0, Base, "<", "a {} label", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // label!("my_surface", [1.0, 2.0], 12.0, Base, "<", "a {} label", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__label!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    )
}

/// Sends an open or closed polyline to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::line;
///
/// let pos1 = [3.234, -1.223];
/// let pos2 = [2.713, 0.577];
/// let pos3 = [6.283, 0.692];
///
/// // text is only allowed on single lines
/// polyline!("main_surface", (pos1, pos2), 5.0, Base, "--", "Position is: x: {}, y: {}", pos1[0], pos1[1]);
/// polyline!("main_surface", (pos1, pos2), 5.0, Base, "--");
/// polyline!("main_surface", (pos1, pos2), 5.0, Base);
/// polyline!("main_surface", (pos1, pos2, pos3), 5.0, Base, "--");
/// polyline!("main_surface", (pos1, pos2, pos3), 5.0, Base);
/// // adding a last , makes it closed -> draws a triangle
/// polyline!("main_surface", (pos1, pos2, pos3,), 5.0, Base, "--");
/// polyline!("main_surface", (pos1, pos2, pos3,), 5.0, Base);
/// ```
#[macro_export]
macro_rules! polyline {
    // polyline!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], 5.0, "o", Base, "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // polyline!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], 5.0, "o", Base, "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // polyline!(target: "my_target", "my_surface", [1.0, 2.0], 5.0, "o", Base, "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($target, $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    });

    // polyline!("my_surface", [1.0, 2.0], 5.0, "o", Base, "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__line!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &($crate::__private_api::module_path!(), $crate::__private_api::module_path!(), $crate::__private_api::loc()),
            $($arg)+
        )
    )
}

#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __message {
    ($vlogger:expr, $surface:expr, $loc:expr, color: $color:tt, $($arg:tt)+) => {
        $crate::__private_api::vlog_message(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $crate::__color!($color),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $($arg:tt)+) => {
        $crate::__private_api::vlog_message(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $crate::__color!(Base),
            $surface,
            $loc
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __point {
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $size:expr, $color:tt, $style:tt, $($arg:tt)+) => {
        $crate::__private_api::vlog_point(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $pos,
            $size,
            $crate::__color!($color),
            $crate::__point_style!($style),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $size:expr, $color:tt, $style:tt) => {
        $crate::__private_api::vlog_point(
            $vlogger,
            $crate::__private_api::format_args!(""),
            $pos,
            $size,
            $crate::__color!($color),
            $crate::__point_style!($style),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $size:expr, $color:tt) => {
        $crate::__private_api::vlog_point(
            $vlogger,
            $crate::__private_api::format_args!(""),
            $pos,
            $size,
            $crate::__color!($color),
            $crate::__point_style!("o"),
            $surface,
            $loc
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __label {
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, ($size:expr, $color:tt, $align:tt), $($arg:tt)+) => {
        $crate::__private_api::vlog_label(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $pos,
            $size,
            $crate::__color!($color),
            $crate::__alignment!($align),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $($arg:tt)+) => {
        $crate::__private_api::vlog_label(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $pos,
            12.0, // default size of 12 pixels
            $crate::__color!(Base),
            $crate::__alignment!("x"),
            $surface,
            $loc
        )
    };
}

#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __line {
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $pos2:expr), $size:expr, $color:tt, $style:tt, $($arg:tt)+) => {
        $crate::__private_api::vlog_line(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $pos1,
            $pos2,
            $size,
            $crate::__color!($color),
            $crate::__line_style!($style),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $pos2:expr), $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, ($pos1, $pos2), $size, $color, "-")
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $($pos2:expr),+), $size:expr, $color:tt, $style:tt) => {
        let mut last = $pos1;
        $(
        let next = $pos2;
        $crate::__private_api::vlog_line(
            $vlogger,
            $crate::__private_api::format_args!(""),
            last,
            next.clone(),
            $size,
            $crate::__color!($color),
            $crate::__line_style!($style),
            $surface,
            $loc
        );
        last = next;
        )+
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $($pos2:expr),+), $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, ($pos1, $($pos2),+), $size, $color, "-")
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $($pos2:expr,)+), $size:expr, $color:tt, $style:tt) => {
        let mut last = $pos1;
        let first = last.clone();
        $(
        let next = $pos2;
        $crate::__private_api::vlog_line(
            $vlogger,
            $crate::__private_api::format_args!(""),
            last,
            next.clone(),
            $size,
            $crate::__color!($color),
            $crate::__line_style!($style),
            $surface,
            $loc
        );
        last = next;
        )+
        $crate::__private_api::vlog_line(
            $vlogger,
            $crate::__private_api::format_args!(""),
            last,
            first,
            $size,
            $crate::__color!($color),
            $crate::__line_style!($style),
            $surface,
            $loc
        );
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $($pos2:expr,)+), $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, ($pos1, $($pos2,)+), $size, $color, "-")
    };
}

/// Determines if a message vlogged at the specified level in that module will
/// be vlogged.
///
/// This can be used to avoid expensive computation of vlog message arguments if
/// the message would be ignored anyway.
///
/// # Examples
///
/// ```
/// use v_log::{message, vlog_enabled};
///
/// # struct Data { x: u32, y: u32 }
/// # fn expensive_call() -> Data { Data { x: 0, y: 0 } }
/// # let my_vlogger = v_log::__private_api::GlobalVLogger;
/// if vlog_enabled!("main_surface") {
///     let data = expensive_call();
///     message!("main_surface", color: Info, "expensive debug data: {} {}", data.x, data.y);
/// }
///
/// if vlog_enabled!(target: "Global", "main_surface") {
///    let data = expensive_call();
///    message!(target: "Global", "main_surface", "expensive debug data: {} {}", data.x, data.y);
/// }
///
/// if vlog_enabled!(vlogger: my_vlogger, "main_surface") {
///    let data = expensive_call();
///    message!(target: "Global", "main_surface", "expensive debug data: {} {}", data.x, data.y);
/// }
/// ```
#[macro_export]
macro_rules! vlog_enabled {
    // vlog_enabled!(vlogger: my_vlogger, target: "my_target", "my_surface")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr) => {{
        $crate::__private_api::enabled($crate::__vlog_vlogger!($vlogger), $surface, $target)
    }};

    // vlog_enabled!(vlogger: my_vlogger, "my_surface")
    (vlogger: $vlogger:expr, $surface:expr) => {{
        $crate::__private_api::enabled(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            $crate::__private_api::module_path!(),
        )
    }};

    // vlog_enabled!(target: "my_target", "my_surface")
    (target: $target:expr, $surface:expr) => {{
        $crate::__private_api::enabled(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            $target,
        )
    }};

    // vlog_enabled!("my_surface")
    ($surface:expr) => {{
        $crate::__private_api::enabled(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            $crate::__private_api::module_path!(),
        )
    }};
}

// Determine the vlogger to use, and whether to take it by-value or by reference

#[doc(hidden)]
#[macro_export]
macro_rules! __vlog_vlogger {
    (__vlog_global_vlogger) => {{
        $crate::__private_api::GlobalVLogger
    }};

    ($vlogger:expr) => {{
        &($vlogger)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __point_style {
    ("O") => {
        $crate::PointStyle::FilledCircle
    };
    ("-O") => {
        $crate::PointStyle::Circle
    };
    ("--O") => {
        $crate::PointStyle::DashedCircle
    };
    ("o") => {
        $crate::PointStyle::Point
    };
    ("-o") => {
        $crate::PointStyle::PointOutline
    };
    ("s") => {
        $crate::PointStyle::PointSquare
    };
    ("-s") => {
        $crate::PointStyle::PointSquareOutline
    };
    ("x") => {
        $crate::PointStyle::PointCross
    };
    ("d") => {
        $crate::PointStyle::PointDiamond
    };
    ("-d") => {
        $crate::PointStyle::PointDiamondOutline
    };
    ($s:literal) => {
        compile_error!(concat!("unknown point style ", $s))
    };
    ($s:ident) => {
        $crate::PointStyle::$s
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __line_style {
    ("-") => {
        $crate::LineStyle::Simple
    };
    ("--") => {
        $crate::LineStyle::Dashed
    };
    ("->") => {
        $crate::LineStyle::Arrow
    };
    ("_>") => {
        $crate::LineStyle::InsideHarpoonCCW
    };
    ("<_") => {
        $crate::LineStyle::InsideHarpoonCW
    };
    ($s:literal) => {
        panic!(concat!("unknown line style ", $s))
    };
    ($s:ident) => {
        $crate::LineStyle::$s
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __alignment {
    ("<") => {
        $crate::TextAlignment::Left
    };
    (">") => {
        $crate::TextAlignment::Right
    };
    (".") => {
        $crate::TextAlignment::Center
    };
    ("x") => {
        $crate::TextAlignment::Flexible
    };
    ($a:literal) => {
        compile_error!(concat!("unknown text alignment ", $a))
    };
    ($a:ident) => {
        $crate::TextAlignment::$a
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __color {
    ($hex:literal) => {
        $crate::Color::Hex($hex)
    };
    ($name:ident) => {
        $crate::Color::$name
    };
}
