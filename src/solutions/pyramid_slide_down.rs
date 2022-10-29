//https://www.codewars.com/kata/551f23362ff852e2ab000037/train/rust

use std::sync::{Arc, Mutex};

//first iterative solution
fn _calculate_new_down_val(up: &Vec<u16>, down: &Vec<u16>) -> Vec<u16> {
    let mut return_vec: Vec<u16> = Vec::new();
    for (index, value) in up.iter().enumerate() {
        return_vec.push(down[index..index + 2].iter().max().unwrap() + value);
    }
    return_vec
}

//value calculation with iterators
fn calculate_new_down_val(up: &Vec<u16>, down: &Vec<u16>) -> Vec<u16> {
    up.iter()
        .enumerate()
        .map(|(index, val)| val + down[index..index + 2].iter().max().unwrap())
        .collect::<Vec<u16>>()
}

fn _longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let len = pyramid.len();
    if len < 2 {
        return pyramid[0][0];
    }
    let mut down = calculate_new_down_val(&pyramid[len - 2], &pyramid[len - 1]);
    for up in pyramid[0..len - 2].iter().rev() {
        down = calculate_new_down_val(&up, &down);
    }
    down[0]
}

//cool recursive solution with graph structure
#[derive(Debug)]
struct Node {
    val: u16,
    left: Option<Arc<Mutex<Node>>>,
    right: Option<Arc<Mutex<Node>>>,
}
impl Node {
    fn new(val: u16) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    fn calculate(&self, node: Option<Arc<Mutex<Node>>>) -> u16 {
        let val: u16 = match (self.left.clone(), self.right.clone()) {
            (Some(left), Some(right)) => {
                let left_val = self.calculate(Some(left));
                let right_val = self.calculate(Some(right));
                if left_val > right_val {
                    left_val + self.val
                } else {
                    right_val + self.val
                }
            }
            (Some(left), None) => {
                let self_left = left.clone();
                self.calculate(Some(self_left)) + self.val
            }
            (None, Some(right)) => {
                let self_right = right.clone();
                self.calculate(Some(self_right)) + self.val
            }
            (None, None) => {
                println!("i'm fucking here");
                self.val
            },
        };
        val
    }
}

fn set_child(root: Arc<Mutex<Node>>, left_node: Arc<Mutex<Node>>, right_node: Arc<Mutex<Node>>) {
    let mut root_copy_lock = root.lock().unwrap();
    root_copy_lock.left = Some(left_node);
    root_copy_lock.right = Some(right_node);
}
fn increment_counter(node_counter: Arc<Mutex<u16>>){
    let mut counter_lock = node_counter.lock().unwrap();
    *counter_lock += 1;
}

fn create_tree(pyramid: &[Vec<u16>]) -> Arc<Mutex<Node>> {

    let node_counter: Arc<Mutex<u16>>= Arc::new(Mutex::new(1));

    let root = Arc::new(Mutex::new(Node::new(pyramid[0][0])));
    let mut old_node_vec: Vec<Arc<Mutex<Node>>> = vec![root.clone()];
    for step in pyramid[1..].iter() {
        let mut new_node_vec: Vec<Arc<Mutex<Node>>> = Vec::new();
        
        for (index, node) in old_node_vec.iter().enumerate() {
            if new_node_vec.len() == 0 {
                new_node_vec.push(Arc::new(Mutex::new(Node::new(step[index]))));
                increment_counter(node_counter.clone());
            }
            new_node_vec.push(Arc::new(Mutex::new(Node::new(step[index + 1]))));
            increment_counter(node_counter.clone());
            set_child(
                node.clone(),
                new_node_vec[index].clone(),
                new_node_vec[index + 1].clone(),
            );
        }
        old_node_vec = new_node_vec;
    }
    dbg!(node_counter);
    dbg!(&root);
    root
    
}

pub fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    if pyramid.len() < 2 {
        return pyramid[0][0];
    }
    let tree_root = create_tree(pyramid);
    //let tree_root = Arc::new(Mutex::new(Node::new(pyramid[0][0])));
    let tree_lock = tree_root.lock().unwrap();
    //tree_lock.calculate(None)
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(
            longest_slide_down(&small),
            23,
            "It should work for small pyramids"
        );
    }
    /*
    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(
            longest_slide_down(&medium),
            1074,
            "It should work for medium pyramids"
        );
    }
    */
}
