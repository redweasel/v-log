//! WARNING: this is not part of the crate's public API and is subject to change at any time

use crate::{
    vlogger, Color, LineStyle, Metadata, MetadataBuilder, PointStyle, Record, TextAlignment, VLog,
    Visual,
};
use std::fmt::Arguments;
use std::panic::Location;
pub use std::{format_args, module_path, stringify};

// VLog implementation.

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
}

pub fn clear<L>(vlogger: L, target: &str, surface: &str)
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
    vlogger: L,
    args: Arguments,
    visual: Visual,
    size: f64,
    color: Color,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static Location),
) where
    L: VLog,
{
    let mut builder = Record::builder();
    let (target, module_path, loc) = target_module_path_and_loc;

    builder
        .args(args)
        .visual(visual)
        .size(size)
        .color(color)
        .surface(surface)
        .target(target)
        .module_path_static(Some(module_path))
        .file_static(Some(loc.file()))
        .line(Some(loc.line()));

    vlogger.vlog(&builder.build());
}

pub fn vlog_point<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: L,
    args: Arguments,
    pos: P,
    diameter: f64,
    color: Color,
    style: PointStyle,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static Location),
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
    vlogger: L,
    args: Arguments,
    pos1: P,
    pos2: P,
    thickness: f64,
    color: Color,
    style: LineStyle,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static Location),
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
pub fn vlog_label<'a, P: IntoIterator<Item = f64>, L>(
    vlogger: L,
    args: Arguments,
    pos: P,
    size: f64,
    color: Color,
    alignment: TextAlignment,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static Location),
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
    vlogger: L,
    args: Arguments,
    color: Color,
    surface: &str,
    target_module_path_and_loc: &(&str, &'static str, &'static Location),
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
