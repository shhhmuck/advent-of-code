const INPUT: &str = include_str!("./input.txt");
fn main() {
    let solve = part_1(INPUT);
    println!("{solve}")
}

fn part_1(input: &str) -> usize {
    input.split(',').map(hash).sum::<usize>()
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
}
