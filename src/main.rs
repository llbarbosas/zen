#[derive(Clone)]
enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone)]
enum Basis {
    Auto,
    Value(f32),
}

#[derive(Clone)]
enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone)]
enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone)]
enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone)]
struct Spacing {
    top: f64,
    left: f64,
    bottom: f64,
    right: f64,
}

impl Spacing {
    fn all(value: f64) -> Self {
        Spacing {
            top: value,
            left: value,
            right: value,
            bottom: value,
        }
    }

    fn top(top: f64) -> Self {
        Spacing {
            top,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }

    fn left(left: f64) -> Self {
        Spacing {
            top: 0.0,
            left,
            right: 0.0,
            bottom: 0.0,
        }
    }

    fn right(right: f64) -> Self {
        Spacing {
            top: 0.0,
            left: 0.0,
            right,
            bottom: 0.0,
        }
    }

    fn bottom(bottom: f64) -> Self {
        Spacing {
            top: 0.0,
            left: 0.0,
            right: 0.0,
            bottom,
        }
    }

    fn tlrb(top: f64, left: f64, right: f64, bottom: f64) -> Self {
        Spacing {
            top,
            left,
            right,
            bottom,
        }
    }
}

#[derive(Clone)]
struct Node {
    direction: Direction,
    basis: Basis,
    grow: f32,
    shrink: f32,
    wrap: Wrap,
    justify_content: JustifyContent,
    align_items: Align,
    align_self: Align,
    align_content: Align,
    width: f64,
    height: f64,
    max_width: f64,
    max_height: f64,
    min_width: f64,
    min_height: f64,
    aspect_ratio: f64,
    padding: Spacing,
    margin: Spacing,
    border: Spacing,
    children: Box<[Node]>,
}

impl Node {
    fn new() -> Node {
        Node {
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
            children: Box::from([]),
        }
    }

    fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    fn basis(mut self, basis: Basis) -> Self {
        self.basis = basis;
        self
    }

    fn grow(mut self, grow: f32) -> Self {
        self.grow = grow;
        self
    }

    fn shrink(mut self, shrink: f32) -> Self {
        self.shrink = shrink;
        self
    }

    fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    fn align_items(mut self, align_items: Align) -> Self {
        self.align_items = align_items;
        self
    }

    fn align_self(mut self, align_self: Align) -> Self {
        self.align_self = align_self;
        self
    }

    fn align_content(mut self, align_content: Align) -> Self {
        self.align_content = align_content;
        self
    }

    fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    fn max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    fn min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    fn padding(mut self, padding: Spacing) -> Self {
        self.padding = padding;
        self
    }

    fn margin(mut self, margin: Spacing) -> Self {
        self.margin = margin;
        self
    }

    fn border(mut self, border: Spacing) -> Self {
        self.border = border;
        self
    }

    fn insert_child(mut self, child: Self) -> Self {
        self.width += child.width;
        self.height += child.height;
        self.children = Box::from([self.children, Box::from([child])].concat());
        self
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node({}x{}, children: {})",
            self.width,
            self.height,
            self.children.len()
        )
    }
}

fn main() {
    let node1 = Node::new().width(100.0).height(100.0);
    let node2 = Node::new().width(300.0).height(400.0);

    let root = Node::new().insert_child(node1).insert_child(node2);

    println!("root: {}", root);
}
