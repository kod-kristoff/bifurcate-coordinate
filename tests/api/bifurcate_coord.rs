use bifurcate_coordinate::{weight_recursive, BifurcateCoordinate};

use crate::shared::Node;

#[test]
fn empty_node_is_empty() {
    let n = Node::empty();

    assert!(n.is_empty());
}

#[test]
fn weight_recursive_on_empty_node_returns_0() {
    let c = Node::empty();

    assert_eq!(weight_recursive(&c), 0);
}

#[test]
fn non_empty_node_is_not_empty() {
    let n = Node::new(Node::empty(), Node::empty());

    assert!(!n.is_empty());
}

#[test]
fn non_empty_node_has_weight_1() {
    let n = Node::new(Node::empty(), Node::empty());

    assert_eq!(weight_recursive(&n), 1);
}

#[test]
fn node_w_left_child_has_weight_2() {
    let n = Node::new(Node::left(Node::empty()), Node::empty());

    assert_eq!(weight_recursive(&n), 2);
}

#[test]
fn node_w_right_child_has_weight_2() {
    let n = Node::new(Node::empty(), Node::left(Node::empty()));

    assert_eq!(weight_recursive(&n), 2);
}

#[test]
fn node_w_2_children_has_weight_3() {
    let n = Node::new(Node::right(Node::empty()), Node::left(Node::empty()));

    assert_eq!(weight_recursive(&n), 3);
}
