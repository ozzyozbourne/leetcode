pub use std::{
    cell::Ref, cell::RefCell, cell::RefMut, collections::HashMap, collections::VecDeque, rc::Rc,
};
pub type Node = Rc<RefCell<TreeNode>>;
pub type T = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: T,
    pub right: T,
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

#[allow(dead_code)]
fn get_ref<'a>(node: &'a T) -> Ref<'a, TreeNode> {
    node.as_ref().unwrap().borrow()
}

#[allow(dead_code)]
fn get_mut<'a>(node: &'a T) -> RefMut<'a, TreeNode> {
    node.as_ref().unwrap().borrow_mut()
}

macro_rules! consume_option_borrow {
    ($option:expr) => {
        $option.unwrap().borrow()
    };
}

macro_rules! consume_option_borrow_mut {
    ($option:expr) => {
        $option.unwrap().borrow_mut()
    };
}

macro_rules! borrow_mut {
    ($option:expr) => {
        $option.as_ref().unwrap().borrow_mut()
    };
}

macro_rules! borrow {
    ($option:expr) => {
        $option.as_ref().unwrap().borrow()
    };
}

macro_rules! treeNode {
    () => {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    };
    ($a:expr, $b:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: $a,
            right: $b,
        })))
    };
}

macro_rules! tn {
    ($v:expr) => {
        Some(Rc::new(RefCell::new(TreeNode::new($v))))
    };
    ($v:expr, $l:expr, $r:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: $l,
            right: $r,
        })))
    };
}

macro_rules! bmut {
    ($t:expr) => {
        $t.as_ref().unwrap().borrow_mut()
    };
}
macro_rules! b {
    ($t:expr) => {
        $t.as_ref().unwrap().borrow()
    };
}

#[derive(Debug)]
pub struct SmallerToLarger {
    pub root: T,
    pub min: i32,
}

#[derive(Debug)]
pub struct LargerToSmaller {
    pub root: T,
    pub max: i32,
}

impl SmallerToLarger {
    fn new(root: T) -> Self {
        SmallerToLarger {
            root,
            min: i32::MIN,
        }
    }
}

impl LargerToSmaller {
    fn new(root: T) -> Self {
        LargerToSmaller {
            root,
            max: i32::MAX,
        }
    }
}

impl Iterator for SmallerToLarger {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.root.clone() {
            if node.borrow().left.is_some() && b!(node.borrow().left).val > self.min {
                let mut pre = node.borrow().left.clone();
                while b!(pre).right.is_some() && b!(pre).right != self.root {
                    pre = pre.unwrap().borrow().right.clone();
                }
                if b!(pre).right == self.root {
                    self.min = node.borrow().val;
                    _ = pre.unwrap().borrow_mut().right.take();
                    self.root = node.borrow().right.clone();
                    break;
                } else {
                    pre.unwrap().borrow_mut().right = self.root.clone();
                    self.root = node.borrow().left.clone();
                }
            } else {
                self.min = node.borrow().val;
                self.root = node.borrow().right.clone();
                break;
            }
        }
        Some(self.min)
    }
}

pub fn max_ancestor_diff(root: T) -> i32 {
    fn helper(root: T, mut min: i32, mut max: i32, mut res: i32) -> i32 {
        if root.is_none() {
            return res;
        }
        res = res.max((b!(root).val - min).abs().max((b!(root).val - max).abs()));
        min = min.min(b!(root).val);
        max = max.max(b!(root).val);
        res = helper(b!(root).left.clone(), min, max, res);
        res = helper(b!(root).right.clone(), min, max, res);
        res
    }
    helper(root.clone(), b!(root).val, b!(root).val, 0)
}

