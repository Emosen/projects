use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// enum Link {
//     Empty,
//     More(Box<Node>)
// }

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> Option<T> {
    pub fn as_ref(&self) -> Option<&T>;
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }

    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: mem::replace(&mut self.head, None),
    //     });
    //     self.head = Some(new_node);
    // }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // pub fn pop(&mut self) -> Option<i32> {
    //     // match self.head {
    //     match mem::replace(&mut self.head, None) {
    //         None => None,
    //         Some(node) => {
    //             //TODO
    //             let node = *node;
    //             self.head = node.next;
    //             Some(node.elem)
    //         }
    //     }
    // }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }
}


fn main(){
    // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2,
    // Box::new(List::Nil))));
    // println!("{:?}!", list);            

}

// impl Drop for List {
//     fn drop(&mut self){
//         let mut cur_link = mem::replace(&mut self.head, None);
//         // `while let ` == "do this thing until this pattern doesn't match"
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, None);
//             // boxed_node goes out of scope and gets dropped here;
//             // but its Node's `next` field has been set to None
//             // so no unbounded recursion occurs.
//         }
//     }
// }

impl<T> Drop for List<T> {
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        // `while let ` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
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