use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    // let solve = part_1(INPUT);
    let solve = part_2(INPUT);
    println!("{solve}")
}

fn part_1(input: &str) -> usize {
    let s = Instant::now();
    let ans = input.split(',').map(hash).sum();
    let e = Instant::now();
    println!("Processed in {:?}", e.duration_since(s));

    ans
}

fn part_2(input: &str) -> usize {
    let s = Instant::now();
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for s in input.split(',') {
        let mut operation = '.';
        let (label, focal_length) = s
            .split_once(|c| match c {
                '-' => {
                    operation = '-';
                    return true;
                }
                '=' => {
                    operation = '=';
                    return true;
                }
                _ => false,
            })
            .expect("matches example");

        let h = hash(label);

        // println!("STEP: {s} OPERATION: {operation} LABEL: {label} HASH: {h}");

        map.entry(h)
            .and_modify(|e| match operation {
                '-' => {
                    if let Some(p) = e.iter().position(|s| s.contains(label)) {
                        e.remove(p);
                    }
                }
                '=' => {
                    let val = format!("{label} {focal_length}");
                    if let Some(p) = e.iter().position(|s| s.contains(label)) {
                        e[p] = val;
                    } else {
                        e.push(val);
                    }
                }
                _ => unreachable!("operation will only be - or ="),
            })
            .or_insert_with(|| match operation {
                '-' => vec![],
                '=' => {
                    vec![format!("{label} {focal_length}")]
                }
                _ => unreachable!("operation will only be - or ="),
            });
    }

    let mut total = 0;

    for (k, v) in map {
        let box_num = k + 1;
        for (lens_idx, lens) in v.iter().enumerate() {
            let slot_num = lens_idx + 1;
            let focal_length = lens.split_once(' ').unwrap().1;
            total += box_num * slot_num * focal_length.parse::<usize>().unwrap()
        }
    }

    let e = Instant::now();
    println!("Processed in {:?}", e.duration_since(s));

    total
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |a, c| (a + c as usize) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        )
    }
}
