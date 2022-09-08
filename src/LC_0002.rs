#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
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

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(-1);
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;
        
        let mut carry: i32 = 0;
        
        while p != None || q != None || carry != 0 {
            let sum = match (&p, &q) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => carry,
            };
            
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            
            p = if p != None { p.unwrap().next } else { p };
            q = if q != None { q.unwrap().next } else { q };
        }
        
        dummy_head.next
    }
}

fn create_list_node(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(-1);
    let mut current = &mut dummy_head;
    for value in values.iter() {
        current.next = Some(Box::new(ListNode::new(*value)));
        current = current.next.as_mut().unwrap();
    }
    dummy_head.next
}

fn print_list_node(l: Option<Box<ListNode>>) {
    let mut cur = l;
    while cur != None {
        let cur_unwrap = cur.unwrap();
        println!("{}", cur_unwrap.val);
        cur = cur_unwrap.next;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::create_list_node;
    use super::print_list_node;
    
    #[test]
    fn LC_0002_1() {
        let l1 = create_list_node(vec![2, 4, 3]);
        let l2 = create_list_node(vec![5, 6, 4]);
        let l = Solution::add_two_numbers(l1, l2);
        print_list_node(l);
    }

    #[test]
    fn LC_0002_2() {
        let l1 = create_list_node(vec![0]);
        let l2 = create_list_node(vec![0]);
        let l = Solution::add_two_numbers(l1, l2);
        print_list_node(l);
    }
}
