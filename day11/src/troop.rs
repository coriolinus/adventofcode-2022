use crate::env_is_set;
use crate::models::Monkey;

fn least_common_multiple_many(of: impl IntoIterator<Item = u128>) -> Option<u128> {
    of.into_iter().reduce(num_integer::lcm)
}

pub struct Troop {
    monkeys: Vec<Monkey>,
    test_lcm: Option<u128>,
}

impl Troop {
    pub fn new(monkeys: Vec<Monkey>, part_one: bool) -> Self {
        let test_lcm = (!part_one).then(|| {
            least_common_multiple_many(monkeys.iter().map(|monkey| monkey.test.divisible_by))
                .unwrap_or(1)
        });
        Self { monkeys, test_lcm }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Monkey> {
        self.monkeys.iter()
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

        if env_is_set("MONKEY_TRACE") {
            eprintln!("Monkey {monkey_idx}:");
        }

        while let Some(mut item_worry) = monkey_mut!(monkey_idx).items.pop_front() {
            monkey_mut!(monkey_idx).inspect_count += 1;

            if env_is_set("MONKEY_TRACE") {
                eprintln!("  Monkey inspects an item with a worry level of {item_worry}.");
            }

            let monkey = monkey!(monkey_idx);
            item_worry = monkey.operation.perform(item_worry);
            if env_is_set("MONKEY_TRACE") {
                eprintln!("    Worry level increases to {item_worry}.");
            }

            if let Some(lcm) = self.test_lcm {
                item_worry %= lcm;
            } else {
                item_worry /= 3;
                if env_is_set("MONKEY_TRACE") {
                    eprintln!("    Monkey gets bored with item. Worry reduced to {item_worry}.");
                }
            }

            let divisibility;
            let destination = if item_worry % monkey.test.divisible_by == 0 {
                divisibility = "is";
                monkey.true_destination.0
            } else {
                divisibility = "is not";
                monkey.false_destination.0
            };
            if env_is_set("MONKEY_TRACE") {
                eprintln!(
                    "    Current worry level {divisibility} divisible by {}.",
                    monkey.test.divisible_by
                );
                eprintln!(
                    "    Item with worry level {item_worry} is thrown to monkey {destination}."
                );
            }
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
