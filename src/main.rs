use zen::layout::RenderTree;
use zen::node::Node;

fn main() {
    let node1 = Node::new().width(100.0).height(100.0);
    let node2 = Node::new().width(300.0).height(400.0);
    let root = Node::new();

    let tree = RenderTree::new().append_to(&root, &vec![node1, node2]);
    tree.show();

    let node1 = node1.width(2.0);

    tree.update(&node1).show();

    println!("root: {}", root);
}
