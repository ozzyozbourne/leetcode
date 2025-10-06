use std::{cell::RefCell, collections::VecDeque, ptr::null_mut, rc::Rc};
pub type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn spiral_matrix(b: i32, r: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let mut get_val = || match head.take() {
        Some(node) => {
            head = node.next;
            node.val
        }
        None => -1,
    };
    let (mut b, mut r) = (b as usize, r as usize);
    let (mut l, mut t, mut res) = (0, 0, vec![vec![-1; r]; b]);
    while l < r && t < b {
        for i in l..r {
            res[t][i] = get_val();
        }
        t += 1;
        for i in t..b {
            res[i][r - 1] = get_val();
        }
        r -= 1;
        if !(l < r && t < b) {
            break;
        }
        for i in (l..r).rev() {
            res[b - 1][i] = get_val();
        }
        b -= 1;
        for i in (t..b).rev() {
            res[i][l] = get_val();
        }
        l += 1;
    }
    res
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    use std::cell::Ref;
    match root {
        None => 0,
        Some(node) => {
            let node: Ref<TreeNode> = node.borrow();
            match node {
                _ if node.val > high => range_sum_bst(node.left.clone(), low, high),
                _ if node.val < low => range_sum_bst(node.right.clone(), low, high),
                _ => {
                    node.val
                        + range_sum_bst(node.left.clone(), low, high)
                        + range_sum_bst(node.right.clone(), low, high)
                }
            }
        }
    }
}

pub fn vec_to_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    match v.is_empty() {
        true => None,
        false => {
            let (root, mut queue, mut i) = (
                Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))),
                VecDeque::new(),
                1,
            );
            queue.push_back(Rc::clone(&root));
            while i < v.len() {
                match queue.pop_front() {
                    Some(node) => {
                        if let Some(val) = v[i] {
                            let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                            node.borrow_mut().left = Some(Rc::clone(&left_child));
                            queue.push_back(left_child);
                        }
                        i += 1;
                        if i < v.len() {
                            match v[i] {
                                Some(val) => {
                                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                                    node.borrow_mut().right = Some(Rc::clone(&right_child));
                                    queue.push_back(right_child);
                                }
                                None => {}
                            }
                        }
                        i += 1;
                    }
                    None => {}
                }
            }
            Some(root)
        }
    }
}

#[cfg(test)]
mod lc_114_flatten_binary_tree_to_linked_list {

    use crate::{Node, Rc, vec_to_tree};

    fn flatten(root: &mut Option<Node>) {
        fn dfs(node: &mut Option<Node>) -> Option<Node> {
            match node {
                None => None,
                Some(root) => {
                    let left_tail = dfs(&mut root.borrow_mut().left);
                    let right_tail = dfs(&mut root.borrow_mut().right);
                    if let Some(ref left_tail) = left_tail {
                        left_tail.borrow_mut().right = root.borrow_mut().right.take();
                        let right_temp = root.borrow_mut().left.take();
                        root.borrow_mut().right = right_temp;
                    }
                    right_tail.or(left_tail).or(Some(Rc::clone(root)))
                }
            }
        }
        dfs(root);
    }

    #[test]
    fn test_lc_114_one() {
        let mut tree_node = vec_to_tree(vec![
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ]);
        flatten(&mut tree_node);
        assert_eq!(
            tree_node,
            vec_to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6),
            ])
        );
    }

    #[test]
    fn test_lc_114_two() {
        let mut tree_node = vec_to_tree(Vec::new());
        flatten(&mut tree_node);
        assert_eq!(tree_node, vec_to_tree(Vec::new()));
    }

    #[test]
    fn test_lc_114_three() {
        let mut tree_node = vec_to_tree(vec![Some(0)]);
        flatten(&mut tree_node);
        assert_eq!(tree_node, vec_to_tree(vec![Some(0)]));
    }
}

#[cfg(test)]
mod lc_144_binary_tree_preorder_traversal {

    use crate::{Node, vec_to_tree};

    fn preorder_traversal(root: Option<Node>) -> Vec<i32> {
        fn dfs(root: Option<Node>, res: &mut Vec<i32>) {
            match root {
                Some(node) => {
                    res.push(node.borrow().val);
                    dfs(node.borrow_mut().left.take(), res);
                    dfs(node.borrow_mut().right.take(), res);
                }
                None => {}
            }
        }
        let mut res = vec![];
        dfs(root, &mut res);
        res
    }

