// Copyright 2026 redweasel. Based on the `log` crate by the Rust Project Developers Copyright 2015.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A lightweight visual vlogging/debugging facade. Useful for geometry applications.
//!
//! The `v-log` crate provides a single vlogging API that abstracts over the
//! actual vlogging implementation. Libraries can use the vlogging API provided
//! by this crate, and the consumer of those libraries can choose the vlogging
//! implementation that is most suitable for its use case.
//!
//! If no vlogging implementation is selected, the facade falls back to a "noop"
//! implementation, which has very little overhead.
//!
//! A vlog request consists of a _target_, a _surface_, and a _visual_. A target is a
//! string which defaults to the module path of the location of the vlog request,
//! though that default may be overridden. Vlogger implementations typically use
//! the target to filter requests based on some user configuration. A surface is
//! a space or context in which the drawing is done. Vlogger implementations may
//! choose different ways to represent them, but any named surface must be either
//! filtered out or displayed by the implementation. There is no global "main-surface",
//! as drawing surfaces/spaces are created on demand. One can think of these as
//! plot figures or desktop windows.
//!
//! # Usage
//!
//! The basic use of the vlog crate is through the vlogging macros:
//! [`point!`], [`polyline!`], [`message!`], [`label!`], [`clear!`].
//! They form the basic building blocks of drawing.

#![warn(missing_docs)]
#![deny(missing_debug_implementations, unconditional_recursion)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

#[cfg(feature = "std")]
use std::error;
use std::fmt;

#[cfg(target_has_atomic = "ptr")]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(target_has_atomic = "ptr"))]
use std::cell::Cell;
#[cfg(not(target_has_atomic = "ptr"))]
use std::sync::atomic::Ordering;

#[macro_use]
mod macros;
#[doc(hidden)]
pub mod __private_api;

#[cfg(not(target_has_atomic = "ptr"))]
struct AtomicUsize {
    v: Cell<usize>,
}

#[cfg(not(target_has_atomic = "ptr"))]
impl AtomicUsize {
    const fn new(v: usize) -> AtomicUsize {
        AtomicUsize { v: Cell::new(v) }
    }

    fn load(&self, _order: Ordering) -> usize {
        self.v.get()
    }

    fn store(&self, val: usize, _order: Ordering) {
        self.v.set(val)
    }
}

// Any platform without atomics is unlikely to have multiple cores, so
// writing via Cell will not be a race condition.
#[cfg(not(target_has_atomic = "ptr"))]
unsafe impl Sync for AtomicUsize {}

// The VLOGGER static holds a pointer to the global vlogger. It is protected by
// the STATE static which determines whether VLOGGER has been initialized yet.
static mut VLOGGER: &dyn VLog = &NopVLogger;

static STATE: AtomicUsize = AtomicUsize::new(0);

// There are three different states that we care about: the vlogger's
// uninitialized, the vlogger's initializing (set_vlogger's been called but
// VLOGGER hasn't actually been set yet), or the vlogger's active.
const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;

static SET_VLOGGER_ERROR: &str = "attempted to set a vlogger after the vlogging system \
                                 was already initialized";

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum MaybeStaticStr<'a> {
    Static(&'static str),
    Borrowed(&'a str),
}

impl<'a> MaybeStaticStr<'a> {
    #[inline]
    fn get(&self) -> &'a str {
        match *self {
            MaybeStaticStr::Static(s) => s,
            MaybeStaticStr::Borrowed(s) => s,
        }
    }
}

/// The "payload" of a vlog command.
///
/// # Use
///
/// `Record` structures are passed as parameters to the [`vlog`][VLog::vlog]
/// method of the [`VLog`] trait. Vlogger implementors manipulate these
/// structures in order to display vlog commands. `Record`s are automatically
/// created by the macros and so are not seen by vlog users.
#[derive(Clone, Debug)]
pub struct Record<'a> {
    metadata: Metadata<'a>,
    visual: Visual,
    color: Color,
    size: f64,
    args: fmt::Arguments<'a>,
    module_path: Option<MaybeStaticStr<'a>>,
    file: Option<MaybeStaticStr<'a>>,
    line: Option<u32>,
}

