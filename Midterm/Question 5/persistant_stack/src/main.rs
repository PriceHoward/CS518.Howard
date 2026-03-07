use std::collections::VecDeque;
struct Arena<T> {
    nodes: Vec<Node<T>>,
    stack_history: VecDeque<Stack>,
    capacity: usize,
}

struct Node<T> {
    value: T,
    next: Option<usize>,
}

#[derive(Clone, Copy)]
struct Stack {
    top_of_stack: Option<usize>,
    size_of_stack: usize,
}

impl<T: Clone> Arena<T> {
    fn new(capacity: usize) -> Self {
        let mut stack_history = VecDeque::with_capacity(capacity); //Creates an empty deque with space for at least capacity elements. https://doc.rust-lang.org/std/collections/struct.VecDeque.html
        stack_history.push_back(Stack { top_of_stack: None, size_of_stack: 0 });
        Arena { nodes: Vec::new(), stack_history, capacity }
    }

    fn alloc(&mut self, value: T, next: Option<usize>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node { value, next });
        index
    }

    fn save_to_stack_history(&mut self, stack: Stack) {
        if self.stack_history.len() == self.capacity {
            self.stack_history.pop_front();
        }
        self.stack_history.push_back(stack);
    }

    fn push(&mut self, stack: Stack, value: T) -> Stack {
        let index = self.alloc(value, stack.top_of_stack);
        let new_stack = Stack { top_of_stack: Some(index), size_of_stack: stack.size_of_stack + 1 };
        self.save_to_stack_history(new_stack);
        new_stack
    }

    fn pop(&mut self, stack: Stack) -> (T, Stack) {
        let top_of_stack_index = stack.top_of_stack.expect("pop on empty stack");
        let node = &self.nodes[top_of_stack_index];
        let value = node.value.clone();
        let new_stack = Stack { top_of_stack: node.next, size_of_stack: stack.size_of_stack - 1 };
        self.save_to_stack_history(new_stack);
        (value, new_stack)
    }

    fn peek(&self, stack: Stack) -> Option<&T> {
        stack.top_of_stack.map(|index| &self.nodes[index].value)
    }

    fn version(&mut self) -> Stack {
        if let Some(old) = self.stack_history.pop_front() {
            self.stack_history.push_back(old);
        }
        *self.stack_history.front().unwrap()
    }

    fn current(&self) -> Stack {
        *self.stack_history.back().unwrap()
    }
}

fn main() {
    let mut arena: Arena<i32> = Arena::new(8);

    let stack_object0 = arena.current();
    let stack_object1 = arena.push(stack_object0, 10);
    let stack_object2 = arena.push(stack_object1, 20);
    let stack_object3 = arena.push(stack_object2, 30);

    println!("peek stack_object3 = {:?}", arena.peek(stack_object3));
    println!("peek stack_object2 = {:?}", arena.peek(stack_object2));
    println!("peek stack_object1 = {:?}", arena.peek(stack_object1));

    let (value, stack_object4) = arena.pop(stack_object3);
    println!("popped  = {}", value); // 30
    println!("peek stack_object4 = {:?}", arena.peek(stack_object4));

    println!("peek stack_object3 after pop = {:?}", arena.peek(stack_object3));

    println!("--- Stack history ---");
    for _ in 0..arena.stack_history.len() {
        let version = arena.version();
        println!("Stack History top={:?} size={}", version.top_of_stack, version.size_of_stack);
    }
}
