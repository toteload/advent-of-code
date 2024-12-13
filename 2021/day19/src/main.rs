use itertools::{iproduct, Itertools};
use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{i32, newline},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};
use std::iter;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
use std::{
    fs::File,
    io::{self, Read},
};

type Vec3 = nalgebra::Vector3<i32>;

type Orientation = nalgebra::Matrix3<i32>;

type IndexPair = (usize, usize);

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Vec3>,
    inter_distances: Vec<(u32, IndexPair)>,
}

fn beacon_inter_diffs(beacons: &Vec<Vec3>) -> Vec<Vec3> {
    (0..beacons.len())
        .combinations(2)
        .map(|idx| beacons[idx[0]] - beacons[idx[1]])
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    offset: Vec3,
    orient: Orientation,
}

impl Transform {
    fn new(offset: &Vec3, orient: &Orientation) -> Transform {
        Transform {
            offset: *offset,
            orient: *orient,
        }
    }

    fn identity() -> Transform {
        Transform {
            offset: Vec3::zeros(),
            orient: Orientation::identity(),
        }
    }

    fn from_orientation(orient: &Orientation) -> Transform {
        Transform {
            offset: Vec3::zeros(),
            orient: *orient,
        }
    }

    fn apply(&self, pos: &Vec3) -> Vec3 {
        self.offset + (self.orient * pos)
    }

    fn combine(&self, t: &Transform) -> Transform {
        Transform {
            offset: self.offset + t.offset,
            orient: self.orient * t.orient,
        }
    }
}

fn parse_header(s: &str) -> IResult<&str, &str> {
    let (s, _) = take_till(|c| c == '\n')(s)?;
    take(1usize)(s)
}

fn manhattan_distance(a: &Vec3, b: &Vec3) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>() as u32
}

fn parse_position(s: &str) -> IResult<&str, Vec3> {
    let (s, x) = i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, z) = i32(s)?;

    Ok((s, Vec3::new(x, y, z)))
}

fn compute_inter_distances(beacons: &Vec<Vec3>) -> Vec<(u32, IndexPair)> {
    let mut ds = (0..beacons.len())
        .combinations(2)
        .map(|idx| {
            let i = idx[0];
            let j = idx[1];
            (manhattan_distance(&beacons[i], &beacons[j]), (i, j))
        })
        .collect::<Vec<_>>();

    ds.sort_by(|(a, _), (b, _)| a.cmp(b));

    ds
}

fn parse_scanner(s: &str) -> IResult<&str, Scanner> {
    let (s, beacons) = preceded(parse_header, separated_list0(newline, parse_position))(s)?;
    let inter_distances = compute_inter_distances(&beacons);
    Ok((
        s,
        Scanner {
            beacons,
            inter_distances,
        },
    ))
}

fn parse(s: &str) -> IResult<&str, Vec<Scanner>> {
    separated_list0(tag("\n\n"), parse_scanner)(s)
}

fn vec3_cmp(a: &Vec3, b: &Vec3) -> Ordering {
    a[0].cmp(&b[0]).then(a[1].cmp(&b[1])).then(a[2].cmp(&b[2]))
}

fn find_relative_transform(a: &Scanner, b: &Scanner) -> Option<Transform> {
    let mut beacons_a = a.beacons.clone();
    beacons_a.sort_by(vec3_cmp);
    let mut inter_diffs_a = beacon_inter_diffs(&beacons_a);
    inter_diffs_a.sort_by(vec3_cmp);

    let axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        .iter()
        .map(|v| Vec3::from_column_slice(v))
        .permutations(3);
    let signs = iter::repeat([-1, 1].iter())
        .take(3)
        .multi_cartesian_product();

    assert_eq!(axes.clone().count(), 6);
    assert_eq!(signs.clone().count(), 8);

    let orientations = axes.cartesian_product(signs).map(|(a, s)| {
        let cols = a
            .iter()
            .zip(s)
            .map(|(a, s)| (*s) * (*a))
            .collect::<Vec<_>>();
        Orientation::from_columns(&cols)
    });

    let mut possible_orient = None;

    for orient in orientations {
        let mut oriented_beacons_b = b.beacons.iter().map(|p| orient * p).collect::<Vec<_>>();
        oriented_beacons_b.sort_by(vec3_cmp);
        let mut inter_diffs_b = beacon_inter_diffs(&oriented_beacons_b);
        inter_diffs_b.sort_by(vec3_cmp);

        let overlap_count = count_equals_in_sorted_slices(&inter_diffs_a, &inter_diffs_b, vec3_cmp);

        if overlap_count >= 66 {
            possible_orient = Some(orient);
        }
    }

    dbg!(&possible_orient);

    let Some(orient) = possible_orient else {
        return None;
    };

    let beacons_b = b
        .beacons
        .iter()
        .copied()
        .map(|p| orient * p)
        .collect::<Vec<_>>();

    for (pa, pb) in beacons_a.iter().cartesian_product(beacons_b.iter()) {
        let offset = pa - pb;

        let mut transformed_b = beacons_b
            .iter()
            .copied()
            .map(|p| p + offset)
            .collect::<Vec<_>>();
        transformed_b.sort_by(vec3_cmp);

        let beacon_overlap_count =
            count_equals_in_sorted_slices(&beacons_a, &transformed_b, vec3_cmp);

        if beacon_overlap_count >= 12 {
            return Some(Transform::new(&offset, &orient));
        }
    }

    None
}

