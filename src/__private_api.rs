//! WARNING: this is not part of the crate's public API and is subject to change at any time

use crate::{
    vlogger, Color, LineStyle, Metadata, MetadataBuilder, PointStyle, Record, TextAlignment, VLog,
    Visual,
};
use std::fmt::Arguments;
use std::panic::Location;
pub use std::{format_args, module_path};

// VLog implementation.

#[macro_export]
#[doc(hidden)]
macro_rules! __abs_module_path {
    () => {
        $crate::__abs_module_path!($crate::__private_api::module_path!())
    };
    ($target:expr) => {
        (
            $target,
            {
                // avoid leaking build system absolute file paths into release builds!
                // any feature relying on absolute paths will only work in debug builds.
                #[cfg(debug_assertions)]
                {
                    ::core::concat!(::core::env!("CARGO_MANIFEST_DIR"), "/", ::core::file!())
                }
                #[cfg(not(debug_assertions))]
                {
                    ::core::file!()
                }
            },
            $crate::__private_api::module_path!(),
            $crate::__private_api::loc(),
        )
    };
}

/// The global vlogger proxy.
#[derive(Debug)]
pub struct GlobalVLogger;

impl VLog for GlobalVLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        vlogger().enabled(metadata)
    }

    fn vlog(&self, record: &Record) {
        vlogger().vlog(record)
    }

    fn clear(&self, surface: &str) {
        vlogger().clear(surface)
    }

    fn flush(&self) {
        vlogger().flush()
    }
}

pub fn clear<L>(vlogger: &L, target: &str, surface: &str)
where
    L: VLog,
{
    if vlogger.enabled(
        &MetadataBuilder::new()
            .target(target)
            .surface(surface)
            .build(),
    ) {
        vlogger.clear(&surface);
    }
}

fn vlog<'a, L>(
    vlogger: &L,
    args: Arguments,
    visual: Visual,
    size: f64,
    color: Color,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut builder = Record::builder();
    let (target, file_path, module_path, loc) = target_module_path_and_loc;

    builder
        .args(args)
        .visual(visual)
        .size(size)
        .color(color)
        .surface(surface)
        .target(target)
        .module_path_static(Some(module_path))
        .file(Some(file_path))
        .line(Some(loc.line()));

    vlogger.vlog(&builder.build());
}

