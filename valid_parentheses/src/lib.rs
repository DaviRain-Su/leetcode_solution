pub mod stack {
    type Link<T> = Option<Box<StackNode<T>>>;

    #[derive(Debug)]
    pub struct Stack<T> {
        top: Link<T>,
    }

    #[derive(Debug, Clone)]
    struct StackNode<T> {
        val: T,
        next: Link<T>,
    }

    // impl <T> StackNode<T> {
    //     fn new(val: T) -> Self {
    //         StackNode {
    //             val,
    //             next: None,
    //         }
    //     }
    // }

    impl <T> Stack<T> {
        pub fn new() -> Self {
            Stack { top: None }
        }

        pub fn push(&mut self, val: T) {
            // let mut node = StackNode::new(val);
            // let next = self.top.take();
            // node.next = next;
            // self.top = Some(Box::new(node));
            let new_node = Box::new(StackNode{
                val,
                next: self.top.take(),
            });

            self.top = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.top
                .take()
                .map(|node| {
                self.top = node.next;
                node.val
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.top
                .as_ref()
                .map(|node|
                    &node.val
                )
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.top
                .as_mut()
                .map(|node| {
                    &mut node.val
                })
        }

        pub fn is_empty(&self) -> bool {
            match self.peek() {
                None => true,
                Some(_) => false,
            }
        }
    }


    impl<T>  Drop for Stack<T> {
        fn drop(&mut self) {
            let mut cut_link = self.top.take();
            while let Some(mut boxed_node) = cut_link{
                cut_link = boxed_node.next.take();
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::Stack;
        #[test]
        fn basic() {
            let mut s = Stack::<i32>::new();

            assert_eq!(s.pop(), None);
            s.push(1);
            s.push(2);
            s.push(3);

            println!("s = {:?}", s);

            assert_eq!(s.pop(), Some(3));
            assert_eq!(s.pop(), Some(2));
            assert_eq!(s.pop(), Some(1));
            assert_eq!(s.pop(), None);
        }

        #[test]
        fn peek() {
            let mut s = Stack::new();

            assert_eq!(s.peek(), None);
            assert_eq!(s.peek_mut(), None);
            s.push(1);
            s.push(2);
            s.push(3);

            assert_eq!(s.peek(), Some(&3));
            assert_eq!(s.peek_mut(), Some(&mut 3));

            s.peek_mut().map(|val| {
                *val = 34;
            });
            assert_eq!(s.peek(), Some(&34));
            assert_eq!(s.pop(), Some(34));
        }
    }
}