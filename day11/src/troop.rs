use crate::models::Monkey;

struct Troop(Vec<Monkey>);

impl Troop {
    fn turn_for(&mut self, monkey_idx: usize) {
        macro_rules! monkey {
            ($idx:expr) => {
                self.0
                    .get($idx)
                    .expect("only valid indices are ever requested")
            };
        }
        macro_rules! monkey_mut {
            ($idx:expr) => {
                self.0
                    .get_mut($idx)
                    .expect("only valid indices are ever requested")
            };
        }

        eprintln!("Monkey {monkey_idx}:");

        while let Some(mut item_worry) = monkey_mut!(monkey_idx).items.pop_front() {
            eprintln!("  Monkey inspects an item with a worry level of {item_worry}.");

            let monkey = monkey!(monkey_idx);
            item_worry = monkey.operation.perform(item_worry);
            eprintln!("    Worry level increases to {item_worry}.");
            item_worry /= 3;
            eprintln!("    Monkey gets bored with item. Worry reduced to {item_worry}.");
            let mut divisibility;
            let destination = if item_worry % monkey.test.divisible_by == 0 {
                divisibility = "is";
                monkey.true_destination.0
            } else {
                divisibility = "is not";
                monkey.false_destination.0
            };
            eprintln!(
                "    Current worry level {divisibility} divisible by {}.",
                monkey.test.divisible_by
            );
            eprintln!("    Item with worry level {item_worry} is thrown to monkey {destination}.");
            monkey_mut!(destination).items.push_back(item_worry);
        }
    }
}
