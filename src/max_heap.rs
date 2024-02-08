pub struct MaxHeap<T> {
    arr: Vec<T>,
    heap_size: usize,
}

impl<T> MaxHeap<T> {
    pub fn new() -> MaxHeap<T> {
        MaxHeap {
            arr: Vec::new(),
            heap_size: 0,
        }
    }
}

impl<T: Ord> MaxHeap<T> {
    #[inline]
    // assumes idx1 < size
    // returns idx with the largest value
    unsafe fn largest(&mut self, idx1: usize, idx2: usize) -> usize {
        debug_assert!(idx1 < self.heap_size);
        if idx2 < self.heap_size
            && self
                .arr
                .get_unchecked(idx1)
                .cmp(self.arr.get_unchecked(idx2))
                .is_lt()
        {
            idx2
        } else {
            idx1
        }
    }

    unsafe fn swap(&mut self, idx1: usize, idx2: usize) {
        // TODO impelment unchecked
        debug_assert!(idx1 < self.heap_size);
        debug_assert!(idx1 < self.heap_size);
        self.arr.swap(idx1, idx2);
    }

    #[inline]
    fn parent_index(&mut self, idx: usize) -> usize {
        let right = if idx & 1 == 1 {
            // odd
            idx + 1
        } else {
            idx
        };
        (right / 2) - 1
    }

    pub fn insert(&mut self, value: T) {
        let mut index = self.heap_size;
        self.heap_size = index + 1;
        self.arr.push(value);
        while index > 0 {
            let parent = self.parent_index(index);
            unsafe {
                if self
                    .arr
                    .get_unchecked(parent)
                    .cmp(self.arr.get_unchecked(index))
                    .is_lt()
                {
                    self.swap(parent, index);
                    index = parent;
                } else {
                    break;
                }
            }
        }
    }

    fn max_heapify(&mut self, mut index: usize) {
        let size = self.heap_size;
        while index < size {
            let right = (index + 1) * 2;
            let largest = unsafe {
                let max_right = self.largest(index, right);
                let left = right - 1;
                self.largest(max_right, left)
            };
            if largest == index {
                break;
            } else {
                unsafe {
                    self.swap(index, largest);
                }
                index = largest;
            }
        }
    }

    pub fn extract_root(&mut self) -> Option<T> {
        let size = self.heap_size;
        if size == 0 {
            return None;
        };
        if size == 1 {
            self.heap_size = 0;
            return self.arr.pop();
        };
        let last = self.heap_size - 1;
        self.heap_size = last;
        unsafe { self.swap(0, last) };
        self.max_heapify(0);
        self.arr.pop()
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(value: Vec<T>) -> Self {
        let heap_size = value.len();
        let mut heap = MaxHeap {
            arr: value,
            heap_size,
        };
        for idx in (0..(heap_size / 2)).rev() {
            heap.max_heapify(idx)
        }
        heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_1() {
        // Setup/Run
        let heap = MaxHeap::from(vec![1, 2, 3]);

        // test output
        assert_eq!(heap.arr[0], 3);
        let mut others = vec![heap.arr[1], heap.arr[2]];
        others.sort();
        assert_eq!(others, vec![1, 2]);
    }

    #[test]
    fn test_create_2() {
        // Setup/Run
        let heap = MaxHeap::from(vec![1, 2, 3, 4, 5, 6i32]);

        // test output
        assert_eq!(heap.arr[0], 6);
        let slice = &heap.arr[1..6];
        let mut others = Vec::from(slice);
        others.sort();
        assert_eq!(others, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insert_1() {
        // Setup
        let mut heap = MaxHeap::from(vec![1, 2, 3, 4, 5, 6i32]);

        // pre condition
        assert_eq!(heap.arr[0], 6);

        // Run
        heap.insert(44);

        // test output
        assert_eq!(heap.arr[0], 44);
    }

    #[test]
    fn test_insert_2() {
        // Setup
        let mut heap = MaxHeap::new();

        // Run/test
        heap.insert(1);
        assert_eq!(heap.arr[0], 1);
        heap.insert(2);
        assert_eq!(heap.arr[0], 2);
        heap.insert(3);
        assert_eq!(heap.arr[0], 3);
        heap.insert(4);
        assert_eq!(heap.arr[0], 4);
        heap.insert(5);
        assert_eq!(heap.arr[0], 5);
        heap.insert(6);
        assert_eq!(heap.arr[0], 6);
        heap.insert(7);
        assert_eq!(heap.arr[0], 7);
    }

    #[test]
    fn test_insert_3() {
        // Setup
        let mut heap = MaxHeap::new();

        // Run/test
        heap.insert(7);
        assert_eq!(heap.arr[0], 7);
        heap.insert(1);
        assert_eq!(heap.arr[0], 7);
        heap.insert(2);
        assert_eq!(heap.arr[0], 7);
        heap.insert(3);
        assert_eq!(heap.arr[0], 7);
        heap.insert(4);
        assert_eq!(heap.arr[0], 7);
        heap.insert(5);
        assert_eq!(heap.arr[0], 7);
        heap.insert(6);
        assert_eq!(heap.arr[0], 7);
    }


    #[test]
    fn test_extract_1() {
        // Setup
        let mut heap = MaxHeap::new();

        // Test
        assert_eq!(heap.extract_root(), None);

        // Additional Setup
        heap.insert(7);
        heap.insert(1);
        heap.insert(2);
        heap.insert(3);
        heap.insert(4);
        heap.insert(5);
        heap.insert(6);

        // Test
        assert_eq!(heap.extract_root(), Some(7));
        assert_eq!(heap.extract_root(), Some(6));
        assert_eq!(heap.extract_root(), Some(5));
        assert_eq!(heap.extract_root(), Some(4));
        assert_eq!(heap.extract_root(), Some(3));
        assert_eq!(heap.extract_root(), Some(2));
        assert_eq!(heap.extract_root(), Some(1));
        assert_eq!(heap.extract_root(), None);
    }
}