    #[test]
    fn test_lc_144_one() {
        assert_eq!(
            preorder_traversal(vec_to_tree(vec![Some(1), Some(2), Some(3)])),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_lc_144_two() {
        assert_eq!(preorder_traversal(vec_to_tree(vec![])), vec![]);
    }

    #[test]
    fn test_lc_144_three() {
        assert_eq!(preorder_traversal(vec_to_tree(vec![Some(1)])), vec![1]);
    }
}

#[cfg(test)]
mod lc_145_binary_tree_postorder_traversal {

    use crate::{Node, vec_to_tree};

    fn postorder_traversal(root: Option<Node>) -> Vec<i32> {
        fn dfs(root: Option<Node>, res: &mut Vec<i32>) {
            match root {
                Some(node) => {
                    dfs(node.borrow_mut().left.take(), res);
                    dfs(node.borrow_mut().right.take(), res);
                    res.push(node.borrow_mut().val);
                }
                None => {}
            }
        }
        let mut res = Vec::new();
        dfs(root, &mut res);
        res
    }

    #[test]
    fn test_lc_145_one() {
        assert_eq!(
            postorder_traversal(vec_to_tree(vec![Some(1), Some(2), Some(3)])),
            vec![2, 3, 1]
        );
    }

    #[test]
    fn test_lc_145_two() {
        assert_eq!(postorder_traversal(vec_to_tree(vec![])), vec![]);
    }

    #[test]
    fn test_lc_145_three() {
        assert_eq!(postorder_traversal(vec_to_tree(vec![Some(1)])), vec![1]);
    }
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut curr = &head;
    for _ in 0..k {
        match curr {
            Some(node) => curr = &node.next,
            None => return head,
        }
    }

    let (mut prev, mut curr) = (None, head);
    let unsf_head = curr.as_mut().unwrap().as_mut() as *mut ListNode;

    for _ in 0..k {
        let next = curr.as_mut().unwrap().next.take();
        curr.as_mut().unwrap().next = prev;
        (prev, curr) = (curr, next);
    }

    unsafe {
        (*unsf_head).next = reverse_k_group(curr, k);
    }
    prev
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    fn divide(l: usize, r: usize, lists: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if l > r {
            None
        } else if l == r {
            lists[l].take()
        } else {
            let m = l + (r - l) / 2;
            let (left, right) = (divide(l, m, lists), divide(m + 1, r, lists));
            conquer(left, right)
        }
    }

    fn conquer(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        while left.is_some() && right.is_some() {
            if left.as_ref().unwrap().val <= right.as_ref().unwrap().val {
                current.next = left;
                current = current.next.as_mut().unwrap();
                left = current.next.take();
            } else {
                current.next = right;
                current = current.next.as_mut().unwrap();
                right = current.next.take();
            }
        }

        current.next = left.or(right);
        dummy.next
    }

    if lists.is_empty() {
        None
    } else {
        divide(0, lists.len() - 1, &mut lists)
    }
}

pub fn merge_two_lists(
    mut left: Option<Box<ListNode>>,
    mut right: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let conquer = || {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while left.is_some() && right.is_some() {
            if left.as_ref().unwrap().val <= right.as_ref().unwrap().val {
                curr.next = left;
                curr = curr.next.as_mut().unwrap();
                left = curr.next.take();
            } else {
                curr.next = right;
                curr = curr.next.as_mut().unwrap();
                right = curr.next.take();
            }
        }

        curr.next = left.or(right);

        dummy.next
    };
    let res = conquer();
    res
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = &mut head;
    while let Some(c) = curr.as_ref()
        && let Some(ref n) = curr.as_ref().unwrap().next
    {
        if c.val == n.val {
            curr.as_mut().unwrap().next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            curr = &mut curr.as_mut().unwrap().next;
        }
    }
    head
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { next: head, val: 0 });
    let mut curr = &mut dummy;
    while let Some(ref mut n) = curr.next {
        if n.val == val {
            curr.next = n.next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }
    dummy.next
}
type T = Option<Box<ListNode>>;
pub fn remove_elements1(head: T, val: i32) -> T {
    let mut dummy = Box::new(ListNode { next: head, val: 0 });
    let mut curr = &mut dummy;

    while let Some(ref mut node) = curr.next {
        if node.val != val {
            curr.next = node.next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }
    dummy.next
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(curr) = head.as_mut() {
        let next = curr.next.take();
        curr.next = prev;
        prev = head;
        head = next;
    }
    prev
}

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let raw_head = head.as_mut().unwrap().as_mut() as *mut ListNode;
    unsafe {
        let (mut slow, mut fast, mut prev) = (raw_head, raw_head, null_mut());

        while !fast.is_null()
            && let Some(ref mut f) = (*fast).next
        {
            (prev, slow) = (slow, (*slow).next.as_mut().unwrap().as_mut() as *mut _);
            fast = f
                .next
                .as_mut()
                .map(|n| n.as_mut() as *mut _)
                .unwrap_or(null_mut());
        }

        if prev.is_null() {
            head
        } else {
            (*prev).next.take()
        }
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let (mut slow, mut fast, mut prev) = (&head, &head, &None);

    while let Some(ref f1) = fast.as_ref()
        && let Some(ref f2) = f1.next
    {
        (prev, fast, slow) = (slow, &f2.next, &slow.as_ref().unwrap().next);
    }
    if slow.is_none() {
        return true;
    }
    let (usf_mid, usf_prev): (*mut ListNode, *mut ListNode) = (
        slow.as_ref().unwrap().as_ref() as *const _ as *mut _,
        prev.as_ref().unwrap().as_ref() as *const _ as *mut _,
    );

    let mut sec_head = match fast {
        &Some(_) => unsafe {
            let res = (*usf_mid).next.take();
            drop((*usf_prev).next.take());
            res
        },
        &None => unsafe { (*usf_prev).next.take() },
    };

    let mut prev = None;
    while sec_head.is_some() {
        let next = sec_head.as_mut().unwrap().next.take();
        sec_head.as_mut().unwrap().next = prev;
        prev = sec_head;
        sec_head = next;
    }

    let (mut left, mut right) = (&head, &prev);

    while right.is_some() {
        if left.as_ref().unwrap().val != right.as_ref().unwrap().val {
            return false;
        }
        (left, right) = (&left.as_ref().unwrap().next, &right.as_ref().unwrap().next);
    }

    return true;
}
