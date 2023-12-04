#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct Iter<'a> {
    next: Option<&'a Box<ListNode>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_ref();
            n
        })
    }
}

impl<'a> IntoIterator for &'a Box<ListNode> {
    type Item = &'a Box<ListNode>;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { next: Some(self) }
    }
}

impl FromIterator<i32> for Box<ListNode> {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut current = Box::from(ListNode::new(iter.next().unwrap_or_default()));
        let mut tmp = &mut current;
        for i in iter {
            tmp = tmp.next.get_or_insert(Box::from(ListNode::new(i)));
        }
        current
    }
}

fn main() {
    let l1 = Some([2, 3, 1].iter().map(|&v| v).collect::<Box<ListNode>>());
    let l2 = Some([5, 6, 4].iter().map(|&v| v).collect::<Box<ListNode>>());

    let mut iter1 = l1.as_ref().unwrap().into_iter();
    let mut iter2 = l2.as_ref().unwrap().into_iter();
    let mut carry = 0;

    let r: Box<ListNode> = std::iter::from_fn(move || {
        let v1 = iter1.next();
        let v2 = iter2.next();
        if v1.is_some() || v2.is_some() || carry != 0 {
            let r = v1.map_or(0, |v| v.val) + v2.map_or(0, |v| v.val) + carry;
            carry = r / 10;
            Some(r % 10)
        } else {
            None
        }
    })
    .collect();

    r.into_iter().for_each(|i| {
        println!("{}", i.val);
    });
}
