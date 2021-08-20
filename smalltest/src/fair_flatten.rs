use std::collections::vec_deque::*;

enum InnerOuter<I, O> {
    Inner(I),
    Outer(O),
}

pub struct FairFlatten<I, O> {
    queue: VecDeque<InnerOuter<I, O>>,
}

impl<I, O> FairFlatten<I, O> {
    pub fn new(i: I) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(InnerOuter::Inner(i));
        FairFlatten { queue }
    }
}

impl<I, O> Iterator for FairFlatten<I, O>
where
    // I: Iterator<Item: IntoIterator<IntoIter = O, Item = O::Item>>,
    I: Iterator<Item = O>,
    O: Iterator,
{
    type Item = O::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let queue_item = self.queue.pop_front();
            match queue_item {
                None => {
                    return None;
                }
                Some(InnerOuter::Inner(mut i)) => {
                    if let Some(o) = i.next() {
                        self.queue.push_back(InnerOuter::Outer(o));
                        self.queue.push_back(InnerOuter::Inner(i));
                    }
                }
                Some(InnerOuter::Outer(mut o)) => {
                    if let Some(item) = o.next() {
                        self.queue.push_back(InnerOuter::Outer(o));
                        return Some(item);
                    }
                }
            };
        }
    }
}