impl Iterator for LargerToSmaller {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.root.clone() {
            if node.borrow().right.is_some() && b!(node.borrow().right).val < self.max {
                let mut pre = node.borrow().right.clone();
                while b!(pre).left.is_some() && b!(pre).left != self.root {
                    pre = pre.unwrap().borrow().left.clone()
                }
                if b!(pre).left == self.root {
                    self.max = node.borrow().val;
                    _ = pre.unwrap().borrow_mut().left.take();
                    self.root = node.borrow().left.clone();
                    break;
                } else {
                    pre.unwrap().borrow_mut().left = self.root.clone();
                    self.root = node.borrow().right.clone();
                }
            } else {
                self.max = node.borrow().val;
                self.root = node.borrow().left.clone();
                break;
            }
        }
        Some(self.max)
    }
}

pub fn generate_trees(n: i32) -> Vec<T> {
    let mut dp = HashMap::<(i32, i32), Rc<Vec<T>>>::new();
    fn generate(left: i32, right: i32, dp: &mut HashMap<(i32, i32), Rc<Vec<T>>>) -> Rc<Vec<T>> {
        if left > right {
            return Rc::new(vec![None]);
        }
        let key = (left, right);
        if dp.contains_key(&key) {
            return dp.get(&key).unwrap().clone();
        }
        let mut res = Vec::new();
        for val in left..=right {
            for left_tree in generate(left, val - 1, dp).iter() {
                for right_tree in generate(val + 1, right, dp).iter() {
                    res.push(tn!(val, left_tree.clone(), right_tree.clone()));
                }
            }
        }

        let res = Rc::new(res);
        dp.insert(key, res.clone());
        res
    }
    let res = generate(1, n, &mut dp);
    drop(dp);
    Rc::into_inner(res).unwrap()
}

pub fn get_all_elements(root1: T, root2: T) -> Vec<i32> {
    struct MorrisIter {
        nxtright: T,
        root: T,
    }
    impl MorrisIter {
        fn new(root: T) -> Self {
            let mut res = MorrisIter {
                nxtright: root.clone(),
                root,
            };
            res.next();
            res
        }

        fn has_next(&self) -> bool {
            self.root.is_some()
        }

        fn peek(&self) -> i32 {
            b!(self.root).val
        }

        fn poll(&mut self) -> i32 {
            let res = b!(self.root).val;
            Self::next(self);
            res
        }

        fn next(&mut self) {
            self.root = self.nxtright.clone();
            while let Some(node) = self.root.clone() {
                if node.borrow().left.is_some() {
                    let mut pre = node.borrow().left.clone();
                    while b!(pre).right.is_some() && b!(pre).right != self.root {
                        pre = pre.unwrap().borrow().right.clone();
                    }
                    if b!(pre).right == self.root {
                        _ = pre.unwrap().borrow_mut().right.take();
                        self.nxtright = b!(self.root).right.clone();
                        break;
                    } else {
                        bmut!(pre).right = self.root.clone();
                        self.root = node.borrow().left.clone();
                    }
                } else {
                    self.nxtright = b!(self.root).right.clone();
                    break;
                }
            }
        }
    }
    let (mut res, mut iter1, mut iter2) =
        (Vec::new(), MorrisIter::new(root1), MorrisIter::new(root2));
    while iter1.has_next() && iter2.has_next() {
        if iter1.peek() < iter2.peek() {
            res.push(iter1.poll());
        } else {
            res.push(iter2.poll());
        }
    }
    while iter1.has_next() {
        res.push(iter1.poll());
    }
    while iter2.has_next() {
        res.push(iter2.poll());
    }
    res
}

pub fn find_target(root: T, k: i32) -> bool {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let (mut l_iter, mut r_iter) = (
        SmallerToLarger::new(root.clone()),
        LargerToSmaller::new(root),
    );
    let (mut l, mut r) = (l_iter.next().unwrap(), r_iter.next().unwrap());
    while l < r {
        match (l + r).cmp(&k) {
            Less => l = l_iter.next().unwrap(),
            Greater => r = r_iter.next().unwrap(),
            Equal => return true,
        }
    }
    false
}

