use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(PartialEq, Eq)]
struct HeapEntry<T: Eq + PartialEq> {
    fscore: u64,
    state: T,
}

impl<T: Eq> Ord for HeapEntry<T> {
    fn cmp(&self, other: &HeapEntry<T>) -> std::cmp::Ordering {
        self.fscore.cmp(&other.fscore)
    }
}

impl<T: Eq> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &HeapEntry<T>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar_search<T, H, N, M, S>(start: T, goal: T, heuristic: H, neighbours: N, stepcost: S) -> Option<Vec<T>> 
where
    T: Clone + PartialEq + Eq + Hash,
    H: Fn(&T) -> u64,
    N: Fn(&T) -> M,
    M: Iterator<Item=T>,
    S: Fn(&T, &T) -> u64
    {

    let mut open: BinaryHeap<HeapEntry<T>> = BinaryHeap::new();
    open.push(HeapEntry { fscore: heuristic(&start), state: start.clone(), });

    let mut parent: HashMap<T, T> = HashMap::new();

    // The g score of a node is the cost of the cheapest path from start to n currently known.
    let mut gscore: HashMap<T, u64> = HashMap::from([(start.clone(), 0)]);

    // The f score is the predicted cost of a path from start to goal through n.
    // fscore[n] = gscore[n] + heuristic(n)
    let mut fscore: HashMap<T, u64> = HashMap::from([(start.clone(), heuristic(&start))]);

    while let Some(HeapEntry { state: current, ..}) = open.pop() {
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
            let tentative_gscore = gscore.get(&current).unwrap_or(&u64::MAX) + stepcost(&current, &neighbour);
            if tentative_gscore < *gscore.get(&neighbour).unwrap_or(&u64::MAX) {
                parent.insert(neighbour.clone(), current.clone());
                gscore.insert(neighbour.clone(), tentative_gscore);
                let neighbour_fscore = tentative_gscore + heuristic(&neighbour);
                fscore.insert(neighbour.clone(), neighbour_fscore);

                let open_contains_neighbour = open.iter().find(|e| e.state == neighbour).is_some();
                if open_contains_neighbour {
                    open.push(HeapEntry {
                        fscore: neighbour_fscore,
                        state: neighbour,
                    });
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]

}

fn main() {

}