use std::marker::PhantomData;
use std::ptr;

mod linked_list;

struct Node<T> {
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
    value: T,
}

pub struct LinkedList<T> {
    front_p: Option<*mut Node<T>>,
    back_p: Option<*mut Node<T>>,
    length: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    point: Option<*mut Node<T>>,
}

pub struct Iter<'a, T> {
    current_node: Option<*mut Node<T>>,
    phantom: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front_p: None,
            back_p: None,
            length: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front<'a>(&'a mut self) -> Cursor<'a, T> {
        let ptr = self.front_p;
        Cursor {
            list: self,
            point: ptr,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back<'a>(&'a mut self) -> Cursor<'_, T> {
        let ptr = self.back_p;
        Cursor {
            list: self,
            point: ptr,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current_node: self.front_p,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        if self.length == 0 {
            return;
        }
        unsafe {
            let mut ptr = self.front_p.unwrap();
            while let Some(next_ptr) = (*ptr).next {
                ptr::drop_in_place(ptr);
                ptr = next_ptr;
            }
            ptr::drop_in_place(ptr);
        }
    }
}

// unsafe impl<T> Send for LinkedList<T> where T: Send {}
// unsafe impl<T> Sync for LinkedList<T> where T: Sync {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.point {
            None => None,
            Some(node) => unsafe { Some(&mut (*node).value) },
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        match self.point {
            None => None,
            Some(node) => unsafe {
                match (*node).next {
                    None => None,
                    Some(next_node) => {
                        self.point = Some(next_node);
                        Some(&mut (*next_node).value)
                    }
                }
            },
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        match self.point {
            None => None,
            Some(node) => unsafe {
                match (*node).prev {
                    None => None,
                    Some(prev_node) => {
                        self.point = Some(prev_node);
                        Some(&mut (*prev_node).value)
                    }
                }
            },
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        match self.point {
            None => None,
            Some(node) => unsafe {
                match ((*node).prev, (*node).next) {
                    (None, None) => {
                        self.list.front_p = None;
                        self.list.back_p = None;
                        self.point = None;
                    }
                    (Some(prev_node), None) => {
                        (*prev_node).next = None;
                        self.list.back_p = Some(prev_node);
                        self.point = Some(prev_node);
                    }
                    (None, Some(next_node)) => {
                        (*next_node).prev = None;
                        self.list.front_p = Some(next_node);
                        self.point = Some(next_node);
                    }
                    (Some(prev_node), Some(next_node)) => {
                        (*prev_node).next = Some(next_node);
                        (*next_node).prev = Some(prev_node);
                        self.point = Some(next_node);
                    }
                }
                self.list.length -= 1;
                let ret_val = ptr::read(&(*node).value);
                ptr::drop_in_place(node);
                Some(ret_val)
            },
        }
    }

    pub fn insert_after(&mut self, element: T) {
        match self.point {
            None => {
                let new_node: *mut _ = Box::into_raw(Box::new(Node {
                    prev: None,
                    next: None,
                    value: element,
                }));
                self.list.front_p = Some(new_node);
                self.list.back_p = Some(new_node);
                self.point = Some(new_node);
            }
            Some(node) => unsafe {
                match (*node).next {
                    None => {
                        let new_node: *mut _ = Box::into_raw(Box::new(Node {
                            prev: Some(node),
                            next: None,
                            value: element,
                        }));
                        (*node).next = Some(new_node);
                        self.list.back_p = Some(new_node);
                    }
                    Some(next_node) => {
                        let new_node: *mut _ = Box::into_raw(Box::new(Node {
                            prev: Some(node),
                            next: Some(next_node),
                            value: element,
                        }));
                        (*node).next = Some(new_node);
                        (*next_node).prev = Some(new_node);
                    }
                }
            },
        }
        self.list.length += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        match self.point {
            None => {
                let new_node: *mut _ = Box::into_raw(Box::new(Node {
                    prev: None,
                    next: None,
                    value: element,
                }));
                self.list.front_p = Some(new_node);
                self.list.back_p = Some(new_node);
                self.point = Some(new_node);
            }
            Some(node) => unsafe {
                match (*node).prev {
                    None => {
                        let new_node: *mut _ = Box::into_raw(Box::new(Node {
                            prev: None,
                            next: Some(node),
                            value: element,
                        }));
                        (*node).prev = Some(new_node);
                        self.list.front_p = Some(new_node);
                    }
                    Some(prev_node) => {
                        let new_node: *mut _ = Box::into_raw(Box::new(Node {
                            prev: Some(prev_node),
                            next: Some(node),
                            value: element,
                        }));
                        (*node).prev = Some(new_node);
                        (*prev_node).next = Some(new_node);
                    }
                }
            },
        }
        self.list.length += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.current_node {
            None => None,
            Some(node) => unsafe {
                let value = &(*node).value;
                self.current_node = (*node).next;
                Some(value)
            },
        }
    }
}