pub fn build_tree(mut preorder: Vec<i32>, inorder: Vec<i32>) -> T {
    let (root, mut i) = (tn!(preorder.remove(0)), 0);
    let mut curr = root.clone();
    for p in preorder {
        if b!(curr).val == inorder[i] {
            i += 1; // the parent of curr
            while b!(curr).right.is_some() && b!(b!(curr).right).val == inorder[i] {
                (curr, i) = (curr.unwrap().borrow_mut().right.take(), i + 1);
            }
            bmut!(curr).right = tn!(p, None, curr.clone());
            curr = curr.unwrap().borrow().right.clone();
        } else {
            bmut!(curr).left = tn!(p, None, curr.clone());
            curr = curr.unwrap().borrow().left.clone();
        }
    }
    while b!(curr).right.is_some() {
        curr = curr.unwrap().borrow_mut().right.take();
    }
    root
}

pub fn reverse_odd_levels(root: T) -> T {
    fn helper(r1: T, r2: T, d: i32) {
        if r1.is_none() && r2.is_none() {
            return;
        }
        if (d & 1) == 1 {
            bmut!(r1).val = b!(r2).val;
            bmut!(r2).val = b!(r1).val;
        }
        helper(b!(r1).left.clone(), b!(r2).right.clone(), d + 1);
        helper(b!(r1).right.clone(), b!(r2).left.clone(), d + 1);
    }
    helper(b!(root).left.clone(), b!(root).right.clone(), 1);
    root
}

pub fn create_binary_tree(des: Vec<Vec<i32>>) -> T {
    let (mut nodes, mut has_parent) = (HashMap::<i32, T>::new(), HashMap::<i32, bool>::new());
    for d in des {
        let (parent, child, is_left) = (d[0], d[1], (d[2] & 1) == 1);
        let parent_node = if nodes.contains_key(&parent) {
            nodes.get(&parent).unwrap().clone()
        } else {
            tn!(parent)
        };
        let child_node = if nodes.contains_key(&child) {
            nodes.get(&child).unwrap().clone()
        } else {
            tn!(child)
        };
        if is_left {
            bmut!(parent_node).left = child_node.clone();
        } else {
            bmut!(parent_node).right = child_node.clone();
        }
        has_parent.insert(child, true);
        nodes.insert(child, child_node);
        nodes.insert(parent, parent_node);
    }

    nodes
        .into_iter()
        .filter(|(k, _)| !has_parent.contains_key(k))
        .map(|(_, v)| v)
        .next()
        .unwrap()
}

pub fn bst_from_preorder(mut pre: Vec<i32>) -> T {
    let root = tn!(pre.remove(0));
    let mut curr = root.clone();
    for n in pre {
        if n < b!(curr).val {
            (bmut!(curr).left = tn!(n, curr.clone(), None));
        } else {
            while b!(curr).right.is_some() && n > b!(b!(curr).right).val {
                curr = curr.unwrap().borrow_mut().right.take();
            }
            bmut!(curr).right = tn!(n, None, b!(curr).right.clone());
        }
    }
    while b!(curr).right.is_some() {
        curr = curr.unwrap().borrow_mut().right.take();
    }
    root
}

pub fn all_possible_fbt(n: i32) -> Vec<T> {
    fn bk(n: i32, dp: &mut HashMap<i32, Rc<Vec<T>>>) -> Rc<Vec<T>> {
        match n {
            0 => dp.get(&0).unwrap().clone(),
            1 => dp.get(&1).unwrap().clone(),
            _ if dp.contains_key(&n) => dp.get(&n).unwrap().clone(),
            _ => {
                let mut res = Vec::new();
                for l in (1..n).step_by(2) {
                    let r = n - 1 - l;
                    for t1 in bk(l, dp).iter() {
                        for t2 in bk(r, dp).iter() {
                            res.push(treeNode!(t1.clone(), t2.clone()));
                        }
                    }
                }
                let res = Rc::new(res);
                dp.insert(n, res.clone());
                res
            }
        }
    }
    let result = bk(
        n,
        &mut HashMap::from([
            (0, Rc::new(Vec::<T>::new())),
            (1, Rc::new(vec![treeNode!()])),
        ]),
    );
    Rc::try_unwrap(result).unwrap()
}

pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> T {
    let mut attch = VecDeque::<T>::new();
    for num in nums {
        let node = Rc::new(RefCell::new(TreeNode::new(num)));
        while !attch.is_empty() && attch.back().unwrap().as_ref().unwrap().borrow().val < num {
            node.borrow_mut().left = attch.pop_back().unwrap();
        }
        if !attch.is_empty() {
            attch.back().unwrap().as_ref().unwrap().borrow_mut().right = Some(Rc::clone(&node));
        }
        attch.push_back(Some(node));
    }
    attch.pop_front().unwrap()
}

pub fn balance_bst(root: T) -> T {
    fn create_right_skewed_vine_tree(root: T) -> T {
        let vine_head = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        borrow_mut!(vine_head).right = root;
        let mut current = vine_head.clone();
        while borrow!(current).right.is_some() {
            if borrow!(borrow!(current).right).left.is_some() {
                let right = borrow!(current).right.clone();
                rotate_right(current.clone(), right);
            } else {
                current = consume_option_borrow!(current).right.clone();
            }
        }
        vine_head
    }
    fn get_right_skewed_vine_tree_node_count(mut root: T) -> i32 {
        let mut count = 0;
        while root.is_some() {
            count += 1;
            root = consume_option_borrow!(root).right.clone();
        }
        count
    }
    fn rotate_right(parent: T, node: T) {
        let temp = borrow!(node).left.clone();
        borrow_mut!(node).left = borrow_mut!(temp).right.take();
        borrow_mut!(temp).right = node;
        consume_option_borrow_mut!(parent).right = temp;
    }
    fn rotate_left(parent: T, node: T) {
        let temp = borrow!(node).right.clone();
        borrow_mut!(node).right = borrow_mut!(temp).left.take();
        borrow_mut!(temp).left = node;
        consume_option_borrow_mut!(parent).right = temp;
    }
    fn rotation_left_by_amount(mut root: T, count: i32) {
        for _ in 0..count {
            let right = borrow!(root).right.clone();
            rotate_left(root.clone(), right);
            root = consume_option_borrow!(root).right.clone();
        }
    }
    //create vine
    let vine_head = create_right_skewed_vine_tree(root);
    // get total node count
    let nodecount = get_right_skewed_vine_tree_node_count(borrow_mut!(vine_head).right.clone());
    // get node count of perfect tree
    let mut perfect_tree_node_count = 2_i32.pow(((nodecount + 1) as f32).log2().floor() as u32) - 1;
    // shift the extra nodes to the left
    rotation_left_by_amount(vine_head.clone(), nodecount - perfect_tree_node_count);
    while perfect_tree_node_count > 1 {
        perfect_tree_node_count /= 2;
        rotation_left_by_amount(vine_head.clone(), perfect_tree_node_count);
    }
    consume_option_borrow_mut!(vine_head).right.take()
}

pub fn merge_trees(r1: T, r2: T) -> T {
    match (r1, r2) {
        (None, None) => None,
        (None, Some(n)) | (Some(n), None) => Some(n),
        (Some(n1), Some(n2)) => {
            {
                let (mut n1, n2) = (n1.borrow_mut(), n2.borrow());
                n1.val += n2.val;
                n1.left = merge_trees(n1.left.clone(), n2.left.clone());
                n1.right = merge_trees(n1.right.clone(), n2.right.clone());
            }
            Some(n1)
        }
    }
}

pub fn increasing_bst(mut root: T) -> T {
    let dummy = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let mut tail = dummy.clone();

    while let Some(curr) = root.clone() {
        if curr.borrow().left.is_some() {
            let mut pre = curr.borrow().left.clone();
            while let Some(right) = pre.clone().unwrap().borrow().right.clone() {
                pre = Some(right);
            }
            pre.unwrap().borrow_mut().right = root.clone();
            root = curr.borrow_mut().left.take();
        } else {
            tail.as_ref().unwrap().borrow_mut().right = root.clone();
            tail = Some(Rc::clone(&curr));
            root = curr.borrow().right.clone();
        }
    }
    dummy.unwrap().borrow_mut().right.take()
}

