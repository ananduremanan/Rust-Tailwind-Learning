// Define a struct for the linked list node
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    // Create a new node
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

// Define a function to display the linked list
fn display_list<T>(head: &Option<Box<Node<T>>>)
where
    T: std::fmt::Display,
{
    let mut current = head;
    while let Some(node) = current {
        print!("{}->", node.data);
        current = &node.next;
    }
}

fn main() {
    // Create the linked list
    let mut head = Some(Box::new(Node::new(1)));
    let mut current = &mut head;
    
    // Append nodes to the linked list
    for i in 2..6 {
        let node = Node::new(i);
        if let Some(ref mut cur) = current {
            cur.next = Some(Box::new(node));
            current = &mut cur.next;
        }
    }

    // Display the linked list
    display_list(&head);
}
