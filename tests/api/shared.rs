use bifurcate_coordinate::BifurcateCoordinate;

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn empty() -> Self {
        Node {
            left: None,
            right: None,
        }
    }

    pub fn left(left: Self) -> Self {
        Node {
            left: Some(Box::new(left)),
            right: None,
        }
    }

    pub fn right(right: Self) -> Self {
        Node {
            left: None,
            right: Some(Box::new(right)),
        }
    }

    pub fn new(left: Self, right: Self) -> Self {
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
