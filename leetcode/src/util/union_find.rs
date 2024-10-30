use std::mem;
use std::usize;

pub struct UnionFind {
    weights: Vec<usize>,
    groups: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        let mut groups = Vec::with_capacity(size);
        for i in 0..size {
            groups.push(i);
        }
        UnionFind {
            weights: vec![1; size],
            groups,
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if i >= self.groups.len() {
            return usize::MAX;
        }
        let mut stack = vec![];
        let mut i = i;
        loop {
            let j = unsafe { *self.groups.get_unchecked(i) };
            if i != j {
                stack.push(j);
                i = j;
            } else {
                break;
            }
        }
        for j in stack {
            unsafe {
                *self.groups.get_unchecked_mut(j) = i;
            }
        }
        i
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let (mut i, mut j) = (self.find(i), self.find(j));
        if i == j {
            return false;
        }
        unsafe {
            if *self.weights.get_unchecked(i) < *self.weights.get_unchecked(j) {
                mem::swap(&mut i, &mut j);
            }
            *self.groups.get_unchecked_mut(j) = *self.groups.get_unchecked_mut(i);
            *self.weights.get_unchecked_mut(i) += *self.weights.get_unchecked_mut(j);
        }
        true
    }
}
