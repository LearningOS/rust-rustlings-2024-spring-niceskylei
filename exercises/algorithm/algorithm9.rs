use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.sift_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);

        if left_child_idx > self.count {
            0
        } else if right_child_idx > self.count {
            left_child_idx
        } else {
            let left_child = &self.items[left_child_idx];
            let right_child = &self.items[right_child_idx];
            if (self.comparator)(left_child, right_child) {
                left_child_idx
            } else {
                right_child_idx
            }
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            let parent_idx = self.parent_idx(idx);
            self.items.swap(idx, parent_idx);
            idx = self.parent_idx(idx);
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove_root())
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    fn remove_root(&mut self) -> T {
        if self.count == 0 {
            panic!("Cannot remove root from an empty heap");
        }
        self.items.swap(1, self.count);
        let root = self.items.pop().unwrap();
        self.count -= 1;
        if self.count > 0 {
            self.sift_down(1);
        }
        root
    }

    fn sift_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let min_child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[min_child_idx], &self.items[idx]) {
                self.items.swap(min_child_idx, idx);
                idx = min_child_idx;
            } else {
                break;
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