pub fn bst_to_gst(root: T) -> T {
    let (mut node, mut total) = (root.clone(), 0);
    while let Some(n) = node.clone() {
        if n.borrow().right.is_none() {
            total += n.borrow().val;
            n.borrow_mut().val = total;
            node = n.borrow().left.clone();
        } else {
            let mut pre = n.borrow().right.clone();
            while pre.as_ref().unwrap().borrow().left.is_some()
                && pre.as_ref().unwrap().borrow().left != node
            {
                pre = pre.unwrap().borrow().left.clone();
            }
            if pre.as_ref().unwrap().borrow().left == node {
                total += n.borrow().val;
                _ = pre.unwrap().borrow_mut().left.take();
                n.borrow_mut().val = total;
                node = n.borrow().left.clone();
            } else {
                pre.unwrap().borrow_mut().left = node.clone();
                node = n.borrow().right.clone();
            }
        }
    }
    root
}

pub fn binary_tree_paths(root: T) -> Vec<String> {
    fn dfs(root: T, path: &mut String, res: &mut Vec<String>) {
        if let None = root {
            return;
        }
        if borrow!(root).left.is_none() && borrow!(root).right.is_none() {
            path.push_str(&root.unwrap().borrow().val.to_string());
            res.push(path.clone());
            return;
        }
        path.push_str(&root.as_ref().unwrap().borrow().val.to_string());
        path.push_str("->");
        let len = path.len();
        dfs(root.as_ref().unwrap().borrow().left.clone(), path, res);
        path.truncate(len);
        dfs(root.unwrap().borrow().right.clone(), path, res);
    }
    let (mut res, mut path) = (Vec::new(), String::new());
    dfs(root, &mut path, &mut res);
    res
}

pub fn find_tilt(root: T) -> i32 {
    let mut t = 0;
    fn dfs(root: T, t: &mut i32) -> i32 {
        if let None = root {
            return 0;
        }
        let l = dfs(root.as_ref().unwrap().borrow().left.clone(), t);
        let r = dfs(root.as_ref().unwrap().borrow().right.clone(), t);
        *t += l.abs_diff(r) as i32;
        root.unwrap().borrow().val + l + r
    }
    _ = dfs(root, &mut t);
    t
}
pub fn inorder_traversal_1(root: T) -> Vec<i32> {
    let mut res = Vec::new();
    fn inorder(root: T, res: &mut Vec<i32>) {
        if let Some(node) = root {
            inorder(node.borrow().left.clone(), res);
            res.push(node.borrow().val);
            inorder(node.borrow().right.clone(), res);
        }
    }
    inorder(root, &mut res);
    res
}

pub fn increasing_bst_morris(mut root: T) -> T {
    while let Some(node) = root.clone() {
        if let None = node.borrow().left.clone() {
            root = node.borrow().right.clone();
        } else {
            let mut pre = node.borrow().left.clone();
            while pre.as_ref().unwrap().borrow().right.is_some() {
                pre = pre.unwrap().borrow().right.clone();
            }
            pre.as_ref().unwrap().borrow_mut().right = root.clone();
            root = root.unwrap().borrow_mut().left.take();
        }
    }
    root
}

pub fn invert_tree(root: T) -> T {
    match root {
        None => None,
        Some(node) => {
            let left = node.borrow_mut().left.take();
            node.borrow_mut().left = node.borrow_mut().right.take();
            node.borrow_mut().right = left;
            invert_tree(node.borrow().left.clone());
            invert_tree(node.borrow().right.clone());
            Some(node)
        }
    }
}

pub fn average_of_levels(root: T) -> Vec<f64> {
    let (mut res, mut queue) = (Vec::new(), std::collections::VecDeque::new());
    queue.push_back(root);
    while !queue.is_empty() {
        let (mut level_sum, size) = (0.0, queue.len());
        for _ in 0..size {
            let node = queue.pop_front().unwrap();
            level_sum += node.as_ref().unwrap().borrow().val as f64;
            queue.push_back(node.as_ref().unwrap().borrow().left.clone());
            queue.push_back(node.as_ref().unwrap().borrow().right.clone());
        }
        res.push(level_sum / size as f64);
    }
    res
}

