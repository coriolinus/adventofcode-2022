use aoclib::{
    geometry::{tile::Bool, Map, Point},
    parse,
};
use parse_display::{Display, FromStr};
use std::{
    fmt,
    ops::{Index, IndexMut},
    path::Path,
};

#[derive(Debug)]
struct Cpu {
    program: Vec<Instruction>,
    register: i32,
    cycle_counter: i32,
    cycle_when_instruction_completes: i32,
    pending_instruction: Instruction,
    next_instruction_idx: usize,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            register: 1,
            cycle_counter: Default::default(),
            cycle_when_instruction_completes: Default::default(),
            pending_instruction: Default::default(),
            next_instruction_idx: Default::default(),
        }
    }

    fn tick(&mut self) {
        // first apply / update the completed instruction
        if self.cycle_counter == self.cycle_when_instruction_completes {
            // apply the instruction currently pending
            match self.pending_instruction {
                Instruction::Noop => {}
                Instruction::Addx(v) => self.register += v,
            }

            // set up the next pending instruction
            if self.next_instruction_idx < self.program.len() {
                self.pending_instruction = self.program[self.next_instruction_idx];
                self.cycle_when_instruction_completes =
                    self.cycle_counter + self.pending_instruction.cycles();
                self.next_instruction_idx += 1;
            }
        }

        // bookkeeping
        self.cycle_counter += 1;
    }

    fn trace<'a, F, V>(&'a mut self, inspector: F) -> impl '_ + Iterator<Item = V>
    where
        F: 'a + Fn(&Self) -> V,
    {
        std::iter::from_fn(move || {
            (self.cycle_counter <= self.cycle_when_instruction_completes).then(|| {
                // first get the return value, then tick
                let return_value = inspector(self);
                self.tick();
                return_value
            })
        })
    }

    fn signal_strength(&self) -> i32 {
        self.cycle_counter * self.register
    }
}

#[derive(Default, Debug, Clone, Copy, FromStr, Display)]
enum Instruction {
    #[default]
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i32),
}

impl Instruction {
    fn cycles(self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

// pass through only those items with interesting indices in the stream
fn filter_interesting<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    const INTERESTING: [usize; 6] = [20, 60, 100, 140, 180, 220];
    iter.enumerate()
        .filter(|(idx, _value)| INTERESTING.contains(idx))
        .map(|(_idx, value)| value)
}

struct Screen {
    buffer: Map<Bool>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            buffer: Map::new(40, 6),
        }
    }

    fn beam_position(&self, idx: usize) -> Point {
        let width = self.buffer.width();

        let y = idx / width;
        let x = idx % width;
        (x, y).into()
    }
}

impl Index<usize> for Screen {
    type Output = Bool;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(self.beam_position(index))
    }
}

impl IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.buffer.index_mut(self.beam_position(index))
    }
}

struct VideoSystem {
    cpu: Cpu,
    screen: Screen,
}

impl VideoSystem {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            cpu: Cpu::new(program),
            screen: Screen::new(),
        }
    }

    fn scan(&mut self) {
        const SPRITE: [i32; 3] = [-1, 0, 1];
        for (idx, x_register) in self
            .cpu
            .trace(|cpu| cpu.register)
            // skip 1 because the first trace output comes from before cycle 1
            .skip(1)
            .enumerate()
        {
            let x = self.screen.beam_position(idx).x;
            if SPRITE
                .iter()
                .map(|offset| offset + x_register)
                .any(|sprite| sprite == x)
            {
                self.screen[idx] = true.into()
            }
        }
    }
}

impl fmt::Display for VideoSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.screen.buffer.flip_vertical().fmt(f)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let program: Vec<Instruction> = parse(input)?.collect();
    let mut cpu = Cpu::new(program);
    let signal_strength_sum: i32 =
        filter_interesting(cpu.trace(|cpu| cpu.signal_strength()).enumerate())
            .map(|(_idx, signal_strength)| signal_strength)
            .sum();
    println!("sum of signal strength: {signal_strength_sum}");

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let program: Vec<Instruction> = parse(input)?.collect();
    let mut video_system = VideoSystem::new(program);
    video_system.scan();
    println!("video system shows:\n{video_system}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
