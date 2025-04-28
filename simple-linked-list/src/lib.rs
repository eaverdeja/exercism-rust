type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        let mut current = &self.head;

        while let Some(head) = current {
            size += 1;
            current = &head.next;
        }

        size
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Link::Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
            _ => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.element)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut current = self.head;

        while let Some(mut node) = current {
            current = node.next.take();

            node.next = result.head;
            result.head = Some(node);
        }

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for element in iter {
            list.push(element);
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();

        while let Some(head) = linked_list.head {
            vec.push(head.element);
            linked_list.head = head.next;
        }

        vec.reverse();
        vec
    }
}