#[test]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}

// ———————————————————————————————————————————————————————————
// Tests for Step 1: push / pop at front and back
// ———————————————————————————————————————————————————————————

#[test]
fn basics_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at back ————————————————————————————————————————
#[test]
fn basics_single_element_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_back(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_push_pop_at_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at front ———————————————————————————————————————
#[test]
fn basics_single_element_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_front(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_push_pop_at_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at mixed sides —————————————————————————————————
#[test]
fn basics_push_front_pop_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in 0..10 {
        assert_eq!(list.len(), 10 - i as usize);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn basics_push_back_pop_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in 0..10 {
        assert_eq!(list.len(), 10 - i as usize);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 2: iteration
// ———————————————————————————————————————————————————————————

#[test]
fn iter() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for num in 0..10 {
        list.push_back(num);
    }

    for (num, &entered_num) in (0..10).zip(list.iter()) {
        assert_eq!(num, entered_num);
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 3: full cursor functionality
// ———————————————————————————————————————————————————————————

#[test]
fn cursor_insert_before_on_empty_list() {
    // insert_after on empty list is already tested via push_back()
    let mut list = LinkedList::new();
    list.cursor_front().insert_before(0);
    assert_eq!(Some(0), list.pop_front());
}

#[test]
fn cursor_insert_after_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_front();
        let didnt_run_into_end = cursor.seek_forward(4);
        assert!(didnt_run_into_end);

        for n in (0..10).rev() {
            cursor.insert_after(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

#[test]
fn cursor_insert_before_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_back();
        let didnt_run_into_end = cursor.seek_backward(4);
        assert!(didnt_run_into_end);

        for n in 0..10 {
            cursor.insert_before(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

// "iterates" via next() and checks that it visits the right elements
#[test]
fn cursor_next_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();

    assert_eq!(cursor.peek_mut(), Some(&mut 0));

    for n in 1..10 {
        let next = cursor.next().cloned();
        assert_eq!(next, Some(n));
        assert_eq!(next, cursor.peek_mut().cloned());
    }
}

// "iterates" via prev() and checks that it visits the right elements
#[test]
fn cursor_prev_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_back();

    assert_eq!(cursor.peek_mut(), Some(&mut 9));

    for n in (0..9).rev() {
        let prev = cursor.prev().cloned();
        assert_eq!(prev, Some(n));
        assert_eq!(prev, cursor.peek_mut().cloned());
    }
}

// removes all elements starting from the middle
#[test]
fn cursor_take() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();
    cursor.seek_forward(5);

    for expected in (5..10).chain((0..5).rev()) {
        assert_eq!(cursor.take(), Some(expected));
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 4: clean-up via `Drop`
// ———————————————————————————————————————————————————————————

// The leak tests that are also for this step are separated into
// their own files so that nothing else interferes with the allocator
// whilst they run

// checks number of drops
// may pass for incorrect programs if double frees happen
// exactly as often as destructor leaks
#[test]
fn drop_no_double_frees() {
    use std::cell::Cell;
    struct DropCounter<'a>(&'a Cell<usize>);

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            let num = self.0.get();
            self.0.set(num + 1);
        }
    }

    const N: usize = 15;

    let counter = Cell::new(0);
    let list = std::iter::repeat_with(|| DropCounter(&counter))
        .take(N)
        .collect::<LinkedList<_>>();

    assert_eq!(list.len(), N);
    drop(list);
    assert_eq!(counter.get(), N);
}

#[test]
fn drop_large_list() {
    drop((0..2_000_000).collect::<LinkedList<i32>>());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 5 (advanced): covariance and Send/Sync
// ———————————————————————————————————————————————————————————

// These are compile time tests. They won't compile unless your
// code passes.

#[cfg(feature = "advanced")]
#[test]
fn advanced_linked_list_is_send_sync() {
    trait AssertSend: Send {}
    trait AssertSync: Sync {}

    impl<T: Send> AssertSend for LinkedList<T> {}
    impl<T: Sync> AssertSync for LinkedList<T> {}
}

#[cfg(feature = "advanced")]
#[allow(dead_code)]
#[test]
fn advanced_is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }

    fn a_iter<'a>(i: Iter<'static, &'static str>) -> Iter<'a, &'a str> {
        i
    }
}