impl<'a> Record<'a> {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> RecordBuilder<'a> {
        RecordBuilder::new()
    }

    /// The message/label text.
    #[inline]
    pub fn args(&self) -> &fmt::Arguments<'a> {
        &self.args
    }

    /// The visual element to draw.
    #[inline]
    pub fn visual(&self) -> &Visual {
        &self.visual
    }

    /// The color of the visual element.
    #[inline]
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// The size of the visual element.
    #[inline]
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Metadata about the vlog directive.
    #[inline]
    pub fn metadata(&self) -> &Metadata<'a> {
        &self.metadata
    }

    /// The name of the target of the directive.
    #[inline]
    pub fn target(&self) -> &'a str {
        self.metadata.target()
    }

    /// The name of the surface of the directive.
    #[inline]
    pub fn surface(&self) -> &'a str {
        self.metadata.surface()
    }

    /// The module path of the message.
    #[inline]
    pub fn module_path(&self) -> Option<&'a str> {
        self.module_path.map(|s| s.get())
    }

    /// The module path of the message, if it is a `'static` string.
    #[inline]
    pub fn module_path_static(&self) -> Option<&'static str> {
        match self.module_path {
            Some(MaybeStaticStr::Static(s)) => Some(s),
            _ => None,
        }
    }

    /// The source file containing the message.
    #[inline]
    pub fn file(&self) -> Option<&'a str> {
        self.file.map(|s| s.get())
    }

    /// The source file containing the message, if it is a `'static` string.
    #[inline]
    pub fn file_static(&self) -> Option<&'static str> {
        match self.file {
            Some(MaybeStaticStr::Static(s)) => Some(s),
            _ => None,
        }
    }

    /// The line containing the message.
    #[inline]
    pub fn line(&self) -> Option<u32> {
        self.line
    }
}

/// Builder for [`Record`](struct.Record.html).
///
/// Typically should only be used by vlog library creators or for testing and "shim vloggers".
/// The `RecordBuilder` can set the different parameters of `Record` object, and returns
/// the created object when `build` is called.
///
/// # Examples
///
/// ```
/// use v_log::{Record, Visual, Color};
///
/// let record = Record::builder()
///                 .args(format_args!("Error!"))
///                 .target("myApp")
///                 .surface("AppSurface")
///                 .visual(Visual::Message)
///                 .color(Color::Healthy)
///                 .file(Some("server.rs"))
///                 .line(Some(144))
///                 .module_path(Some("server"))
///                 .build();
/// ```
///
/// Alternatively, use [`MetadataBuilder`](struct.MetadataBuilder.html):
///
/// ```
/// use v_log::{Record, Visual, Color, MetadataBuilder};
///
/// let error_metadata = MetadataBuilder::new()
///                         .target("myApp")
///                         .surface("AppSurface")
///                         .build();
///
/// let record = Record::builder()
///                 .metadata(error_metadata)
///                 .args(format_args!("Error!"))
///                 .visual(Visual::Message)
///                 .color(Color::Healthy)
///                 .line(Some(433))
///                 .file(Some("app.rs"))
///                 .module_path(Some("server"))
///                 .build();
/// ```
#[derive(Debug)]
pub struct RecordBuilder<'a> {
    record: Record<'a>,
}

