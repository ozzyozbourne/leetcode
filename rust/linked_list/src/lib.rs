use std::{cell::RefCell, collections::VecDeque, rc::Rc};
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

    use crate::{vec_to_tree, Node, Rc};

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

    use crate::{vec_to_tree, Node};

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

    use crate::{vec_to_tree, Node};

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
    let org_head_ptr = curr.as_mut().unwrap().as_mut() as *mut ListNode;

    for _ in 0..k {
        let next = curr.as_mut().unwrap().next.take();
        curr.as_mut().unwrap().next = prev;
        (prev, curr) = (curr, next);
    }

    unsafe {
        (*org_head_ptr).next = reverse_k_group(curr, k);
    }
    prev
}
