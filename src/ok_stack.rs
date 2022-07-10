pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        // replace current list head with the new node
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn mut_peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // fields of a tuple struct can be accessed numerically
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use crate::ok_stack::IntoIter;
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek().is_none());

        list.push(1);
        assert_eq!(list.peek(), Some(1).as_ref());

        list.push(2);
        assert_eq!(list.peek(), Some(2).as_ref());

        list.pop();
        assert_eq!(list.peek(), Some(1).as_ref());
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert!(list.mut_peek().is_none());

        list.push(1);
        assert_eq!(list.mut_peek(), Some(1).as_mut());

        list.push(2);
        assert_eq!(list.mut_peek(), Some(2).as_mut());
        list.mut_peek().map(|val| {
            *val += 5
        });

        assert_eq!(list.mut_peek(), Some(&mut 7));

        list.pop();
        assert_eq!(list.mut_peek(), Some(1).as_mut());
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = IntoIter(list);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}