// Copyright 2026 redweasel. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Import this as `use v_log::macros::*` to import only the macros.

pub use crate::{arrow, clear, label, message, point, polyline, vlog_enabled};

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
/// let pos = [3.234, -1.223];
///
/// message!("main_surface", color: Healthy, "Correct position");
/// message!("main_surface", "Position is: x: {}, y: {}", pos[0], pos[1]);
/// ```
#[macro_export]
macro_rules! message {
    // message!(vlogger: my_vlogger, target: "my_target", "my_surface", color: Base, "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // message!(vlogger: my_vlogger, "my_surface", color: Base, "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    });

    // message!(target: "my_target", "my_surface", color: Base, "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__message!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // message!("my_surface", color: Base, "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__message!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!(),
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
    // point!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], 5.0, Base, "o", "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // point!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], 5.0, Base, "o", "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    });

    // point!(target: "my_target", "my_surface", [1.0, 2.0], 5.0, Base, "o", "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__point!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // point!("my_surface", [1.0, 2.0], 5.0, Base, "o", "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__point!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!(),
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
    // label!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], (12.0, Base, "<"), "a {} label", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // label!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], (12.0, Base, "<"), "a {} label", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    });

    // label!(target: "my_target", "my_surface", [1.0, 2.0], (12.0, Base, "<"), "a {} label", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__label!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // label!("my_surface", [1.0, 2.0], (12.0, Base, "<"), "a {} label", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__label!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    )
}

/// Sends an open or closed polyline to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::polyline;
///
/// // Points must be of the same type for arrays to work,
/// // but are only required to implement IntoIterator.
/// // They can be arbitrary dimension, but only the first 3 are used.
/// let pos1 = [3.234, -1.223];
/// let pos2 = [2.713, 0.577];
/// let pos3 = [6.283, 0.692];
///
/// // Draw a single line with thickness 5.0 and color `Base` from pos1 to pos2.
/// // The label may get displayed, but the size, position and and format is up
/// // to the vlogger implementation. E.g. it may only be displayed as a tooltip.
/// polyline!("main_surface", (pos1, pos2), 5.0, Base, "--", "Position is: x: {}, y: {}", pos1[0], pos1[1]);
/// polyline!("main_surface", (pos1, pos2), 5.0, Base, "->");
/// polyline!("main_surface", (pos1, pos2), 5.0, Base);
/// // Draw two connected lines (polyline). These can not be labelled in
/// // the macro, as a label on polylines is hard to control in implementations.
/// // If you need a label, consider drawing the first segment with a label.
/// polyline!("main_surface", closed: [pos1, pos2, pos3], 5.0, Base, "--");
/// polyline!("main_surface", closed: [pos1, pos2, pos3], 5.0, Base);
/// // Draw a triangle (polygon). These can be labelled and the label is placed
/// // in the centeroid of them. Note, this may not be inside of the polygon.
/// // The textsize of the label must be specified before the text.
/// polyline!("main_surface", closed: [pos1, pos2, pos3], 5.0, Base, "--", 12.0, "Position is: x: {}, y: {}", pos1[0], pos1[1]);
/// polyline!("main_surface", closed: [pos1, pos2, pos3], 5.0, Base, "_>");
/// polyline!("main_surface", closed: [pos1, pos2, pos3], 5.0, Base);
/// ```
#[macro_export]
macro_rules! polyline {
    // polyline!(vlogger: my_vlogger, target: "my_target", "my_surface", ([1.0, 2.0], [1.0, 3.0]), 5.0, Base, "-", "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // polyline!(vlogger: my_vlogger, "my_surface", ([1.0, 2.0], [1.0, 3.0]), 5.0, Base, "-", "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    });

    // polyline!(target: "my_target", "my_surface", ([1.0, 2.0], [1.0, 3.0]), 5.0, Base, "-", "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__line!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // polyline!("my_surface", ([1.0, 2.0], [1.0, 3.0]), 5.0, Base, "-", "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__line!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    )
}

