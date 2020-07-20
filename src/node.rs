#![allow(dead_code)]

use crate::id;
use std::fmt;

pub(crate) static INSTANCE_ALLOCATOR: id::Allocator = id::Allocator::new();

#[derive(Clone, Copy)]
pub enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Copy)]
pub enum Basis {
    Auto,
    Value(f32),
}

#[derive(Clone, Copy)]
pub enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy)]
pub enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Copy)]
pub struct Spacing {
    top: f64,
    start: f64,
    bottom: f64,
    end: f64,
}

impl Spacing {
    fn all(value: f64) -> Self {
        Spacing {
            top: value,
            start: value,
            end: value,
            bottom: value,
        }
    }

    fn top(top: f64) -> Self {
        Spacing {
            top,
            start: 0.0,
            end: 0.0,
            bottom: 0.0,
        }
    }

    fn start(start: f64) -> Self {
        Spacing {
            top: 0.0,
            start,
            end: 0.0,
            bottom: 0.0,
        }
    }

    fn end(end: f64) -> Self {
        Spacing {
            top: 0.0,
            start: 0.0,
            end,
            bottom: 0.0,
        }
    }

    fn bottom(bottom: f64) -> Self {
        Spacing {
            top: 0.0,
            start: 0.0,
            end: 0.0,
            bottom,
        }
    }

    fn tseb(top: f64, start: f64, end: f64, bottom: f64) -> Self {
        Spacing {
            top,
            start,
            end,
            bottom,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Node {
    pub id: id::NodeId,
    direction: Direction,
    basis: Basis,
    grow: f32,
    shrink: f32,
    wrap: Wrap,
    justify_content: JustifyContent,
    align_items: Align,
    align_self: Align,
    align_content: Align,
    pub width: f64,
    height: f64,
    max_width: f64,
    max_height: f64,
    min_width: f64,
    min_height: f64,
    aspect_ratio: f64,
    padding: Spacing,
    margin: Spacing,
    border: Spacing,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: INSTANCE_ALLOCATOR.allocate(),
            direction: Direction::Row,
            basis: Basis::Auto,
            grow: 0.0,
            shrink: 0.0,
            wrap: Wrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: Align::Auto,
            align_self: Align::Auto,
            align_content: Align::Auto,
            width: 0.0,
            height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            aspect_ratio: 0.0,
            padding: Spacing::all(0.0),
            margin: Spacing::all(0.0),
            border: Spacing::all(0.0),
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn basis(mut self, basis: Basis) -> Self {
        self.basis = basis;
        self
    }

    pub fn grow(mut self, grow: f32) -> Self {
        self.grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: f32) -> Self {
        self.shrink = shrink;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: Align) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn align_self(mut self, align_self: Align) -> Self {
        self.align_self = align_self;
        self
    }

    pub fn align_content(mut self, align_content: Align) -> Self {
        self.align_content = align_content;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn padding(mut self, padding: Spacing) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Spacing) -> Self {
        self.margin = margin;
        self
    }

    pub fn border(mut self, border: Spacing) -> Self {
        self.border = border;
        self
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.id)
    }
}
