// via https://janstepien.com/that-looks-oddly-familiar/#035
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::fmt::Debug;

pub type Distance = u32;
pub type Hash = u64;

pub trait Metric: Debug {
    fn distance(a: Hash, b: Hash) -> Distance;
}

#[derive(Debug)]
pub struct Simple;
impl Metric for Simple {
    fn distance(a: Hash, b: Hash) -> Distance {
        (a ^ b).count_ones()
    }
}

#[derive(Debug)]
pub enum BKTree<M=Simple> {
    Empty,
    NonEmpty(Node<M>),
}

#[derive(Debug)]
pub struct Node<M=Simple> {
    hash: Hash,
    children: BTreeMap<Distance, Node<M>>,
    metric: PhantomData<M>,
}

impl<M: Metric> BKTree<M> {
    pub fn new() -> Self {
        BKTree::Empty
    }

    pub fn insert(&mut self, new: Hash) {
        match *self {
            BKTree::Empty => *self = BKTree::NonEmpty(Node::single(new)),
            BKTree::NonEmpty(ref mut node) => node.insert(new),
        }
    }

    pub fn find(&self, needle: Hash, tol: Distance) -> Vec<Hash> {
        match self {
            &BKTree::Empty => Vec::new(),
            &BKTree::NonEmpty(ref node) => node.find(needle, tol),
        }
    }
}

impl<M: Metric> Node<M> {
    fn single(hash: Hash) -> Self {
        Node {
            hash,
            children: BTreeMap::new(),
            metric: PhantomData,
        }
    }

    fn insert(&mut self, new: Hash) {
        if self.hash != new {
            let new_dist = M::distance(new, self.hash);
            self.children
                .entry(new_dist)
                .or_insert(Node::single(new))
                .insert(new)
        }
    }

    fn find(&self, needle: Hash, tol: Distance) -> Vec<Hash> {
        let mut result = Vec::new();
        let needle_dist = M::distance(self.hash, needle);
        if needle_dist <= tol {
            result.push(self.hash);
        }
        for (&dist, ref subtree) in &self.children {
            if dist + tol >= needle_dist && needle_dist + tol >= dist {
                for hash in subtree.find(needle, tol) {
                    result.push(hash);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_find_nearby() {
        let mut bk = BKTree::<Simple>::new();
        bk.insert(0x2af7);
        bk.insert(0x6af7);
        let found = bk.find(0x6bf6, 2);
        assert_eq!(found, &[0x6af7][..]);
    }

    #[test]
    fn cant_find_far() {
        let mut bk = BKTree::<Simple>::new();
        bk.insert(0x2af7);
        bk.insert(0x6af7);
        let found = bk.find(0x1bf6, 2);
        assert_eq!(found, &[]);
    }
}
