use crate::node::Node;

use crate::id::NodeId;
use std::fmt;

use std::collections::HashMap;

pub struct RenderTree {
    is_child_of: HashMap<NodeId, NodeId>,
    node_info: HashMap<NodeId, Node>,
}

impl RenderTree {
    pub fn new() -> Self {
        RenderTree {
            is_child_of: HashMap::new(),
            node_info: HashMap::new(),
        }
    }

    pub fn append_to(mut self, parent: &Node, children: &Vec<Node>) -> Self {
        children.iter().for_each(|x| {
            self.is_child_of.insert(x.id, parent.id);
            self.node_info.insert(x.id, *x);
            ()
        });
        self.node_info.insert(parent.id, *parent);
        self
    }

    pub fn update(mut self, node: &Node) -> Self {
        self.node_info.insert(node.id, *node);
        self
    }

    pub fn show(&self) {
        self.is_child_of
            .iter()
            .for_each(|(key, value)| println!("({} {})", key, value));

        self.node_info
            .iter()
            .for_each(|(key, value)| println!("({} {} width: {})", key, value, value.width));
    }

    pub fn render(&self) {
        self.render_direction(LayoutDirection::Ltr)
    }

    pub fn render_direction(&self, direction: LayoutDirection) {}
}

pub enum LayoutDirection {
    Ltr,
    Rtl,
    Inherit,
}

pub struct NodeLayout {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    width: f64,
    height: f64,
}

impl fmt::Display for NodeLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Layout(left: {}, top: {}, width: {}, height: {})",
            self.left, self.top, self.width, self.height
        )
    }
}
