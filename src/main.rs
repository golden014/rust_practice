// use std::ptr::eq;

// struct Node{
//     value: i32,
//     next: Option<Box<Node>>
// }

// fn push_head(head: &mut Option<Box<Node>>, tail: &mut Option<Box<Node>>, new_value: i32) {

//     let mut new_node: Option<Box<Node>> = Some(Box::new(Node{value: new_value, next: None}));

//     //kalau linked list masih kosong
//     if head.is_none() {
//         *head = new_node;
//         *tail = new_node;
//     } 
//     //kalau linked list hanya ada satu node saja
//     else if eq(head, tail) {
//         new_node.unwrap().next = head;
//         *head = new_node;
//     }
// }

// fn print_linked_list(head: &mut Option<Node>) {
//     let temp = head;
//     while temp != None {
//         print!(temp.value);
//         print!(", ")
//         temp = temp.next
//     }
// }

// fn main() {
//     let mut head: Option<Box<Node>> = None;
//     let mut tail: Option<Box<Node>> = None;

// }

use std::{cell::Ref, fmt::Display};

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}


fn print_linked_list<T: Display>(head: &Option<Box<Node<T>>>) {
    let mut curr = head;
    while let Some(node) = curr {
        println!("{}", node.data.to_string());
        curr = &node.next;
    }
}

// fn push_linked_list<T>(curr_head: &mut &Option<Box<Node<T>>>, curr_tail: &mut &Option<Box<Node<T>>>, new_node: &Option<Box<Node<T>>>) {
//     if curr_head.is_none() {
//         *curr_head = new_node;
//         *curr_tail = new_node;
//     }
// }



fn main() {
    let mut head: &Option<Box<Node<i32>>> = &None;
    // let mut tail: &Option<Box<Node<i32>>> = None;

    // //push head
    // head = Some(Box::new(Node {data: 32, next: None}));
    // head = Some(Box::new(Node {data: 22, next: head}));

    print_linked_list(&head);
}

//creating traditional linked list is fundamentally impossible with rust :D