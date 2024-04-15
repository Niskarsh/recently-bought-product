use core::borrow;
use std::{borrow::Borrow, ops::Deref};

use recently_bought_product::{DoublyLinkedList, Node};
fn main() {
    let mut list = DoublyLinkedList::init();
    list.add(&2);
    list.add(&23);
    list.add(&456);
    for &value in list.map.values() {
        let borrowed_node: std::cell::Ref<'_, std::rc::Rc<std::cell::RefCell<Node>>> = value.borrow();
    let node_ref: &std::rc::Rc<std::cell::RefCell<Node>> = borrowed_node.deref();
    let node: std::cell::Ref<'_, Node> = node_ref.borrow();

        println!("{}", node.element);
    }
    list.displayFromTail();
}
