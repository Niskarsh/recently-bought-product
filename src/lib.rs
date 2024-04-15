use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct Node {
    element: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(element: &i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: *element,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]

pub struct DoublyLinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    pub length: i32,
    pub map: HashMap<i32, Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    pub fn init() -> Self {
        DoublyLinkedList {
            head: None, tail: None, length: 0,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, element: &i32) {
        let nextNode = Node::new(element);
        if self.head.is_none() {
            self.head = Some(nextNode.clone());
            self.tail = Some(nextNode.clone());
            self.length += 1;
            self.map.insert(*element, nextNode.clone());
            return;
        }

        match self.map.get(element) {
            None => {},
            Some(entry) => {
                // let ownedEntryNode = entry.to_owned();
                let prevNodeToEntry = entry.borrow().prev.as_ref().map(|a| Rc::clone(a)) ;
                let newNext = entry.borrow().next.as_ref().map(|a| Rc::clone(a));
                prevNodeToEntry.as_ref().unwrap().borrow_mut().next = newNext.clone();
                newNext.as_ref().unwrap().borrow_mut().prev = prevNodeToEntry.clone();
                // let ref mut prevNodeToEntry = ownedEntryNode.borrow().prev.clone();
                // let ref mut newNext = ownedEntryNode.borrow().next.clone();
                // prevNodeToEntry.as_ref().unwrap().deref().borrow_mut().next = newNext.clone();
                // newNext.as_ref().unwrap().deref().borrow_mut().prev = prevNodeToEntry.clone();
                self.map.remove(element);
            },
        };
        let oldTail = self.tail.take().unwrap();
        oldTail.deref().borrow_mut().next = Some(nextNode.clone());
        nextNode.deref().borrow_mut().prev = Some(oldTail.clone());
        self.tail = Some(nextNode.clone());
        self.map.insert(*element, nextNode.clone());
        self.length += 1;

    }

    pub fn displayFromTail(&self) {
        let mut displayPointer = Some(self.tail.as_ref().unwrap().clone());
        while !displayPointer.as_ref().unwrap().deref().borrow().prev.is_none() {
            let element = displayPointer.as_ref().unwrap().deref().borrow().element.clone();
            println!("{}", element);
            // drop(element);
            let prevOne = Some(displayPointer.as_ref().unwrap().as_ref().borrow().prev.as_ref().unwrap().clone());
            displayPointer = Some(prevOne.as_ref().unwrap().clone());


        }
        println!("{}", displayPointer.as_ref().unwrap().as_ref().borrow().element.clone());
    }

}