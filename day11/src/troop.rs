use crate::models::Monkey;

/// Produce trace output if the specified variable is non-empty in the environment
macro_rules! trace_if_set {
    ($key:expr, $format:literal $(, $arg:expr)*) => {
        if !std::env::var($key).unwrap_or_default().is_empty() {
            eprintln!($format, $($arg),*);
        }
    };
}

pub struct Troop {
    monkeys: Vec<Monkey>,
    part_one: bool,
}

impl Troop {
    pub fn new(monkeys: Vec<Monkey>, part_one: bool) -> Self {
        Self { monkeys, part_one }
    }

    fn turn_for(&mut self, monkey_idx: usize) {
        macro_rules! monkey {
            ($idx:expr) => {
                self.monkeys
                    .get($idx)
                    .expect("only valid indices are ever requested")
            };
        }
        macro_rules! monkey_mut {
            ($idx:expr) => {
                self.monkeys
                    .get_mut($idx)
                    .expect("only valid indices are ever requested")
            };
        }

        trace_if_set!("MONKEY_TRACE", "Monkey {monkey_idx}:");

        while let Some(mut item_worry) = monkey_mut!(monkey_idx).items.pop_front() {
            monkey_mut!(monkey_idx).inspect_count += 1;

            trace_if_set!(
                "MONKEY_TRACE",
                "  Monkey inspects an item with a worry level of {item_worry}."
            );

            let monkey = monkey!(monkey_idx);
            item_worry = monkey.operation.perform(item_worry);
            trace_if_set!("MONKEY_TRACE", "    Worry level increases to {item_worry}.");

            if self.part_one {
                item_worry /= 3;
                trace_if_set!(
                    "MONKEY_TRACE",
                    "    Monkey gets bored with item. Worry reduced to {item_worry}."
                );
            }

            let divisibility;
            let destination = if item_worry % monkey.test.divisible_by == 0 {
                divisibility = "is";
                monkey.true_destination.0
            } else {
                divisibility = "is not";
                monkey.false_destination.0
            };
            trace_if_set!(
                "MONKEY_TRACE",
                "    Current worry level {divisibility} divisible by {}.",
                monkey.test.divisible_by
            );
            trace_if_set!(
                "MONKEY_TRACE",
                "    Item with worry level {item_worry} is thrown to monkey {destination}."
            );
            monkey_mut!(destination).items.push_back(item_worry);
        }
    }

    pub fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            self.turn_for(idx);
        }
    }

    pub fn active_monkeys(self, n: usize) -> Vec<Monkey> {
        let mut monkeys = self.monkeys;
        monkeys.sort_by_key(|monkey| std::cmp::Reverse(monkey.inspect_count));
        monkeys.truncate(n);
        monkeys
    }
}
