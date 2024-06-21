pub type T = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(v: i32) -> Self {
        ListNode { val: v, next: None }
    }
}

pub fn vec_to_listnode(v: Vec<i32>) -> Option<Box<ListNode>> {
    if v.len() == 0 {
        None
    } else {
        let mut head = Some(Box::new(ListNode::new(v[0])));
        let mut current = head.as_mut();

        for &num in &v[1..] {
            let new_node = Some(Box::new(ListNode::new(num)));
            if let Some(node) = current {
                node.next = new_node;
                current = node.next.as_mut();
            }
        }
        head
    }
}

pub fn listnode_to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = Vec::new();
    while let Some(head) = h {
        v.push(head.val);
        h = head.next;
    }
    v
}

#[cfg(test)]
mod lc_206 {

    use crate::{listnode_to_vec, vec_to_listnode, ListNode, T};

    fn reverse_list_iterative(mut head: T) -> T {
        let mut prev = None;
        while let Some(mut node) = head {
            let temp = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = temp;
        }
        prev
    }
    fn reverse_list_recursive(head: T) -> T {
        fn dp(node: T, acc: T) -> T {
            match node {
                None => acc,
                Some(mut curr) => {
                    let nx = curr.next.take();
                    curr.next = acc;
                    dp(nx, Some(curr))
                }
            }
        }
        dp(head, None)
    }

