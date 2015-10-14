use std::collections::LinkedList;

fn merge_two_list(list1: &mut LinkedList<i32>, list2: &mut LinkedList<i32>) -> LinkedList<i32> {
    let mut result:LinkedList<i32> = LinkedList::new();

    while !list1.is_empty() && !list2.is_empty() {
        let elem1 = match list1.pop_front() {
            Some(val) => val,
            None => break
        };
        let elem2 = match list2.pop_front() {
            Some(val) => val,
            None => break
        };


        if elem1 < elem2 {
            result.push_back(elem1);
            list2.push_front(elem2);
        } else {
            result.push_back(elem2);
            list1.push_front(elem1);
        }

    }

    if !list1.is_empty() {
        result.append(list1);
    } else if !list2.is_empty() {
        result.append(list2);
    }

    result
}

fn main() {
    let mut list1 = LinkedList::new();
    list1.push_back(1);
    list1.push_back(3);
    list1.push_back(5);
    list1.push_back(7);

    let mut list2 = LinkedList::new();
    list2.push_back(2);
    list2.push_back(4);
    list2.push_back(6);
    list2.push_back(8);

    println!("list1={:?}, list2={:?}", list1, list2);

    let merge_list = merge_two_list(&mut list1, &mut list2);
    println!("merge_list={:?}", merge_list);
}
