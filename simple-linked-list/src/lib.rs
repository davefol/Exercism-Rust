use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {head: None}
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;

        // Option<&Node<T>>
        //  Some<&Node<T>>
        //  None
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            size += 1;
            // current_node : &Node<T>
            // node.next : &Option<Box<Node<T>>>
            // node.next.as_ref() : Option<&Box<Node<T>>>
            current_node = node.next.as_ref().map(|x| &**x);
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        let mut node = Node {data: _element, next: None};
        match self.head {
            Some(_) => {
                let current_head = self.head.take().unwrap();
                node.next = Some(Box::new(current_head));
                self.head = Some(node);
            },
            None => self.head = Some(node)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped_node = self.head.take(); // Option<Node<T>>
        match popped_node {
            Some(node) => { // node: Node<T>
               self.head = node.next.map(|x| *x);
               Some(node.data)
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

}

impl <T: std::clone::Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        // start from the top and push a copy of next
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            list.push(node.data.clone());
            current_node = node.next.as_ref().map(|x| &**x);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        for val in _iter {
            list.push(val)
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: std::clone::Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let list = self.rev();
        let mut v: Vec<T> = Vec::<T>::new();
        let mut current_node = list.head.as_ref();
        while let Some(node) = current_node {
            v.push(node.data.clone());
            current_node = node.next.as_ref().map(|x| &**x);
        }
        v
    }
}
