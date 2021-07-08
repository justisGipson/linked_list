#[derive(Clone)]
enum Link<T> {
    None,
    Tail { item: T },
    Link { item: T, next: Box<Link<T>>} // recursive type - stores an instance of itself
}

#[derive(Clone)]
struct Cursor<T> {
    curr: Link<T>
}

impl<T> Link<T> where T: Copy {
    pub fn new() -> Self{
        Self::None
    }

    pub fn push(&mut self, x: T) {
        match self {
            Self::None => self.to_tail(x),
            Self::Tail { .. } => self.to_link(x),
            Self::Link { next, .. } => next.push(x)
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { item } => {
                let item = *item;
                self.to_none();
                Some(item)
            },
            Self::Link { item, next } => {
                let mut n = Box::new(Self::None);
                let item = *item;
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                Some(item)
            }
        }
    }

    fn to_tail(&mut self, it: T) {
        *self = match self {
            Self::None => Self::Tail { item: it },
            Self::Link { item: _, next: _ } => Self::Tail { item: it },
            _ => panic!("Couldn't convert to Tail")
        }
    }

    fn to_link(&mut self, x: T) {
        *self = match self {
            Self::Tail { item } => {
                Self::Link {
                    item: *item,
                    next: Box::new(Self::Tail { item: x })
                }
            },
            _ => { panic!("Couldn't convert to Link")}
        };
    }

    fn to_none(&mut self) {
        *self = std::mem::replace(self, Link::None);
    }

    fn to_next(&mut self, nxt: Link<T>) {
        *self = nxt;
    }
}

impl<T> IntoIterator for Link<T> where T: Copy {
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor {
            curr: self
        }
    }
}
/// not possible to simply pop the self.curr value. We have to walk through it again, matching on each type and using
/// our swap-the-box technique from before. Since self is referring to an instance of Cursor in this context, our Link
/// variants in our match arms are not mutably borrowed, they’re just values

impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let nxt = match self.curr {
            Link::None => None,
            Link::Tail { item } => {
                self.curr = Link::None;
                Some(item)
            },
            Link::Link { item, ref mut next } => {
                let mut n = Box::new(Link::None);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                Some(item)
            }
        };
        nxt
    }
}

fn main() {
    let mut list: Link<i32> = Link::new();
    let mut list2: Link<i32> = Link::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    list2.push(10);
    list2.push(20);
    list2.push(30);

    // println!("{}", list.pop().unwrap());
    // println!("{}", list.pop().unwrap());
    // println!("{}", list.pop().unwrap());

    println!("{}", list2.pop().unwrap());
    println!("{}", list2.pop().unwrap());
    println!("{}", list2.pop().unwrap());
    println!("---");

    for i in list.clone().into_iter().map(|x| x * 2) {
        println!("{}", i);
    }

    for (i, x) in list.into_iter().enumerate() {
        println!("iter2: {}, {}", i, x);
    }
}
