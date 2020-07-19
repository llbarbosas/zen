use crate::node::Node;

use std::fmt;

pub enum LayoutDirection {
    Ltr,
    Rtl,
    Inherit,
}

pub struct Layout {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    width: f64,
    height: f64,
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Layout(left: {}, top: {}, width: {}, height: {})",
            self.left, self.top, self.width, self.height
        )
    }
}

impl Node {
    pub fn calculate_layout_d(&self, direction: LayoutDirection) -> Layout {
        Layout {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
            height: 0.0,
            width: 0.0,
        }
    }
    
    pub fn calculate_layout(&self) -> Layout {
        self.calculate_layout_d(LayoutDirection::Ltr)
    }
}
