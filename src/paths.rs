use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_paths(n: i32) -> Vec<Vec<i32>> {
    let mut found: HashSet<Vec<i32>> = HashSet::new();

    // Calculate the number of permutations to consider.
    let perms: i32 = (1..n).product();

    for x in 0..perms {
        let possiblity = generate_possibility(x, n);

        if is_reversal(&possiblity) {
            continue;
        }
        let path = normalize_path(possiblity);
        found.insert(path);
    }

    let mut result: Vec<Vec<i32>> = Vec::from_iter(found);
    result.sort();
    result
}

pub fn generate_possibility(perm: i32, n: i32) -> Vec<i32> {
    let mut points: Vec<i32> = Vec::from_iter((1..n).into_iter());
    let mut visits: Vec<i32> = vec![];
    let mut result: Vec<i32> = vec![];
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
    for x in 0..n - 1 {
        let i: usize = x as usize;
        let jump = (n + (visits[i + 1] - visits[i])) % n;

        result.push(jump);
    }
    let jump = (n + (visits[0] - visits[n as usize - 1])) % n;
    result.push(jump);

    result
}

pub fn validate_possibility(path: &Vec<i32>) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut current: i32 = 0;
    let n: i32 = path.len() as i32;

    for x in path.iter() {
        current = (current + x) % n;
        if !visited.insert(current) {
            return false;
        }
    }

    current == 0
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

pub fn normalize_path(mut path: Vec<i32>) -> Vec<i32> {
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
