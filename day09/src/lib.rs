use aoclib::{
    geometry::{Direction, Point},
    parse,
};
use maplit::hashset;
use parse_display::{Display, FromStr};
use std::path::Path;

#[derive(Debug, Clone, Copy, FromStr, Display)]
enum InstructionDirection {
    #[display("R")]
    Right,
    #[display("L")]
    Left,
    #[display("U")]
    Up,
    #[display("D")]
    Down,
}

impl From<InstructionDirection> for Direction {
    fn from(id: InstructionDirection) -> Self {
        match id {
            InstructionDirection::Right => Direction::Right,
            InstructionDirection::Left => Direction::Left,
            InstructionDirection::Up => Direction::Up,
            InstructionDirection::Down => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, FromStr, Display)]
#[display("{direction} {qty}")]
struct Instruction {
    direction: InstructionDirection,
    qty: usize,
}

#[derive(Default, Debug)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Point::default(); num_knots],
        }
    }

    fn tail(&self) -> Point {
        self.knots.last().copied().unwrap_or_default()
    }

    /// `true` when `head` and `tail` are overlapping or within space constraints
    fn obeys_touching_rule(head: Point, tail: Point) -> bool {
        let diff = head - tail;
        (-1..=1).contains(&diff.x) && (-1..=1).contains(&diff.y)
    }

    fn step(&mut self, direction: Direction) {
        if self.knots.is_empty() {
            return;
        }

        self.knots[0] += direction;

        for tail_idx in 1..self.knots.len() {
            let head_idx = tail_idx - 1;
            let head = self.knots[head_idx];
            let tail = self.knots[tail_idx];

            if !Self::obeys_touching_rule(head, tail) {
                let diff = head - tail;
                let dx = diff.x.clamp(-1, 1);
                let dy = diff.y.clamp(-1, 1);
                self.knots[tail_idx] += (dx, dy);
            }

            debug_assert!(Self::obeys_touching_rule(head, self.knots[tail_idx]));
        }
    }
}

fn solve(input: &Path, part: u32, num_knots: usize) -> Result<(), Error> {
    let mut rope = Rope::new(num_knots);
    let mut tail_visited = hashset!(rope.tail());

    for direction in parse::<Instruction>(input)?.flat_map(|instruction| {
        std::iter::repeat::<Direction>(instruction.direction.into()).take(instruction.qty)
    }) {
        rope.step(direction);
        tail_visited.insert(rope.tail());
    }

    println!("tail visited qty (pt. {part}): {}", tail_visited.len());

    Ok(())
}

pub fn part1(input: &Path) -> Result<(), Error> {
    solve(input, 1, 2)
}

pub fn part2(input: &Path) -> Result<(), Error> {
    solve(input, 2, 10)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
