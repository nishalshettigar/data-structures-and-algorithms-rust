

struct LinkedList {
    head: Box<Node>,
    size: usize,
}

enum Node {
    Element(i32, Box<Node>),
    End,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: Box::new(Node::End),
            size: 0,
        }
    }

    fn push(&mut self, val: i32) {
        self.head = Box::new(Node::Element(val, Box::new(Node::End)));
        self.size += 1;
    }

    fn size(&self) -> usize {
        self.size
    }

    /// Get returns a reference to the element. If you want to get the element with ownership, call the `remove` method, as the element should no longer be part of the list, if you want ownership of it.
    fn get(&self, idx: usize) -> Option<&i32> {
        match self.head.as_ref() {
            Node::Element(a, _) => Some(a),
            Node::End => None,
        }
    }
}



fn main() {

}


#[cfg(test)]
mod tests {
    use std::collections::linked_list;

    use super::*;

    #[test]
    fn new_creates_an_empty_linked_list() {
        let mut ll = LinkedList::new();
        
        assert_eq!(0, ll.size());
    }

    #[test]
    fn whenSingleElementIsAdded_ItReturnsSizeAs1() {
        let mut ll = LinkedList::new();
        
        ll.push(8);

        assert_eq!(1, ll.size());
    }

    #[test]
    fn whenSingleElementIsAdded_thenGetMethodReturnsThatValue() {
        let mut ll = LinkedList::new();
        
        ll.push(8);

        assert_eq!(Some(&8), ll.get(0));
    }
}

