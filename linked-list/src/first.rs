use std::mem;

#[derive(Debug)]
pub struct List {
  head: Link
}

impl List {
  pub fn new () -> Self {
    List { head: Link::Empty }
  }

  pub fn push (&mut self, value: i32) {
    let current = Box::new(Node {
      value,
      next: mem::replace(&mut self.head, Link::Empty)
    });
    self.head = Link::More(current)
  }

  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.value)
      }
    }
  }
}


impl Drop for List {
  fn drop(&mut self) {
      let mut cur_link = mem::replace(&mut self.head, Link::Empty);

      while let Link::More(mut boxed_node) = cur_link {
          cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
      }
  }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

#[derive(Debug)]
enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug)]
struct Node {
  value: i32,
  next: Link
}