impl<'a> RecordBuilder<'a> {
    /// Construct new `RecordBuilder`.
    ///
    /// The default options are:
    ///
    /// - `visual`: [`Visual::Message`]
    /// - `color`: [`Color::Base`]
    /// - `size`: `12.0`
    /// - `args`: [`format_args!("")`]
    /// - `metadata`: [`Metadata::builder().build()`]
    /// - `module_path`: `None`
    /// - `file`: `None`
    /// - `line`: `None`
    ///
    /// [`format_args!("")`]: https://doc.rust-lang.org/std/macro.format_args.html
    /// [`Metadata::builder().build()`]: struct.MetadataBuilder.html#method.build
    #[inline]
    pub fn new() -> RecordBuilder<'a> {
        RecordBuilder {
            record: Record {
                visual: Visual::Message,
                color: Color::Base,
                size: 12.0,
                args: format_args!(""),
                metadata: Metadata::builder().build(),
                module_path: None,
                file: None,
                line: None,
            },
        }
    }

    /// Set [`visual`](struct.Record.html#method.visual).
    pub fn visual(&mut self, visual: Visual) -> &mut RecordBuilder<'a> {
        self.record.visual = visual;
        self
    }

    /// Set [`color`](struct.Record.html#method.color).
    pub fn color(&mut self, color: Color) -> &mut RecordBuilder<'a> {
        self.record.color = color;
        self
    }

    /// Set [`size`](struct.Record.html#method.size).
    pub fn size(&mut self, size: f64) -> &mut RecordBuilder<'a> {
        self.record.size = size;
        self
    }

    /// Set [`args`](struct.Record.html#method.args).
    #[inline]
    pub fn args(&mut self, args: fmt::Arguments<'a>) -> &mut RecordBuilder<'a> {
        self.record.args = args;
        self
    }

    /// Set [`metadata`](struct.Record.html#method.metadata). Construct a `Metadata` object with [`MetadataBuilder`](struct.MetadataBuilder.html).
    #[inline]
    pub fn metadata(&mut self, metadata: Metadata<'a>) -> &mut RecordBuilder<'a> {
        self.record.metadata = metadata;
        self
    }

    /// Set [`Metadata::surface`](struct.Metadata.html#method.surface).
    #[inline]
    pub fn surface(&mut self, surface: &'a str) -> &mut RecordBuilder<'a> {
        self.record.metadata.surface = surface;
        self
    }

    /// Set [`Metadata::target`](struct.Metadata.html#method.target)
    #[inline]
    pub fn target(&mut self, target: &'a str) -> &mut RecordBuilder<'a> {
        self.record.metadata.target = target;
        self
    }

    /// Set [`module_path`](struct.Record.html#method.module_path)
    #[inline]
    pub fn module_path(&mut self, path: Option<&'a str>) -> &mut RecordBuilder<'a> {
        self.record.module_path = path.map(MaybeStaticStr::Borrowed);
        self
    }

    /// Set [`module_path`](struct.Record.html#method.module_path) to a `'static` string
    #[inline]
    pub fn module_path_static(&mut self, path: Option<&'static str>) -> &mut RecordBuilder<'a> {
        self.record.module_path = path.map(MaybeStaticStr::Static);
        self
    }

    /// Set [`file`](struct.Record.html#method.file)
    #[inline]
    pub fn file(&mut self, file: Option<&'a str>) -> &mut RecordBuilder<'a> {
        self.record.file = file.map(MaybeStaticStr::Borrowed);
        self
    }

    /// Set [`file`](struct.Record.html#method.file) to a `'static` string.
    #[inline]
    pub fn file_static(&mut self, file: Option<&'static str>) -> &mut RecordBuilder<'a> {
        self.record.file = file.map(MaybeStaticStr::Static);
        self
    }

    /// Set [`line`](struct.Record.html#method.line)
    #[inline]
    pub fn line(&mut self, line: Option<u32>) -> &mut RecordBuilder<'a> {
        self.record.line = line;
        self
    }

    /// Invoke the builder and return a `Record`
    #[inline]
    pub fn build(&self) -> Record<'a> {
        self.record.clone()
    }
}

impl Default for RecordBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about a vlog command.
///
/// # Use
///
/// `Metadata` structs are created when users of the library use
/// vlogging macros.
///
/// They are consumed by implementations of the `VLog` trait in the
/// `enabled` method.
///
/// Users should use the `vlog_enabled!` macro in their code to avoid
/// constructing expensive vlog messages.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Metadata<'a> {
    surface: &'a str,
    target: &'a str,
}

impl<'a> Metadata<'a> {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> MetadataBuilder<'a> {
        MetadataBuilder::new()
    }

    /// The surface to draw on.
    #[inline]
    pub fn surface(&self) -> &'a str {
        self.surface
    }

    /// The name of the target of the directive.
    #[inline]
    pub fn target(&self) -> &'a str {
        self.target
    }
}

/// Builder for [`Metadata`](struct.Metadata.html).
///
/// Typically should only be used by vlog library creators or for testing and "shim vloggers".
/// The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
/// the created object when `build` is called.
///
/// # Example
///
/// ```
/// let target = "myApp";
/// let surface = "AppSurface";
/// use v_log::MetadataBuilder;
/// let metadata = MetadataBuilder::new()
///                     .surface(surface)
///                     .target(target)
///                     .build();
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MetadataBuilder<'a> {
    metadata: Metadata<'a>,
}

