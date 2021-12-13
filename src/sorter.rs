use std::{ops, thread, time::Duration};
use druid::ExtEventSink;
use crate::IterData;



type Atomic = (usize,usize);
pub enum Swap {
    Atomic(Atomic),
    _Swaps(Vec<Atomic>)
}
pub struct SortSlice<'a, Item> {
    list: &'a mut Vec<Item>,
    swaps: Vec<Swap>,
}
impl <'a, Item>SortSlice<'a, Item> {
    pub fn new(list: &'a mut Vec<Item>) -> Self {
        SortSlice {
            list,
            swaps: vec![],
        }
    }
    pub fn swap(&mut self,a: usize, b: usize) {
        self.list.swap(a, b);
        self.swaps.push(Swap::Atomic((a,b)))
    }
    pub fn len(&self)->usize {
        self.list.len()
    }
    pub fn send_swaps(self,event_sink: ExtEventSink) {
        for swap in self.swaps {
            thread::sleep(Duration::from_secs_f32(0.2));
    
            event_sink.add_idle_callback(move |data: &mut IterData| {
                data.iteration = Box::new(data.iteration.as_ref() + 1);
                if let Swap::Atomic((i1,i2)) = swap {
                    data.status.borrow_mut().swap(i1, i2);
                }
            });
        }
    }
}
impl <Item> ops::Index<usize> for SortSlice<'_, Item> {
    type Output = Item;

    fn index(&self, index: usize) -> &Self::Output {
        self.list.index(index)
    }
}

pub fn sort(to_sort: &mut Vec<f32>) -> SortSlice<f32> {
    let mut to_sort = SortSlice::new(to_sort);
    let mut ordered = false;
    while !ordered {
        ordered = true;
        for i in 0..to_sort.len() - 1 {
            if to_sort[i] > to_sort[i+1] {
                ordered = false;
                to_sort.swap(i, i+1);
                break;
            }
        }
    }
    to_sort
}