use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

    use crate::{vec_to_tree, Rc, RefCell, TreeNode};

    type Node = Rc<RefCell<TreeNode>>;

    fn flatten(root: &mut Option<Node>) {
        match root.is_none() {
            true => {}
            false => {
                let mut node = root.as_mut().unwrap().borrow_mut();

                flatten(&mut node.left);
                flatten(&mut node.right);

                let right = node.right.take();
                node.right = node.left.take();
                drop(node);

                let mut cur = root.clone();
                while let Some(r) = cur {
                    let mut r = r.borrow_mut();
                    if r.right.is_none() {
                        r.right = right;
                        return;
                    }
                    cur = r.right.clone();
                }
            }
        }
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
