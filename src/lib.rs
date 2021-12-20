pub trait BifurcateCoordinate {
    fn is_empty(&self) -> bool;

    fn has_left_successor(&self) -> bool;
    fn left_successor(&self) -> &Self;

    fn has_right_successor(&self) -> bool;
    fn right_successor(&self) -> &Self;
}

pub fn weight_recursive<C>(c: &C) -> usize
where
    C: BifurcateCoordinate,
{
    if c.is_empty() {
        return 0;
    }

    let l = if c.has_left_successor() {
        weight_recursive(c.left_successor())
    } else {
        0
    };

    let r = if c.has_right_successor() {
        weight_recursive(c.right_successor())
    } else {
        0
    };

    l + r + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Node {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    impl Node {
        fn empty() -> Self {
            Node {
                left: None,
                right: None,
            }
        }

        fn left(left: Self) -> Self {
            Node {
                left: Some(Box::new(left)),
                right: None,
            }
        }

        fn right(right: Self) -> Self {
            Node {
                left: None,
                right: Some(Box::new(right)),
            }
        }

        fn new(left: Self, right: Self) -> Self {
            Node {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            }
        }
    }
    impl BifurcateCoordinate for Node {
        fn is_empty(&self) -> bool {
            self.left.is_none() && self.right.is_none()
        }

        fn has_left_successor(&self) -> bool {
            self.left.is_some()
        }

        fn left_successor(&self) -> &Self {
            &self.left.as_ref().unwrap()
        }

        fn has_right_successor(&self) -> bool {
            self.right.is_some()
        }

        fn right_successor(&self) -> &Self {
            &self.right.as_ref().unwrap()
        }
    }

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
}
