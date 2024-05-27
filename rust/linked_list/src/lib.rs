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

#[cfg(test)]
mod lc_897_increasing_order_search_tree {

    use crate::{vec_to_tree, Node};

    fn increasing_bst(root: Option<Node>) -> Option<Node> {}

    #[test]
    fn test_lc_897_one() {
        assert_eq!(
            increasing_bst(vec_to_tree(vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                Some(8),
                Some(1),
                None,
                None,
                None,
                Some(7),
                Some(9)
            ])),
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
                None,
                Some(7),
                None,
                Some(8),
                None,
                Some(9)
            ])
        );
    }

    #[test]
    fn test_lc_897_two() {
        assert_eq!(
            increasing_bst(vec_to_tree(vec![Some(5), Some(1), Some(7)])),
            vec_to_tree(vec![Some(1), None, Some(5), None, Some(7), None])
        );
    }
}