pub fn vlog_point<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: &L,
    args: Arguments,
    pos: P,
    diameter: f64,
    color: Color,
    style: PointStyle,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut pos = pos.into_iter();
    vlog(
        vlogger,
        args,
        Visual::Point {
            x: pos.next().unwrap_or(0.0),
            y: pos.next().unwrap_or(0.0),
            z: pos.next().unwrap_or(0.0),
            style,
        },
        diameter,
        color,
        surface,
        target_module_path_and_loc,
    );
}
pub fn vlog_line<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: &L,
    args: Arguments,
    pos1: P,
    pos2: P,
    thickness: f64,
    color: Color,
    style: LineStyle,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut pos1 = pos1.into_iter();
    let mut pos2 = pos2.into_iter();
    vlog(
        vlogger,
        args,
        Visual::Line {
            x1: pos1.next().unwrap_or(0.0),
            y1: pos1.next().unwrap_or(0.0),
            z1: pos1.next().unwrap_or(0.0),
            x2: pos2.next().unwrap_or(0.0),
            y2: pos2.next().unwrap_or(0.0),
            z2: pos2.next().unwrap_or(0.0),
            style,
        },
        thickness,
        color,
        surface,
        target_module_path_and_loc,
    );
}
pub fn vlog_arrow<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: &L,
    args: Arguments,
    pos: P,
    dir: P,
    len: Option<f64>,
    thickness: f64,
    color: Color,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut iter = pos.into_iter();
    let pos = [
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
    ];
    let mut iter = dir.into_iter();
    let mut dir = [
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
    ];
    if let Some(len) = len {
        let mut dir_len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
        if !dir_len.is_finite() {
            // try slower alternative for large values
            dir_len = dir[0].hypot(dir[1]).hypot(dir[2]);
        }
        if dir_len >= 0.0 {
            for d in dir.iter_mut() {
                if d.is_finite() {
                    // don't combine these two for maximal numerical range
                    *d /= dir_len;
                    *d *= len;
                }
            }
        }
    }
    vlog(
        vlogger,
        args,
        Visual::Line {
            x1: pos[0],
            y1: pos[1],
            z1: pos[2],
            x2: pos[0] + dir[0],
            y2: pos[1] + dir[1],
            z2: pos[2] + dir[2],
            style: LineStyle::Arrow,
        },
        thickness,
        color,
        surface,
        target_module_path_and_loc,
    );
}
pub fn vlog_closed_line<'a, P: IntoIterator<Item = f64> + Clone, L>(
    vlogger: &L,
    args: Arguments,
    polygon: impl IntoIterator<Item = P>,
    thickness: f64,
    textsize: f64,
    color: Color,
    style: LineStyle,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut first = None;
    let mut last = None;
    let mut count = 0;
    let mut sum = [0.0; 3];
    for p in polygon.into_iter() {
        if let Some(l) = last.replace(p.clone()) {
            vlog_line(
                vlogger,
                format_args!(""),
                l,
                p.clone(),
                thickness,
                color,
                style,
                surface,
                target_module_path_and_loc,
            );
        } else {
            first = Some(p.clone()); // TODO is this ok?!?
        }
        let mut iter = p.into_iter();
        sum[0] += iter.next().unwrap_or(0.0);
        sum[1] += iter.next().unwrap_or(0.0);
        sum[2] += iter.next().unwrap_or(0.0);
        count += 1;
    }
    assert!(count >= 3);
    vlog_line(
        vlogger,
        format_args!(""),
        last.unwrap(),
        first.unwrap(),
        thickness,
        color,
        style,
        surface,
        target_module_path_and_loc,
    );
    let count = count as f64;
    sum[0] /= count;
    sum[1] /= count;
    sum[2] /= count;
    vlog_label(
        vlogger,
        args,
        sum,
        textsize,
        color,
        TextAlignment::Center,
        surface,
        target_module_path_and_loc,
    );
}
pub fn vlog_axis<'a, P: IntoIterator<Item = f64> + Clone, L>(
    vlogger: &L,
    pos: P,
    dirs: impl IntoIterator<Item = P>,
    scale: f64,
    thickness: f64,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut iter = pos.into_iter();
    let pos = [
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
        iter.next().unwrap_or(0.0),
    ];
    let colors = [Color::X, Color::Y, Color::Z, Color::Missing];
    let mut color_index = 0;
    for dir in dirs {
        let mut iter = dir.into_iter();
        let (dx, dy, dz) = (
            iter.next().map_or(0.0, |v| v * scale),
            iter.next().map_or(0.0, |v| v * scale),
            iter.next().map_or(0.0, |v| v * scale),
        );
        let color = colors[color_index];
        if color_index + 1 < colors.len() {
            color_index += 1;
        }
        vlog_line(
            vlogger,
            format_args!(""),
            pos,
            [pos[0] + dx, pos[1] + dy, pos[2] + dz],
            thickness,
            color,
            LineStyle::Arrow,
            surface,
            target_module_path_and_loc,
        );
    }
}
pub fn vlog_label<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: &L,
    args: Arguments,
    pos: P,
    size: f64,
    color: Color,
    alignment: TextAlignment,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut pos = pos.into_iter();
    vlog(
        vlogger,
        args,
        Visual::Label {
            x: pos.next().unwrap_or(0.0),
            y: pos.next().unwrap_or(0.0),
            z: pos.next().unwrap_or(0.0),
            alignment,
        },
        size,
        color,
        surface,
        target_module_path_and_loc,
    );
}
#[inline(always)]
pub fn vlog_message<'a, L>(
    vlogger: &L,
    args: Arguments,
    color: Color,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static str, &'static Location),
) where
    L: VLog,
{
    vlog(
        vlogger,
        args,
        Visual::Message,
        0.0,
        color,
        surface,
        target_module_path_and_loc,
    );
}

pub fn enabled<L: VLog>(vlogger: L, surface: &str, target: &str) -> bool {
    vlogger.enabled(&Metadata::builder().surface(surface).target(target).build())
}

#[track_caller]
pub fn loc() -> &'static Location<'static> {
    Location::caller()
}
