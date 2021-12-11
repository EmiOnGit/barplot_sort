

type Atomic = (usize,usize);
pub enum Swap {
    Atomic(Atomic),
    Swaps(Vec<Atomic>)
}

pub fn sort(to_sort: &mut Vec<f32>) -> impl Iterator<Item = Swap> {
    let mut swaps = Vec::new();
    let mut ordered = false;
    while !ordered {
        ordered = true;
        for i in 0..to_sort.len() - 1 {
            if to_sort[i] > to_sort[i+1] {
                ordered = false;
                swaps.push(Swap::Atomic((i, i+1)));
                to_sort.swap(i, i+1);
                break;
            }
        }
    }
    swaps.into_iter()
}