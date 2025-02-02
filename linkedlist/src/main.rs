use std::fmt::Display;

enum LinkedList<T> {
    Node(T, Box<LinkedList<T>>),
    Empty,
}

impl<T: Display> LinkedList<T> {
    fn new() -> Self {
        Self::Empty
    }

    fn with(val: T) -> Self {
        Self::Node(val, Box::new(Self::Empty))
    }

    fn traverse(&self) {
        let mut current = self;
        while let Self::Node(this, tail) = current {
            println!("{this}");
            current = tail;
        }
    }

    fn addvalue(&mut self, val: T) {
        let mut current = self;

        while let Self::Node(this, tail) = current {
            current = tail;
        }

        *current = Self::with(val);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.addvalue(3);
    list.addvalue(2);
    list.addvalue(1);
    list.traverse();
}
