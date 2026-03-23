use std::fmt::Display;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

type SharedNodePtr<T> = Rc<RefCell<Node<T>>>;
type WeakNodePtr<T> = Weak<RefCell<Node<T>>>;

struct Node<T: PartialEq + Display + Clone>
{
    value: T,
    next_node: Option<SharedNodePtr<T>>,
    prev_node: Option<WeakNodePtr<T>>,
}

pub struct DoubleLinkedList<T: PartialEq + Display + Clone> {
    size: usize,
    start: Option<SharedNodePtr<T>>,
    end: Option<WeakNodePtr<T>>,
}

impl<T: PartialEq + Display + Clone> DoubleLinkedList<T> {

    pub fn create() -> Self {
            DoubleLinkedList {
                size: 0,
                start: None,
                end: None
            }
        }

    pub fn add_value(&mut self, val: &T) {
        
        let node_ptr: SharedNodePtr<T> = Rc::new(RefCell::new(Node {
            value: val.clone(),
            next_node: None,
            prev_node: None,
        }));

        if self.size == 0 {
            // Use Rc::clone to assign node_ptr into start (shared ownership)
            self.start = Some(Rc::clone(&node_ptr));
            // Rc::downgrade creates a Weak pointer from node_ptr for end
            self.end = Some(Rc::downgrade(&node_ptr));
        } else {
            // To get a pointer to the end node, call as_ref() on end to get 
            // Option<&Weak<...>>. Unwrap() to get &Weak, then upgrade() to get 
            // Option<Rc<...>>. unwrap() again to get Rc<...>. 
            // This is safe here because end always points to a valid node.
            let last_rc = self.end.as_ref().unwrap().upgrade().unwrap();
            // Set the new node's prev_node to last_rc (as Weak)
            node_ptr.borrow_mut().prev_node = Some(Rc::downgrade(&last_rc));
            // Set last node's next_node to the new node (as Rc)
            last_rc.borrow_mut().next_node = Some(Rc::clone(&node_ptr));
            // Update end to the new node
            self.end = Some(Rc::downgrade(&node_ptr));
        }
        self.size += 1;
    }

    // pub fn remove_value(&mut self, val: &T) {
    //     if self.size == 0 {
    //         println!("List is empty.");
    //         return;
    //     }
        
    //     // check if val is first element
    //     if let Some(node) = &mut self.start 
    //         && node.value == *val {
    //         self.start = node.next_node.take();
    //         self.size -= 1;
    //         return;
    //     }
    
    //     let mut current = &mut self.start;
    //     while let Some(node) = current {
    //         if let Some(next) = node.next_node.as_ref() {
    //             if next.value == *val {
    //                 let removed = node.next_node.take().unwrap();
    //                 node.next_node = removed.next_node;
    //                 self.size -= 1;
    //                 return;
    //             } 
    //         } 
    //         current = &mut node.next_node;
    //     }

    //     println!("Value {} was not stored.", val);
    // }

    pub fn print_all(&self) {
        if self.size == 0 {
            println!("List is empty.");
            return;
        }
        
        // To print all elements, traverse the list starting from self.start.
        // self.start is Option<Rc<RefCell<Node>>>. To get an owned Rc (not just a reference),
        // use as_ref() to get Option<&Rc<...>>, then map(Rc::clone) to turn it into Option<Rc<...>>.
        // This lets us own each Rc as we traverse, and Option handles the end-of-list case.
        let mut current = self.start.as_ref().map(Rc::clone);
        println!("List contains {} elements. Printing forward.", self.size);

        while let Some(node) = current {
            println!("Value {}", node.borrow().value);
            current = node.borrow().next_node.as_ref().map(Rc::clone);  // Move to the next node   
        }
    }

    pub fn print_all_backwards(&self) {
        if self.size == 0 {
            println!("List is empty.");
            return;
        }
        
        let mut current = self.end.as_ref().unwrap().upgrade().as_ref().map(Rc::clone);
        println!("List contains {} elements. Printing backwards.", self.size);

        while let Some(node) = current {
            println!("Value {}", node.borrow().value);
            current = node.borrow().prev_node.as_ref().and_then(|w| w.upgrade());  // Move to the next node   
        }
    }


    pub fn size(&self) -> usize {
        self.size
    }

}