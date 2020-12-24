use std::iter::{FromIterator, IntoIterator, Iterator};
use vec_arena::Arena;

/// The null index, akin to null pointers.
///
/// Just like a null pointer indicates an address no object is ever stored at,
/// the null index indicates an index no object is ever stored at.
///
/// Number `!0` is the largest possible value representable by `usize`.
pub const NULL: usize = !0;

#[derive(Debug, Clone)]
pub struct Node<T: Clone + Eq> {
    /// Previous node in the list.
    pub prev: usize,

    /// Next node in the list.
    pub next: usize,

    /// Actual value stored in node.
    pub value: T,
}

impl<T: Clone + Eq> Node<T> {}

#[derive(Clone)]
pub struct List<T: Clone + Eq> {
    /// This is where nodes are stored.
    pub arena: Arena<Node<T>>,

    /// First node in the list.
    pub head: usize,
}

impl<T: Clone + Eq> List<T> {
    /// Constructs a new, empty doubly linked list.
    pub fn new() -> Self {
        List {
            arena: Arena::new(),
            head: NULL,
        }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Links nodes `a` and `b` together, so that `a` comes before `b` in the list.
    fn link(&mut self, a: usize, b: usize) {
        if a != NULL {
            self.arena[a].next = b;
        }
        if b != NULL {
            self.arena[b].prev = a;
        }
    }

    /// Appends `value` to the back of the list.
    fn push_back(&mut self, value: T) -> usize {
        let node = Node {
            prev: NULL,
            next: NULL,
            value,
        };

        let node_addr = self.arena.insert(node);
        //first item in the list, so we have to link it back to itself
        if self.head == NULL {
            self.head = node_addr;
            self.arena[node_addr].next = node_addr;
            self.arena[node_addr].prev = node_addr;
        } else {
            //there was stuff in the list, so we need to link the previous end to the new element,
            //and the new element to the head
            let tail = self.arena[self.head].prev;
            self.link(tail, node_addr);
            self.link(node_addr, self.head);
        }
        node_addr
    }

    pub fn head_node(&self) -> &Node<T> {
        &self.arena[self.head]
    }

    pub fn head_node_mut(&mut self) -> &mut Node<T> {
        &mut self.arena[self.head]
    }

    pub fn get_node(&self, idx: usize) -> &Node<T> {
        &self.arena[idx]
    }
    pub fn get_node_mut(&mut self, idx: usize) -> &mut Node<T> {
        &mut self.arena[idx]
    }
    pub fn remove_next_n(&mut self, index: usize, n: usize) -> (usize, usize) {
        let current_node = self.get_node(index);
        let result_begin = current_node.next;
        let mut next = current_node.next;
        let mut new_next_node = self.get_node(next);
        let mut c = 0;
        // do next n times, to get the new next node
        while c < n {
            next = new_next_node.next;
            new_next_node = self.get_node_mut(next);
            c += 1;
        }
        // stitch together current_node and new_next_node
        let result_end = new_next_node.prev;

        self.link(index, next);
        // return the index of the ends of the spliced out part

        (result_begin, result_end)
    }
    pub fn add_fragment(&mut self, index: usize, fragment_start: usize, fragment_end: usize) {
        let current_node = self.get_node_mut(index);
        let next = current_node.next;
        current_node.next = fragment_start;

        let old_next_node = self.get_node_mut(next);
        old_next_node.prev = fragment_end;

        let fragment_start_node = self.get_node_mut(fragment_start);
        fragment_start_node.prev = index;

        let fragment_end_node = self.get_node_mut(fragment_end);
        fragment_end_node.next = next;
    }

    pub fn fragment_contains(&self, fragment_start: usize, fragment_end: usize, value: &T) -> bool {
        let mut current_index = fragment_start;
        while current_index != fragment_end {
            let current_node = self.get_node(current_index);
            if current_node.value == *value {
                return true;
            }
            current_index = current_node.next;
        }
        let current_node = self.get_node(current_index);
        if current_node.value == *value {
            return true;
        }

        false
    }
}

impl<T: Clone + Eq> FromIterator<T> for List<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut list = List::new();
        for i in iter {
            list.push_back(i);
        }
        list
    }
}

impl<T: Clone + Eq> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let current = self.head;
        IntoIter {
            inner: self,
            current,
        }
    }
}

pub struct IntoIter<T: Clone + Eq> {
    inner: List<T>,
    current: usize,
}

impl<T: Clone + Eq> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let node = self.inner.get_node(self.current);
        self.current = node.next;
        Some(node.value.clone())
    }
}

#[test]
fn circular_dll() {
    let mut list: List<usize> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    let mut head = list.head_node();
    assert_eq!(1, head.value);
    head = list.get_node(head.next);
    assert_eq!(2, head.value);
    head = list.get_node(head.next);
    assert_eq!(3, head.value);
    head = list.get_node(head.next);
    assert_eq!(1, head.value);
}

#[test]
fn from_iterator() {
    let list: List<usize> = (1usize..=3usize).collect();
    let mut head = list.head_node();
    assert_eq!(1, head.value);
    head = list.get_node(head.next);
    assert_eq!(2, head.value);
    head = list.get_node(head.next);
    assert_eq!(3, head.value);
    head = list.get_node(head.next);
    assert_eq!(1, head.value);
}

#[test]
fn into_iterator() {
    let list: List<usize> = (1usize..=3usize).collect();
    let mut iter = list.into_iter();
    assert_eq!(1, iter.next().unwrap());
    assert_eq!(2, iter.next().unwrap());
    assert_eq!(3, iter.next().unwrap());
    assert_eq!(1, iter.next().unwrap());
}

#[test]
fn remove_next_n() {
    let mut list: List<usize> = (1usize..=3usize).collect();
    let head = list.head;
    list.remove_next_n(head, 1);
    let mut iter = list.into_iter();
    assert_eq!(1, iter.next().unwrap());
    assert_eq!(3, iter.next().unwrap());
    assert_eq!(1, iter.next().unwrap());
}

#[test]
fn add_fragment() {
    let mut list: List<usize> = (1usize..=5usize).collect();
    let mut head = list.head;
    let (fragment_start, fragment_end) = list.remove_next_n(head, 2);
    head = list.get_node(head).next;
    list.add_fragment(head, fragment_start, fragment_end);
    let mut iter = list.into_iter();
    assert_eq!(1, iter.next().unwrap());
    assert_eq!(4, iter.next().unwrap());
    assert_eq!(2, iter.next().unwrap());
    assert_eq!(3, iter.next().unwrap());
    assert_eq!(5, iter.next().unwrap());
    assert_eq!(1, iter.next().unwrap());
}

#[test]
fn fragment_contains() {
    let mut list: List<usize> = (1usize..=5usize).collect();
    let (fragment_start, fragment_end) = list.remove_next_n(list.head, 3);
    assert!(list.fragment_contains(fragment_start, fragment_end, &2usize));
    assert!(list.fragment_contains(fragment_start, fragment_end, &3usize));
    assert!(list.fragment_contains(fragment_start, fragment_end, &4usize));
    assert!(!list.fragment_contains(fragment_start, fragment_end, &1usize));
    assert!(!list.fragment_contains(fragment_start, fragment_end, &5usize));
}
