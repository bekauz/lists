use std::mem;

pub struct List {
    head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem: i32,
    next: Link,
}


impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

impl Drop for List {
    fn drop(&mut self) {
        // replace the head with an empty link, returns previous val
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node will go out of scope and get dropped
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // empty list should return nothing upon pop
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        // populated list returns expected results upon pop
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);

        // another check
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}