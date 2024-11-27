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