    #[test]
    fn lc_509_tests() {
        struct TestValue {
            input: Option<Box<ListNode>>,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValue {
                input: vec_to_listnode(vec![1, 2, 3, 4, 5]),
                expected: vec![5, 4, 3, 2, 1],
            },
            TestValue {
                input: vec_to_listnode(vec![1, 2]),
                expected: vec![2, 1],
            },
            TestValue {
                input: vec_to_listnode(vec![]),
                expected: vec![],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(
                listnode_to_vec(reverse_list_iterative(t.input.clone())),
                t.expected
            );
            assert_eq!(listnode_to_vec(reverse_list_recursive(t.input)), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_21 {

    use crate::{listnode_to_vec, vec_to_listnode, ListNode, T};

    fn merge_two_lists_iterative(mut l1: T, mut l2: T) -> T {
        let mut prehead = ListNode::new(-1);
        let mut curr = &mut prehead;
        while let (Some(node1), Some(node2)) = (&l1, &l2) {
            if node1.val < node2.val {
                curr.next = l1.take();
                curr = curr.next.as_mut().unwrap();
                l1 = curr.next.take();
            } else {
                curr.next = l2.take();
                curr = curr.next.as_mut().unwrap();
                l2 = curr.next.take();
            }
        }
        curr.next = l1.or(l2);
        prehead.next
    }
    fn merge_two_lists_recursive(l1: T, l2: T) -> T {
        match (l1, l2) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(mut x), Some(mut y)) => {
                if x.val < y.val {
                    x.next = merge_two_lists_recursive(x.next, Some(y));
                    Some(x)
                } else {
                    y.next = merge_two_lists_recursive(Some(x), y.next);
                    Some(y)
                }
            }
        }
    }

    #[test]
    fn lc_21_tests() {
        struct TestValue {
            input1: T,
            input2: T,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValue {
                input1: vec_to_listnode(vec![1, 2, 4]),
                input2: vec_to_listnode(vec![1, 3, 4]),
                expected: vec![1, 1, 2, 3, 4, 4],
            },
            TestValue {
                input1: vec_to_listnode(vec![]),
                input2: vec_to_listnode(vec![]),
                expected: vec![],
            },
            TestValue {
                input1: vec_to_listnode(vec![]),
                input2: vec_to_listnode(vec![0]),
                expected: vec![0],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(
                listnode_to_vec(merge_two_lists_iterative(
                    t.input1.clone(),
                    t.input2.clone()
                )),
                t.expected
            );
            assert_eq!(
                listnode_to_vec(merge_two_lists_recursive(t.input1, t.input2)),
                t.expected
            );
        }
    }
}

#[cfg(test)]
mod lc_326 {
    use std::cmp::Ordering;

    fn is_power_of_three(n: i32) -> bool {
        fn inner(n: i32, r: i32) -> bool {
            if r != 0 {
                false
            } else {
                match n.cmp(&1) {
                    Ordering::Less => false,
                    Ordering::Equal => true,
                    Ordering::Greater => inner(n / 3, n % 3),
                }
            }
        }
        inner(n, 0)
    }

    fn is_power_of_three_pure_functional(n: i32) -> bool {
        fn inner(n: i32, r: i32) -> bool {
            match (n, r) {
                (_, r) if r != 0 => false,
                (n, _) if n < 1 => false,
                (1, 0) => true,
                (n, _) => inner(n / 3, n % 3),
            }
        }
        inner(n, 0)
    }

    #[test]
    fn lc_326_tests() {
        struct TestValue {
            input: i32,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input: 27,
                expected: true,
            },
            TestValue {
                input: 0,
                expected: false,
            },
            TestValue {
                input: -1,
                expected: false,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_power_of_three(t.input), t.expected);
            assert_eq!(is_power_of_three_pure_functional(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_342 {

    fn is_power_of_four(n: i32) -> bool {
        fn inner(n: i32, r: i32) -> bool {
            match (n, r) {
                (_, r) if r != 0 => false,
                (n, _) if n < 1 => false,
                (1, 0) => true,
                _ => inner(n / 4, n % 4),
            }
        }
        inner(n, 0)
    }

    #[test]
    fn lc_342_tests() {
        struct TestValue {
            input: i32,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input: 16,
                expected: true,
            },
            TestValue {
                input: 5,
                expected: false,
            },
            TestValue {
                input: 1,
                expected: true,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_power_of_four(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_231 {

    fn is_power_of_two(n: i32) -> bool {
        fn inner(n: i32, r: i32) -> bool {
            match (n, r) {
                (_, r) if r != 0 => false,
                (n, _) if n < 1 => false,
                (1, 0) => true,
                _ => inner(n / 2, n % 2),
            }
        }
        inner(n, 0)
    }

    #[test]
    fn lc_231_tests() {
        struct TestValue {
            input: i32,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input: 1,
                expected: true,
            },
            TestValue {
                input: 16,
                expected: true,
            },
            TestValue {
                input: 3,
                expected: false,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_power_of_two(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_203 {

    use crate::{listnode_to_vec, vec_to_listnode, ListNode, T};

    fn remove_elements(head: T, val: i32) -> T {
        //avoid option use senital
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut curr = &mut dummy;

        while let Some(n) = curr.next.as_mut() {
            if n.val == val {
                curr.next = n.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }

    #[test]
    fn lc_231_tests() {
        struct TestValue {
            input1: T,
            input2: i32,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValue {
                input1: vec_to_listnode(vec![1, 2, 6, 3, 4, 5, 6]),
                input2: 6,
                expected: vec![1, 2, 3, 4, 5],
            },
            TestValue {
                input1: vec_to_listnode(vec![]),
                input2: 1,
                expected: vec![],
            },
            TestValue {
                input1: vec_to_listnode(vec![7, 7, 7, 7]),
                input2: 7,
                expected: vec![],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(
                listnode_to_vec(remove_elements(t.input1, t.input2)),
                t.expected
            );
        }
    }
}

#[cfg(test)]
mod lc_234 {

    use crate::{vec_to_listnode, T};

    fn is_palindrome(head: T) -> bool {
        let find_mid = || {
            let (mut mid, mut fast) = (0, &head);
            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
                mid += 1;
            }
            mid
        };
        let count = find_mid();
        let split = || {
            let (mut prev, mut cur) = (None, head);
            for _ in 0..count {
                if let Some(mut node) = cur {
                    cur = node.next;
                    node.next = prev;
                    prev = Some(node);
                }
            }
            prev == cur || prev == cur.unwrap().next
        };
        split()
    }

    #[test]
    fn lc_231_tests() {
        struct TestValue {
            input: T,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input: vec_to_listnode(vec![1, 2, 2, 1]),
                expected: true,
            },
            TestValue {
                input: vec_to_listnode(vec![1, 2]),
                expected: false,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_palindrome(t.input), t.expected);
        }
    }
}