impl<'a> MetadataBuilder<'a> {
    /// Construct a new `MetadataBuilder`.
    ///
    /// The default options are:
    ///
    /// - `surface`: `""`
    /// - `target`: `""`
    #[inline]
    pub fn new() -> MetadataBuilder<'a> {
        MetadataBuilder {
            metadata: Metadata {
                surface: "",
                target: "",
            },
        }
    }

    /// Setter for [`surface`](struct.Metadata.html#method.surface).
    #[inline]
    pub fn surface(&mut self, surface: &'a str) -> &mut MetadataBuilder<'a> {
        self.metadata.surface = surface;
        self
    }

    /// Setter for [`target`](struct.Metadata.html#method.target).
    #[inline]
    pub fn target(&mut self, target: &'a str) -> &mut MetadataBuilder<'a> {
        self.metadata.target = target;
        self
    }

    /// Returns a `Metadata` object.
    #[inline]
    pub fn build(&self) -> Metadata<'a> {
        self.metadata.clone()
    }
}

impl Default for MetadataBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// The style of a point type visual. There is two distinct types of styles.
///
/// 1. Circle with absolute size: [`FilledCircle`](`PointStyle::FilledCircle`), [`Circle`](`PointStyle::Circle`), [`DashedCircle`](`PointStyle::DashedCircle`), [`FilledSquare`](`PointStyle::FilledSquare`), [`Square`](`PointStyle::Square`), [`DashedSquare`](`PointStyle::DashedSquare`).
///    These are useful to draw circles/squares with a fixed size. In a 3D context these represent spheres/cubes instead.
///    The circle outline uses the correct sphere outline in the used view projection, which means they become
///    ellipses/hyperbolas in a perspective projection. The outlined cube is preferrably drawn as a wireframe cube.
/// 2. Point billboard marker where the size is determined in screen coordinates instead of the same space as the position coordinates.
///    Zooming in the view will not change their apparent size. These are useful to mark points.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum PointStyle {
    /* 2D/3D objects */

    /// A filled circle/sphere. [`size`](struct.Record.html#method.size) is the diameter.
    FilledCircle,
    /// A circle/sphere outline. [`size`](struct.Record.html#method.size) is the diameter.
    Circle,
    /// A dashed circle/sphere outline. [`size`](struct.Record.html#method.size) is the diameter.
    DashedCircle,
    /// A filled square/cube. [`size`](struct.Record.html#method.size) is the width.
    FilledSquare,
    /// A square/cube outline/wireframe. [`size`](struct.Record.html#method.size) is the width.
    Square,
    /// A dashed square/cube outline/wireframe. [`size`](struct.Record.html#method.size) is the width.
    DashedSquare,

    /* 2D markers */

    /// A filled circle. Dynamically scaled so the size is the pixel size.
    Point,
    /// A circle outline. Dynamically scaled so the size is the pixel size.
    PointOutline,
    /// A filled square. Dynamically scaled so the size is the pixel size.
    PointSquare,
    /// A square outline. Dynamically scaled so the size is the pixel size.
    PointSquareOutline,
    /// An `x` marker. Dynamically scaled so the size is the pixel size.
    PointCross,
    /// A filled diamond. Dynamically scaled so the size is the pixel size.
    PointDiamond,
    /// A diamond outline. Dynamically scaled so the size is the pixel size.
    PointDiamondOutline,
}

/// The style of a line type visual.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum LineStyle {
    /// A simple straight continuous line
    Simple,
    /// A dashed line
    Dashed,
    /// A line with an arrowhead on the second point.
    Arrow,
    /// A line with half an arrowhead on the second point or along the line. If a polygon is drawn in CCW point order, the harpoon will be on the inside.
    InsideHarpoonCCW,
    /// A line with half an arrowhead on the second point or along the line. If a polygon is drawn in CW point order, the harpoon will be on the inside.
    InsideHarpoonCW,
}

/// The text alignment relative to a specified spacepoint.
/// All variants center the text vertically.
#[derive(Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum TextAlignment {
    /// Align the left side of the text to the position. Vertically centered.
    Left = 0,
    /// Center the text on the position.
    Center = 1,
    /// Align the right side of the text to the position. Vertically centered.
    Right = 2,
    /// Center the text on the position if possible, but the vlogger is allowed
    /// to shift the text by a small amount for better readability.
    #[default]
    Flexible = 3,
}

