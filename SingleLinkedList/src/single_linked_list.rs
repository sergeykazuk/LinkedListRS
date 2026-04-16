use std::fmt::Display;

struct Node<T: PartialEq + Display + Clone>
{
    value: T,
    next_node: Option<Box<Node<T>>>,
}

pub struct SingleLinkedList<T: PartialEq + Display + Clone> {
    size: usize,
    start: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Display + Clone> SingleLinkedList<T> {

    pub fn new() -> Self {
            SingleLinkedList {
                size: 0,
                start: None,
            }
        }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, val: &T) {
        if self.start.is_none() {
            self.start = Some(Box::new(Node{value: val.clone(), next_node: None}));            
        }
        else {
            let mut head = Box::new(Node{value: val.clone(), next_node: None});
            head.next_node = self.start.take();

            self.start = Some(head);
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.empty() {
            None
        }
        else {
            let old_start = self.start.take().unwrap();
            self.start = old_start.next_node;
            self.size -= 1;
            Some(old_start.value)
        }
    }

    pub fn push_back(&mut self, val: &T) {
        if self.start.is_none() {
            self.start = Some(Box::new(Node{value: val.clone(), next_node: None}));            
        }
        else {
            let mut current = &mut self.start;

            while let Some(node) = current {
                current = &mut node.next_node;   
            }

            *current = Some(Box::new(Node{value: val.clone(), next_node: None}));
        }
        self.size += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.empty() {
            return None;
        }

        if self.start.as_ref().unwrap().next_node.is_none() {
            let old_head = self.start.take().unwrap();
            self.size -= 1;
            return Some(old_head.value);
        }

        let mut current = self.start.as_mut().unwrap();

        while current.next_node.as_ref().unwrap().next_node.is_some() {
            current = current.next_node.as_mut().unwrap();
        }

        let old_tail = current.next_node.take().unwrap();
        self.size -= 1;
        Some(old_tail.value)
    }

    pub fn remove_value(&mut self, val: &T) {
        if self.size == 0 {
            println!("List is empty.");
            return;
        }
        
        // check if val is first element
        if let Some(node) = &mut self.start 
            && node.value == *val {
            self.start = node.next_node.take();
            self.size -= 1;
            return;
        }
    
        let mut current = &mut self.start;
        while let Some(node) = current {
            if let Some(next) = node.next_node.as_ref() {
                if next.value == *val {
                    let removed = node.next_node.take().unwrap();
                    node.next_node = removed.next_node;
                    self.size -= 1;
                    return;
                } 
            } 
            current = &mut node.next_node;
        }

        println!("Value {} was not stored.", val);
    }

    pub fn print_all(&self) {
        if self.size == 0 {
            println!("List is empty.");
            return;
        }
        
        let mut current = &self.start;
        println!("List contains {} elements.", self.size);

        while let Some(node) = current {
            println!("Value {}", node.value);
            current = &node.next_node;  // Move to the next node   
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn at(&self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut current = self.start.as_ref();
        let mut i: usize = 0;

        while i < index {
            current = current?.next_node.as_ref();
            i += 1;
        }
        return Some(current.unwrap().value.clone());
    }

}