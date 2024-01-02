use std::mem;

pub struct List<T>{
    head : Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem : T,
    next : Link<T>
}

impl<T> List<T>{
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, val : T){
        let new_node = Node{
            elem : val,
            next : self.head.take()
        };
        self.head = Link::Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::None);
        while let Link::Some(mut node) = link{
            link = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T>{
    fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}

impl<T> List<T>{
    fn iter(& self) -> Iter<T>{
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;

    fn next<'b>(&'b mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T>{
    fn iter_mut(&mut self) -> IterMut<T>{
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_test() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_test() {
        let  mut list : List<i32> = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));

        list.push(3);
        list.peek_mut().map(|val| *val = 42);
    }

    #[test]
    fn into_iter_test() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_test() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut_test() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

}