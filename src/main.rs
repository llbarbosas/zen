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
            children: Box::from([]),
        }
    }

    fn set_width(self, width: f64) -> Self {
        Node { width, ..self }
    }

    fn set_height(self, height: f64) -> Self {
        Node { height, ..self }
    }

    fn set_direction(self, direction: Direction) -> Self {
        Node { direction, ..self }
    }

    fn set_basis(self, basis: Basis) -> Self {
        Node { basis, ..self }
    }

    fn set_grow(self, grow: f32) -> Self {
        Node { grow, ..self }
    }

    fn insert_child(self, child: Self) -> Self {
        Node {
            width: self.width + child.width,
            height: self.height + child.height,
            children: Box::from([self.children, Box::from([child])].concat()),
            ..self
        }
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
    let node1 = Node::new().set_width(100.0).set_height(100.0);
    let node2 = Node::new().set_width(300.0).set_height(400.0);

    let root = Node::new().insert_child(node1).insert_child(node2);

    println!("root: {}", root);
}
