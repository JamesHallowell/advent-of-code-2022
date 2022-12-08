type Tree = usize;
type Height = u32;

struct Map {
    width: usize,
    height: usize,
    tree_heights: Vec<Height>,
}

impl Map {
    fn new(input: &str) -> Self {
        Self {
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
            tree_heights: input
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.width * self.height
    }

    fn trees(&self) -> impl Iterator<Item = Tree> + '_ {
        0..self.len()
    }

    fn row(&self, tree: Tree) -> usize {
        tree / self.width
    }

    fn col(&self, tree: Tree) -> usize {
        tree % self.width
    }

    fn is_on_edge(&self, tree: Tree) -> bool {
        self.col(tree) == 0
            || self.col(tree) == self.width - 1
            || self.row(tree) == 0
            || self.row(tree) == self.height - 1
    }

    fn left(&self, tree: usize) -> impl Iterator<Item = Tree> + '_ {
        (0..tree)
            .rev()
            .filter(move |&other| self.row(other) == self.row(tree))
    }

    fn right(&self, tree: Tree) -> impl Iterator<Item = Tree> + '_ {
        (tree + 1..self.len()).filter(move |&other| self.row(other) == self.row(tree))
    }

    fn up(&self, tree: Tree) -> impl Iterator<Item = Tree> + '_ {
        (0..tree)
            .rev()
            .filter(move |&other| self.col(other) == self.col(tree))
    }

    fn down(&self, tree: Tree) -> impl Iterator<Item = Tree> + '_ {
        (tree + 1..self.len()).filter(move |&other| self.col(other) == self.col(tree))
    }

    fn is_visible_from_outside(&self, tree: Tree) -> bool {
        if self.is_on_edge(tree) {
            return true;
        }

        let does_not_block = |other| self.tree_heights[other] < self.tree_heights[tree];

        self.left(tree).all(does_not_block)
            || self.right(tree).all(does_not_block)
            || self.up(tree).all(does_not_block)
            || self.down(tree).all(does_not_block)
    }

    fn scenic_score(&self, tree: Tree) -> usize {
        if self.is_on_edge(tree) {
            return 0;
        }

        let not_blocked = |tree_in_view: &Tree| -> bool {
            self.tree_heights[*tree_in_view] < self.tree_heights[tree]
        };

        self.left(tree).take_while_inclusive(not_blocked).count()
            * self.right(tree).take_while_inclusive(not_blocked).count()
            * self.up(tree).take_while_inclusive(not_blocked).count()
            * self.down(tree).take_while_inclusive(not_blocked).count()
    }
}

fn main() {
    let map = Map::new(include_str!("input.txt"));

    let result = map
        .trees()
        .filter(|&tree| map.is_visible_from_outside(tree))
        .count();
    println!("Part 1: {result}");

    let result = map
        .trees()
        .map(|tree| map.scenic_score(tree))
        .max()
        .unwrap();
    println!("Part 2: {result}");
}

struct TakeWhileInclusive<I, P> {
    iter: Option<I>,
    predicate: P,
}

impl<I, P> Iterator for TakeWhileInclusive<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.as_mut().and_then(|iter| iter.next()) {
            if !(self.predicate)(&item) {
                self.iter = None;
            }
            return Some(item);
        }
        None
    }
}

trait IteratorExt: Iterator {
    fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusive<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeWhileInclusive {
            iter: Some(self),
            predicate,
        }
    }
}

impl<I> IteratorExt for I where I: Iterator {}