/// Sends an arrow or multiple arrows to the vlogger.
///
/// # Examples
///
/// ```
/// use v_log::arrow;
///
/// // Points must be of the same type for arrays to work,
/// // but are only required to implement IntoIterator.
/// // They can be arbitrary dimension, but only the first 3 are used.
/// let pos = [3.234, -1.223];
/// let dir = [0.7071, 0.7071];
/// let space = [[1.0, 0.0], [0.0, 1.0]];
///
/// // Draw an arrow from `pos` pointing in direction `dir` with length 50.0 and thickness 5.0.
/// // If `dir` has length zero or zero length is specified, a zero length arrow will get sent.
/// // The label may get displayed, but the size, position and and format is up
/// // to the vlogger implementation. E.g. it may only be displayed as a tooltip.
/// # #[cfg(feature = "std")]
/// arrow!("main_surface", pos, dir, (50.0), 5.0, Base, "Position is: x: {}, y: {}", pos[0], pos[1]);
/// # #[cfg(feature = "std")]
/// arrow!("main_surface", pos, dir, (50.0), 5.0, Base);
/// // Draw an arrow from `pos` pointing in direction `dir` with the length of `dir` and thickness 5.0.
/// arrow!("main_surface", pos, dir, 5.0, Base, "Position is: x: {}, y: {}", pos[0], pos[1]);
/// arrow!("main_surface", pos, dir, 5.0, Base);
/// // Draw multiple arrows from `pos` and scale them with 50.0.
/// // The colors are X, Y, Z and then "Missing" for all others.
/// arrow!("main_surface", pos, space, (50.0), 5.0);
/// ```
#[macro_export]
macro_rules! arrow {
    // arrow!(vlogger: my_vlogger, target: "my_target", "my_surface", [1.0, 2.0], [1.0, 3.0], 5.0, Base, "a {} event", "log")
    (vlogger: $vlogger:expr, target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__arrow!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // arrow!(vlogger: my_vlogger, "my_surface", [1.0, 2.0], [1.0, 3.0], 5.0, Base, "a {} event", "log")
    (vlogger: $vlogger:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__arrow!(
            $crate::__vlog_vlogger!($vlogger),
            $surface,
            &$crate::__abs_module_path!(),
            $($arg)+
        )
    });

    // arrow!(target: "my_target", "my_surface", [1.0, 2.0], [1.0, 3.0], 5.0, Base, "a {} event", "log")
    (target: $target:expr, $surface:expr, $($arg:tt)+) => ({
        $crate::__arrow!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!($target),
            $($arg)+
        )
    });

    // arrow!("my_surface", [1.0, 2.0], [1.0, 3.0], 5.0, Base, "a {} event", "log")
    ($surface:expr, $($arg:tt)+) => (
        $crate::__arrow!(
            $crate::__vlog_vlogger!(__vlog_global_vlogger),
            $surface,
            &$crate::__abs_module_path!(),
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
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $pos2:expr), $size:expr, $color:tt, $style:tt) => {
        $crate::__line!($vlogger, $surface, $loc, ($pos1, $pos2), $size, $color, $style, "");
    };
    ($vlogger:expr, $surface:expr, $loc:expr, ($pos1:expr, $pos2:expr), $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, ($pos1, $pos2), $size, $color, "-", "");
    };
    ($vlogger:expr, $surface:expr, $loc:expr, closed: $point_list:expr, $size:expr, $color:tt, $style:tt, $textsize:expr, $($arg:tt)+) => {
        $crate::__private_api::vlog_closed_line(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $point_list,
            $size,
            $textsize,
            $crate::__color!($color),
            $crate::__line_style!($style),
            $surface,
            $loc
        );
    };
    ($vlogger:expr, $surface:expr, $loc:expr, closed: $point_list:expr, $size:expr, $color:tt, $style:tt) => {
        $crate::__line!($vlogger, $surface, $loc, closed: $point_list, $size, $color, $style, 0.0, "");
    };
    ($vlogger:expr, $surface:expr, $loc:expr, closed: $point_list:expr, $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, closed: $point_list, $size, $color, "-", 0.0, "");
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $point_list:expr, $size:expr, $color:tt, $style:tt) => {
        let mut last = None;
        let col = $crate::__color!($color);
        let line_style = $crate::__line_style!($style);
        let mut count = 0;
        for p in $point_list {
            if let Some(f) = last {
                $crate::__private_api::vlog_line(
                    $vlogger,
                    $crate::__private_api::format_args!(""),
                    f,
                    p,
                    $size,
                    col,
                    line_style,
                    $surface,
                    $loc
                );
            }
            last = Some(p);
            count += 1;
        }
        assert!(count >= 2);
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $point_list:expr, $size:expr, $color:tt) => {
        $crate::__line!($vlogger, $surface, $loc, $point_list, $size, $color, "-");
    };
}

#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __arrow {
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $dir:expr, ($len:expr), $size:expr, $color:tt, $($arg:tt)+) => {
        #[cfg(feature = "std")]
        {
            $crate::__private_api::vlog_arrow(
                $vlogger,
                $crate::__private_api::format_args!($($arg)+),
                $pos,
                $dir,
                Some($len),
                $size,
                $crate::__color!($color),
                $surface,
                $loc
            )
        }
        #[cfg(not(feature = "std"))]
        {
            compile_error!("std required for arrow macro with fixed length")
        }
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $dir:expr, ($len:expr), $size:expr, $color:tt) => {
        $crate::__arrow!($vlogger, $surface, $loc, $pos, $dir, ($len), $size, $color, "")
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $dir:expr, $size:expr, $color:tt, $($arg:tt)+) => {
        $crate::__private_api::vlog_arrow(
            $vlogger,
            $crate::__private_api::format_args!($($arg)+),
            $pos,
            $dir,
            None,
            $size,
            $crate::__color!($color),
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $dirs:expr, ($scale:expr), $size:expr) => {
        $crate::__private_api::vlog_axis(
            $vlogger,
            $pos,
            $dirs,
            $scale,
            $size,
            $surface,
            $loc
        )
    };
    ($vlogger:expr, $surface:expr, $loc:expr, $pos:expr, $dir:expr, $size:expr, $color:tt) => {
        $crate::__arrow!($vlogger, $surface, $loc, $pos, $dir, $size, $color, "")
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
        &$crate::__private_api::GlobalVLogger
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
    ("S") => {
        $crate::PointStyle::FilledSquare
    };
    ("-S") => {
        $crate::PointStyle::Square
    };
    ("--S") => {
        $crate::PointStyle::DashedSquare
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
    ($s:ident) => {{
        use $crate::PointStyle::*;
        $s
    }};
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
    ($s:expr) => {{
        use $crate::LineStyle::*;
        $s
    }};
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
    ($a:expr) => {{
        use $crate::TextAlignment::*;
        $a
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __color {
    ($hex:literal) => {
        $crate::Color::Hex($hex)
    };
    ($name:expr) => {{
        use $crate::Color::*;
        $name
    }};
}
