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
    head: Point,
    tail: Point,
}

impl Rope {
    /// `true` when `head` and `tail` are overlapping or within space constraints
    fn obeys_touching_rule(&self) -> bool {
        let diff = self.head - self.tail;
        (-1..=1).contains(&diff.x) && (-1..=1).contains(&diff.y)
    }

    fn step(&mut self, direction: Direction) {
        self.head += direction;
        if !self.obeys_touching_rule() {
            let diff = self.head - self.tail;
            let dx = diff.x.clamp(-1, 1);
            let dy = diff.y.clamp(-1, 1);
            self.tail += (dx, dy);
        }
        debug_assert!(self.obeys_touching_rule());
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut rope = Rope::default();
    let mut tail_visited = hashset!(rope.tail);

    for direction in parse::<Instruction>(input)?.flat_map(|instruction| {
        std::iter::repeat::<Direction>(instruction.direction.into()).take(instruction.qty)
    }) {
        rope.step(direction);
        tail_visited.insert(rope.tail);
    }

    println!("tail visited qty: {}", tail_visited.len());

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!("input file: {:?}", input)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
