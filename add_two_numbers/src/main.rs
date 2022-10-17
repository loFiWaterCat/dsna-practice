fn main() {
    // let l1 = [2, 4, 3];
    // let l2 = [5, 6, 4];
    let mut l1a = ListNode::new(2);
    let mut l1b = ListNode::new(4);
    let l1c = ListNode::new(3);
    l1b.next = Some(Box::new(l1c));
    l1a.next = Some(Box::new(l1b));



}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>)
    -> Option<Box<ListNode>> {
    let mut l1_current = l1;
    let mut l2_current = l2;
    let result_node = Some(Box::new(ListNode::new(0)));
    return result_node;
}

fn print_list(root: Option<Box<ListNode>>) {
    let current_node = root;
    let mut result = Vec::new();

    if let Some(node) = current_node {
        result.push(node.val);
    }
}
