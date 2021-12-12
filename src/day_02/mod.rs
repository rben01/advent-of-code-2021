use crate::Answer;

struct Position {
    h: i32,
    v: i32,
}

fn do_pt1<S: AsRef<str>>(s: S) -> Option<Position> {
    let mut h = 0;
    let mut v = 0;
    for line in s.as_ref().lines() {
        let mut tokens_iter = line.split_whitespace();
        let dir = tokens_iter.next()?;
        let dist = tokens_iter.next()?.parse::<i32>().ok()?;

        match dir {
            "forward" => h += dist,
            "up" => v -= dist,
            "down" => v += dist,
            _ => return None,
        }
    }

    Some(Position { h, v })
}
fn do_pt2<S: AsRef<str>>(s: S) -> Option<Position> {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    for line in s.as_ref().lines() {
        let mut tokens_iter = line.split_whitespace();
        let dir = tokens_iter.next()?;
        let dist = tokens_iter.next()?.parse::<i32>().ok()?;

        match dir {
            "forward" => {
                h += dist;
                v += aim * dist
            }
            "up" => aim -= dist,
            "down" => aim += dist,
            _ => return None,
        }
    }

    Some(Position { h, v })
}

pub(crate) fn pt1() -> i32 {
    let Position { h, v } = do_pt1(include_str!("./input.txt")).unwrap();
    h * v
}

pub(crate) fn pt2() -> i32 {
    let Position { h, v } = do_pt2(include_str!("./input.txt")).unwrap();
    h * v
}

pub fn ans() -> Answer<i32, i32> {
    (pt1(), pt2()).into()
}