pub fn is_unival_tree(root: T) -> bool {
    fn helper(mut root: T, cons: i32) -> bool {
        while let Some(node) = root.clone() {
            if node.borrow().left.is_none() {
                if node.borrow().val != cons {
                    return false;
                }
                root = node.borrow().right.clone();
            } else {
                let mut pre = node.borrow().left.clone();
                while pre.as_ref().unwrap().borrow().right.is_some()
                    && pre.as_ref().unwrap().borrow().right != root
                {
                    pre = pre.unwrap().borrow().right.clone();
                }
                if pre.as_ref().unwrap().borrow().right == root {
                    pre.as_ref().unwrap().borrow_mut().right = None;
                    root = node.borrow().right.clone();
                } else {
                    if node.borrow().val != cons {
                        return false;
                    }
                    pre.as_ref().unwrap().borrow_mut().right = root.clone();
                    root = node.borrow().left.clone();
                }
            }
        }
        true
    }
    helper(root.clone(), root.unwrap().borrow().val)
}

pub fn leaf_similar(r1: T, r2: T) -> bool {
    fn helper(r: T, mut res: Vec<i32>) -> Vec<i32> {
        match r {
            None => res,
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    res.push(node.borrow().val);
                }
                res = helper(node.borrow().left.clone(), res);
                res = helper(node.borrow().right.clone(), res);
                res
            }
        }
    }
    helper(r1, Vec::new()) == helper(r2, Vec::new())
}

pub fn average_of_subtree(root: T) -> i32 {
    let mut res = 0;
    fn helper(root: T, res: &mut i32) -> (i32, i32) {
        if let None = root.clone() {
            return (0, 0);
        }
        let (lsum, lcnt) = helper(root.as_ref().unwrap().borrow().left.clone(), res);
        let (rsum, rcnt) = helper(root.as_ref().unwrap().borrow().right.clone(), res);

        let curr_sum = root.as_ref().unwrap().borrow().val + lsum + rsum;
        let curr_cnt = 1 + lcnt + rcnt;
        let avg = curr_sum / curr_cnt;

        if avg == root.as_ref().unwrap().borrow().val {
            *res += 1;
        }
        (curr_sum, curr_cnt)
    }
    _ = helper(root, &mut res);
    res
}

pub fn inorder_traversal_morris(mut root: T) -> Vec<i32> {
    let mut res = Vec::new();
    while let Some(node) = root.clone() {
        if let None = node.borrow().left.clone() {
            res.push(node.borrow().val);
            root = node.borrow().right.clone();
        } else {
            let mut pre = node.borrow().left.clone();
            while pre.as_ref().unwrap().borrow().right.is_some()
                && pre.as_ref().unwrap().borrow().right != root
            {
                pre = pre.unwrap().borrow().right.clone();
            }
            if pre.as_ref().unwrap().borrow().right.is_some() {
                res.push(node.borrow().val);
                pre.as_ref().unwrap().borrow_mut().right.take();
                root = node.borrow().right.clone();
            } else {
                pre.as_ref().unwrap().borrow_mut().right = root.clone();
                root = node.borrow().left.clone();
            }
        }
    }
    res
}

pub fn inorder_traversal(mut root: T) -> Vec<i32> {
    let mut res = Vec::new();
    while let Some(node) = root.clone() {
        if node.borrow().left.is_some() {
            let mut pre = node.borrow().left.clone();
            while pre.as_ref().unwrap().borrow().right.is_some() {
                pre = pre.unwrap().borrow().right.clone();
            }
            pre.as_ref().unwrap().borrow_mut().right = root.clone();
            let left = node.borrow_mut().left.take();
            root = left;
        } else {
            res.push(node.borrow().val);
            root = node.borrow().right.clone();
        }
    }
    res
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
