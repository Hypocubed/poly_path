use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolyPath {
    pub size: i32,
    pub path: Vec<i32>,
}

impl PolyPath {
    pub fn new_from_verts(verts: &Vec<i32>) -> PolyPath {
        if !PolyPath::validate_verts(verts) {
            panic!("The vertex path {:?} is invalid.", verts);
        }

        let result: PolyPath = PolyPath {
            size: verts.len() as i32,
            path: PolyPath::normalize_path(PolyPath::verts_to_path(verts)),
        };

        return result;
    }

    // Confirms that each vert only appears once.
    pub fn validate_verts(verts: &Vec<i32>) -> bool {
        let size: usize = verts.len();
        let mut visited: HashSet<i32> = HashSet::new();

        if size < 3 {
            return false;
        }

        for x in 0..size {
            if visited.contains(&verts[x]) || verts[x] < 0 || verts[x] >= size as i32 {
                return false;
            }

            visited.insert(verts[x]);
        }

        return true;
    }

    fn verts_to_path(verts: &Vec<i32>) -> Vec<i32> {
        let size: usize = verts.len();
        let mut path: Vec<i32> = vec![];

        for x in 0..size - 1 {
            let i: usize = x as usize;
            let jump = (size as i32 + (verts[i + 1] - verts[i])) % size as i32;

            path.push(jump);
        }
        let jump = (size as i32 + (verts[0] - verts[size as usize - 1])) % size as i32;
        path.push(jump);

        return path;
    }

    fn normalize_path(mut path: Vec<i32>) -> Vec<i32> {
        let n: i32 = path.len() as i32;
        let mut possiblities: Vec<Vec<i32>> = vec![];

        // Get rotations
        for _x in 0..path.len() {
            possiblities.push(path.clone());

            let jump: i32 = path.pop().unwrap();
            path.insert(0, jump);
        }

        // Get reverse rotations
        path.reverse();
        for _x in 0..path.len() {
            possiblities.push(path.clone());

            let jump: i32 = path.pop().unwrap();
            path.insert(0, jump);
        }

        // Get mirrored rotations
        for _x in 0..path.len() {
            let jump: i32 = (n - path.pop().unwrap()) % n;
            path.insert(0, jump);
        }
        for _x in 0..path.len() {
            possiblities.push(path.clone());

            let jump: i32 = path.pop().unwrap();
            path.insert(0, jump);
        }

        // Get reverse mirrored rotations
        path.reverse();
        for _x in 0..path.len() {
            possiblities.push(path.clone());

            let jump: i32 = path.pop().unwrap();
            path.insert(0, jump);
        }

        possiblities.sort();
        possiblities.first().unwrap().to_vec()
    }
}

pub fn generate_possibility(perm: i32, n: i32) -> Vec<i32> {
    let mut points: Vec<i32> = Vec::from_iter((1..n).into_iter());
    let mut visits: Vec<i32> = vec![];
    let mut total: i32 = perm;
    let mut current: i32;

    // Get permutation of points.
    visits.push(0);
    for x in 1..n {
        current = total % (n - x);
        total = total / (n - x);

        visits.push(points.remove(current as usize));
    }

    // Convert into jump path
    // for x in 0..n - 1 {
    //     let i: usize = x as usize;
    //     let jump = (n + (visits[i + 1] - visits[i])) % n;
    //
    //     result.push(jump);
    // }
    // let jump = (n + (visits[0] - visits[n as usize - 1])) % n;
    // result.push(jump);

    return visits;
}

pub fn find_paths(n: i32) -> Vec<PolyPath> {
    let mut found: HashSet<PolyPath> = HashSet::new();

    // Calculate the number of permutations to consider.
    let perms: i32 = (1..n).product();

    for x in 0..perms {
        let possiblity = generate_possibility(x, n);

        // if is_reversal(&possiblity) {
        //     continue;
        // }

        let path = PolyPath::new_from_verts(&possiblity);
        found.insert(path);
    }

    let mut result: Vec<PolyPath> = Vec::from_iter(found);
    result.sort();
    result
}

pub fn is_reversal(path: &Vec<i32>) -> bool {
    let below: i32 = path.len() as i32 / 2;
    let above: i32 = (path.len() as i32 / 2) + (path.len() as i32 % 2);

    let mut low = 0;
    let mut high = 0;

    for x in path.iter() {
        if x < &below {
            low = low + 1;
        } else if x > &above {
            high = high + 1;
        }
    }

    return high > low;
}