fn count_equals_in_sorted_slices<T, F: Fn(&T, &T) -> Ordering>(a: &[T], b: &[T], cmp: F) -> usize {
    let mut ia = 0;
    let mut ib = 0;

    let mut counter = 0;

    while ia < a.len() && ib < b.len() {
        match cmp(&a[ia], &b[ib]) {
            Ordering::Equal => {
                counter += 1;
                ia += 1;
                ib += 1;
            }
            Ordering::Less => ia += 1,
            Ordering::Greater => ib += 1,
        }
    }

    counter
}

fn maybe_neighbour(a: &Scanner, b: &Scanner) -> bool {
    let dsa = a
        .inter_distances
        .iter()
        .map(|(d, _)| d)
        .copied()
        .collect::<Vec<_>>();
    let dsb = b
        .inter_distances
        .iter()
        .map(|(d, _)| d)
        .copied()
        .collect::<Vec<_>>();

    const OVERLAP_THRESHOLD: usize = 66;

    count_equals_in_sorted_slices(&dsa, &dsb, u32::cmp) >= OVERLAP_THRESHOLD
}

fn potential_neighbours(scanners: &[Scanner], base: usize, candidates: &[usize]) -> Vec<usize> {
    candidates
        .iter()
        .copied()
        .filter(|i| maybe_neighbour(&scanners[base], &scanners[*i]))
        .collect()
}

fn find_scanner_locations(scanners: &[Scanner]) -> Vec<Transform> {
    let mut disoriented = (1..scanners.len()).collect::<Vec<_>>();
    let mut oriented = HashMap::from([(0, Transform::identity())]);
    let mut queue = vec![0];

    while let Some(idx) = queue.pop() {
        let potentials = potential_neighbours(scanners, idx, &disoriented);

        for pidx in potentials {
            let base = &scanners[idx];
            let p = &scanners[pidx];

            if let Some(transform) = find_relative_transform(base, p) {
                queue.push(pidx);
                disoriented.remove(disoriented.iter().position(|x| *x == pidx).unwrap());
                oriented.insert(
                    pidx,
                    oriented.get(&idx).map(|t| t.combine(&transform)).unwrap(),
                );
            }
        }
    }

    assert!(disoriented.is_empty());

    (0..scanners.len())
        .map(|i| *oriented.get(&i).unwrap())
        .collect()
}

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    //io::stdin().lock().read_to_string(&mut input).unwrap();

    let (_, scanners) = parse(&input).unwrap();

    //for idx in (0..scanners.len()).combinations(2) {
    //    let i = idx[0];
    //    let j = idx[1];

    //    let a = &scanners[i];
    //    let b = &scanners[j];

    //    if maybe_neighbour(a, b) {
    //        let transform = find_relative_transform(a, b);
    //        dbg!(transform);
    //    }
    //}

    for scanner in scanners.iter().skip(1) {
        dbg!(find_relative_transform(&scanners[0], scanner));
    }

    //let transforms = dbg!(find_scanner_locations(&scanners));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_transform_for_pairs() {
        let a = [Vec3::new(1, 4, 0), Vec3::new(5, 8, 0)];
        let b = [Vec3::new(1, 11, 0), Vec3::new(-3, 7, 0)];
        let ts = find_transforms_between_pair(&a, &b);
        dbg!(&ts);
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn transform_orient() {
        let p = Vec3::new(1, 3, 5);

        let t = Transform {
            offset: Vec3::new(1, 1, 1),
            order: Vec3::new(2, 1, 0),
            sign: Vec3::new(-1, 1, -1),
        };

        assert_eq!(
            apply_transform(&p, &t),
            Vec3::new(-5, 3, -1) + Vec3::new(1, 1, 1)
        );
    }
}
