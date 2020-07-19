use zen::node::Node;
fn main() {
    let node1 = Node::new().width(100.0).height(100.0);
    let node2 = Node::new().width(300.0).height(400.0);

    let root = Node::new().insert_child(node1).insert_child(node2);
    let layout = root.calculate_layout();

    println!("root: {}, {}", root, layout);
}