/// A visual element to be drawn by the vlogger.
#[derive(Clone, Debug, Default)]
pub enum Visual {
    /// Just a vlog message to be shown in the vlogger instead of the regular vlogs.
    #[default]
    Message,
    /// A text label placed in space with the message string.
    Label {
        /// The spacepoint x-coordinate
        x: f64,
        /// The spacepoint y-coordinate
        y: f64,
        /// The spacepoint z-coordinate for 3D visualisations.
        z: f64,
        /// The alignment of the text relative to the spacepoint.
        alignment: TextAlignment,
    },
    /// A circle/point placed in space.
    Point {
        /// The spacepoint x-coordinate
        x: f64,
        /// The spacepoint y-coordinate
        y: f64,
        /// The spacepoint z-coordinate for 3D visualisations.
        z: f64,
        /// The drawing style of the circle/point.
        style: PointStyle,
    },
    /// A line placed in space.
    Line {
        /// The 1. spacepoint x-coordinate
        x1: f64,
        /// The 1. spacepoint y-coordinate
        y1: f64,
        /// The 1. spacepoint z-coordinate for 3D visualisations.
        z1: f64,
        /// The 2. spacepoint x-coordinate
        x2: f64,
        /// The 2. spacepoint y-coordinate
        y2: f64,
        /// The 2. spacepoint z-coordinate for 3D visualisations.
        z2: f64,
        /// The drawing style of the line.
        style: LineStyle,
    },
}

/// Basic debugging theme colors.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub enum Color {
    /// Base line color. E.g. white on black background.
    #[default]
    Base,
    /// Some shade of green.
    Healthy,
    /// Some shade of blue.
    Info,
    /// Some shade of yellow.
    Warn,
    /// Some shade of red.
    Error,
    /// Some shade of red (**R**gb = **X**YZ)
    X,
    /// Some shade of green (r**G**b = X**Y**Z)
    Y,
    /// Some shade of blue (rg**B** = XY**Z**)
    Z,
    /// A specific color by hexcode. The MSB is red, the LSB is alpha.
    Hex(u32),
}

/// A trait encapsulating the operations required of a vlogger.
pub trait VLog {
    /// Determines if a vlog command with the specified metadata would be
    /// vlogged.
    ///
    /// This is used by the `vlog_enabled!` macro to allow callers to avoid
    /// expensive computation of vlog message arguments if the message would be
    /// discarded anyway.
    fn enabled(&self, metadata: &Metadata) -> bool;
    /// Draw a point or line in 3D or 2D (ignoring z or using it as z-index).
    ///
    /// This is only called if the vlogging of messages with the given metadata is enabled.
    fn vlog(&self, record: &Record);
    /// Clear a drawing surface e.g. to redraw its content.
    fn clear(&self, surface: &str);
}

/// A dummy initial value for VLOGGER.
struct NopVLogger;

impl VLog for NopVLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        false
    }

    fn vlog(&self, _: &Record) {}
    fn clear(&self, _: &str) {}
}

impl<T> VLog for &'_ T
where
    T: ?Sized + VLog,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        (**self).enabled(metadata)
    }

    fn vlog(&self, record: &Record) {
        (**self).vlog(record);
    }

    fn clear(&self, surface: &str) {
        (**self).clear(surface);
    }
}

#[cfg(feature = "std")]
impl<T> VLog for std::boxed::Box<T>
where
    T: ?Sized + VLog,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.as_ref().enabled(metadata)
    }

    fn vlog(&self, record: &Record) {
        self.as_ref().vlog(record);
    }

    fn clear(&self, surface: &str) {
        self.as_ref().clear(surface);
    }
}

#[cfg(feature = "std")]
impl<T> VLog for std::sync::Arc<T>
where
    T: ?Sized + VLog,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.as_ref().enabled(metadata)
    }

    fn vlog(&self, record: &Record) {
        self.as_ref().vlog(record);
    }

    fn clear(&self, surface: &str) {
        self.as_ref().clear(surface);
    }
}

/// Sets the global vlogger to a `Box<VLog>`.
///
/// This is a simple convenience wrapper over `set_vlogger`, which takes a
/// `Box<VLog>` rather than a `&'static VLog`. See the documentation for
/// [`set_vlogger`] for more details.
///
/// Requires the `std` feature.
///
/// # Errors
///
/// An error is returned if a vlogger has already been set.
///
/// [`set_vlogger`]: fn.set_vlogger.html
#[cfg(all(feature = "std", target_has_atomic = "ptr"))]
pub fn set_boxed_vlogger(vlogger: Box<dyn VLog>) -> Result<(), SetVLoggerError> {
    set_vlogger_inner(|| Box::leak(vlogger))
}

