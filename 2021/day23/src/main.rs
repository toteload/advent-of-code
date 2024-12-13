use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use once_cell::unsync::Lazy;

#[derive(PartialEq, Eq)]
struct HeapEntry<T: Eq + PartialEq> {
    fscore: u64,
    state: T,
}

impl<T: Eq> Ord for HeapEntry<T> {
    fn cmp(&self, other: &HeapEntry<T>) -> std::cmp::Ordering {
        self.fscore.cmp(&other.fscore).reverse()
    }
}

impl<T: Eq> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &HeapEntry<T>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar_search<'a, T, H, N, M, S>(
    start: T,
    goal: T,
    heuristic: H,
    neighbours: N,
    stepcost: S,
) -> Option<Vec<T>>
where
    T: Clone + PartialEq + Eq + Hash,
    H: Fn(&T) -> u64,
    N: Fn(&T) -> M,
    M: Iterator<Item = T>,
    S: Fn(&T, &T) -> u64,
{
    let mut open: BinaryHeap<HeapEntry<T>> = BinaryHeap::new();
    open.push(HeapEntry {
        fscore: heuristic(&start),
        state: start.clone(),
    });

    let mut parent: HashMap<T, T> = HashMap::new();

    // The g score of a node is the cost of the cheapest path from start to n currently known.
    let mut gscore: HashMap<T, u64> = HashMap::from([(start.clone(), 0)]);

    // The f score is the predicted cost of a path from start to goal through n.
    // fscore[n] = gscore[n] + heuristic(n)
    let mut fscore: HashMap<T, u64> = HashMap::from([(start.clone(), heuristic(&start))]);

    while let Some(HeapEntry { state: current, .. }) = open.pop() {
        if current == goal {
            let mut path = Vec::new();
            let mut last = &goal;
            while last != &start {
                path.push(last.clone());
                last = parent.get(last).unwrap();
            }
            path.push(start.clone());
            path.reverse();
            return Some(path);
        }

        for neighbour in neighbours(&current) {
            let tentative_gscore =
                gscore.get(&current).unwrap_or(&u64::MAX) + stepcost(&current, &neighbour);
            if tentative_gscore < *gscore.get(&neighbour).unwrap_or(&u64::MAX) {
                parent.insert(neighbour.clone(), current.clone());
                gscore.insert(neighbour.clone(), tentative_gscore);
                let neighbour_fscore = tentative_gscore + heuristic(&neighbour);
                fscore.insert(neighbour.clone(), neighbour_fscore);

                let open_contains_neighbour = open.iter().find(|e| e.state == neighbour).is_some();
                if !open_contains_neighbour {
                    open.push(HeapEntry {
                        fscore: neighbour_fscore,
                        state: neighbour.clone(),
                    });
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_1d() {
        let goal = 5isize;
        let path = astar_search(
            0isize,
            goal,
            |x| x.abs_diff(goal) as u64,
            |x| [x - 1, x + 1].into_iter(),
            |a, b| a.abs_diff(*b) as u64,
        );

        assert_eq!(path, Some(vec![0, 1, 2, 3, 4, 5]));
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct AmphipodState {
    pos: [u8; 27],
}

static DISTANCE: Lazy<HashMap<(usize, usize), u64>> = Lazy::new(||{
    let mut m = HashMap::new();
    m.insert((0,0), 0);
    m
});

fn state_neighbours(s: &AmphipodState) -> Vec<AmphipodState> {
    // Either we move an amphipod from the hallway to its destination room,
    // or the first amphipod in a room to the hallway.

    todo!()
    //let mut ns = Vec::new();

    //for i in 0..4 {
    //    // If all the amphipods present in this column are home, then we are
    //    // not allowed to move any of them.
    //    let all_home = s.cols[i].iter().all(|x| *x == (i as u8+1) || *x == 0);
    //    if all_home {
    //        continue;
    //    }

    //    let top = s.cols[i].iter().position(|x| *x != 0);
    //    let Some(top) = top else { continue; };

    //    // The top amphipod in this column has to be moved to some position
    //    // in the hallway.
    //    todo!();
    //}

    //for (i, x) in s.hallway.iter().enumerate().filter(|&(_, x)| *x != 0) {
    //    // See if we can move an amphipod from the hallway into its home column.
    //    let dst = (x - 1) as usize;

    //}

    //ns
}

fn heuristic(s: &AmphipodState) -> u64 {
    todo!()
}

fn stepcost(from: &AmphipodState, to: &AmphipodState) -> u64 {
    let diff = from.pos.iter().enumerate().zip(to.pos.iter().enumerate()).filter(|((i,a),(j,b))| a != b);
    todo!()
}

const START: AmphipodState = AmphipodState {
    pos: todo!()
};

const GOAL: AmphipodState = AmphipodState {
    pos: todo!()
};

fn main() {
    let path = astar_search(
        START,
        GOAL,
        heuristic,
        |s| state_neighbours(s).into_iter(),
        stepcost,
    );
}
