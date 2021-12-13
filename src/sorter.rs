use std::{ops, thread, time::Duration};
use druid::{ExtEventSink, Target};
use crate::{FINISH_SORTING};



type Atomic = (usize,usize);
#[derive(PartialEq, Clone, Debug,Copy)]
pub enum Swap {
    Atomic(Atomic),
}
impl Swap {
    pub fn apply<T>(&self, list: &mut Vec<T>) {
        if let Swap::Atomic((a,b)) = self {
            list.swap(*a, *b);

        }
    }
}
pub struct SortSlice<Item> {
    list: Vec<Item>,
    swaps: Vec<Swap>,
}
impl <Item> From::<Vec<Item>> for SortSlice<Item> {
    fn from(items: Vec<Item>) -> Self {
        SortSlice::new(items)
    }
}
impl <Item> From::<&[Item]> for SortSlice<Item>
where Item: Sized + Clone {
    fn from(items: &[Item]) -> Self {
        Self::new(items.to_vec())
    }
}
impl <Item> From::<&mut [Item]> for SortSlice<Item>
where Item: Sized + Clone {
    fn from(items: &mut [Item]) -> Self {
        Self::new(items.to_vec())
    }
}
impl <Item>SortSlice<Item> {
    pub fn new(list: Vec<Item>) -> Self {
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
            println!("send swaps");

            thread::sleep(Duration::from_secs_f32(0.2));
            
            event_sink.submit_command(
                FINISH_SORTING, 
                swap, 
                Target::Auto)
            .expect("command failed to submit");
        }
    
    }
}
impl <Item> ops::Index<usize> for SortSlice<Item> {
    type Output = Item;

    fn index(&self, index: usize) -> &Self::Output {
        self.list.index(index)
    }
}

pub fn sort(to_sort: &mut Vec<f32>) -> SortSlice<f32> {
    let mut to_sort = SortSlice::new(to_sort.to_vec());
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

pub fn insertion<T: PartialOrd + Copy + Sized + std::fmt::Debug>(xs: &mut [T]) -> SortSlice<T> {
    println!("insertion with data {:?}", xs);
    let mut xs = SortSlice::from(xs);
    let (mut i, len) = (1, xs.len());
    while i < len {
        let mut j = i;
        while j > 0 && xs[j-1] > xs[j] {
            xs.swap(j, j-1);
            j = j - 1;
        }
        i = i + 1;
    }
    println!("insertion");

    xs
}