/// Sets the global vlogger to a `&'static VLog`.
///
/// This function may only be called once in the lifetime of a program. Any vlog
/// events that occur before the call to `set_vlogger` completes will be ignored.
///
/// This function does not typically need to be called manually. VLogger
/// implementations should provide an initialization method that installs the
/// vlogger internally.
///
/// # Availability
///
/// This method is available even when the `std` feature is disabled. However,
/// it is currently unavailable on `thumbv6` targets, which lack support for
/// some atomic operations which are used by this function. Even on those
/// targets, [`set_vlogger_racy`] will be available.
///
/// # Errors
///
/// An error is returned if a vlogger has already been set.
///
/// # Examples
///
/// ```ignore
/// use v_log::{message, Record, Metadata};
///
/// static MY_VLOGGER: MyVLogger = MyVLogger;
///
/// struct MyVLogger;
///
/// impl v_log::VLog for MyVLogger {...}
///
/// # fn main(){
/// v_log::set_vlogger(&MY_VLOGGER).unwrap();
///
/// message!("hello vlog");
/// # }
/// ```
///
/// [`set_vlogger_racy`]: fn.set_vlogger_racy.html
#[cfg(target_has_atomic = "ptr")]
pub fn set_vlogger(vlogger: &'static dyn VLog) -> Result<(), SetVLoggerError> {
    set_vlogger_inner(|| vlogger)
}

#[cfg(target_has_atomic = "ptr")]
fn set_vlogger_inner<F>(make_vlogger: F) -> Result<(), SetVLoggerError>
where
    F: FnOnce() -> &'static dyn VLog,
{
    match STATE.compare_exchange(
        UNINITIALIZED,
        INITIALIZING,
        Ordering::Acquire,
        Ordering::Relaxed,
    ) {
        Ok(UNINITIALIZED) => {
            unsafe {
                VLOGGER = make_vlogger();
            }
            STATE.store(INITIALIZED, Ordering::Release);
            Ok(())
        }
        Err(INITIALIZING) => {
            while STATE.load(Ordering::Relaxed) == INITIALIZING {
                std::hint::spin_loop();
            }
            Err(SetVLoggerError(()))
        }
        _ => Err(SetVLoggerError(())),
    }
}

/// A thread-unsafe version of [`set_vlogger`].
///
/// This function is available on all platforms, even those that do not have
/// support for atomics that is needed by [`set_vlogger`].
///
/// In almost all cases, [`set_vlogger`] should be preferred.
///
/// # Safety
///
/// This function is only safe to call when it cannot race with any other
/// calls to `set_vlogger` or `set_vlogger_racy`.
///
/// This can be upheld by (for example) making sure that **there are no other
/// threads**, and (on embedded) that **interrupts are disabled**.
///
/// It is safe to use other vlogging functions while this function runs
/// (including all vlogging macros).
///
/// [`set_vlogger`]: fn.set_vlogger.html
pub unsafe fn set_vlogger_racy(vlogger: &'static dyn VLog) -> Result<(), SetVLoggerError> {
    match STATE.load(Ordering::Acquire) {
        UNINITIALIZED => {
            unsafe {
                VLOGGER = vlogger;
            }
            STATE.store(INITIALIZED, Ordering::Release);
            Ok(())
        }
        INITIALIZING => {
            // This is just plain UB, since we were racing another initialization function
            unreachable!("set_vlogger_racy must not be used with other initialization functions")
        }
        _ => Err(SetVLoggerError(())),
    }
}

/// The type returned by [`set_vlogger`] if [`set_vlogger`] has already been called.
///
/// [`set_vlogger`]: fn.set_vlogger.html
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct SetVLoggerError(());

impl fmt::Display for SetVLoggerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(SET_VLOGGER_ERROR)
    }
}

// The Error trait is not available in libcore
#[cfg(feature = "std")]
impl error::Error for SetVLoggerError {}

/// Returns a reference to the vlogger.
///
/// If a vlogger has not been set, a no-op implementation is returned.
pub fn vlogger() -> &'static dyn VLog {
    // Acquire memory ordering guarantees that current thread would see any
    // memory writes that happened before store of the value
    // into `STATE` with memory ordering `Release` or stronger.
    //
    // Since the value `INITIALIZED` is written only after `VLOGGER` was
    // initialized, observing it after `Acquire` load here makes both
    // write to the `VLOGGER` static and initialization of the vlogger
    // internal state synchronized with current thread.
    if STATE.load(Ordering::Acquire) != INITIALIZED {
        static NOP: NopVLogger = NopVLogger;
        &NOP
    } else {
        unsafe { VLOGGER }
    }
}
