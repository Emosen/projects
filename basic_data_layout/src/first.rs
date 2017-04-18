// pub enum List{
//     Empty,
//     Elem(i32, List),
// }
// #[derive(Debug)]
// enum List<T>{
//     Cons(T, Box<List<T>>),
//     Nil,
// }
use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>)
}

pub struct List {
    head: Link,
}

impl List {
    // pub fn push(&mut self, elem: i32){
    //     let new_node = Node {
    //         elem: elem,
    //         next: self.head,
    //     };
    // }
    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        // match self.head {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                //TODO
                result = None;
            }
            Link::More(node) => {
                //TODO
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }
}

fn main(){
    // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2,
    // Box::new(List::Nil))));
    // println!("{:?}!", list);            

}

// pub trait Drop {
//     fn drop(&mut self);
// }

// impl Drop for List {
//     fn drop(&mut self){
//         // NOTE: you can't actually explicityly call `drop` in real Rust code;
//         // we're pretending to be the compiler!
//         list.head.drop(); //tail recursive - good!
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self){
//         match list.head {
//             Link::Empty => {} // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop();  // tail recursive - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

impl Drop for List {
    fn drop(&mut self){
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let ` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

mod test {
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();

        //Check empty list behaves right
        assert_eq!(list.pop(), None);

        //Populate List
        list.push(1);
        list.push(2);
        list.push(3);

        //Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

    }
}