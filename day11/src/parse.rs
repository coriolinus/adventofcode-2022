use std::{collections::VecDeque, path::Path};

use parse_display::{Display, FromStr};

use crate::{
    models::{Monkey, MonkeyBuilder, MonkeyId, Operation, Test},
    Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromStr, Display)]
#[display("If {output}: throw to monkey {destination}")]
struct ConditionOutput {
    output: bool,
    destination: MonkeyId,
}

#[derive(FromStr, Display)]
enum InputLine {
    #[display("Monkey {0}:")]
    Monkey(MonkeyId),
    #[display("  Starting items: {0}")]
    #[from_str(regex = r"^\s*Starting items: (?P<0>[\d, ]+)$")]
    StartingItems(String),
    #[display("{0}")]
    Operation(Operation),
    #[display("{0}")]
    Test(Test),
    #[display("{0}")]
    ConditionOutput(ConditionOutput),
    #[display("")]
    #[from_str(regex = r"^$")]
    Blank,
}

pub fn parse(input: &Path) -> Result<Vec<Monkey>, Error> {
    fn set_or_err(
        builder: &mut Option<MonkeyBuilder>,
        update: impl FnOnce(MonkeyBuilder) -> MonkeyBuilder,
    ) -> Result<(), Error> {
        let inner = builder.take().ok_or(Error::MissingContent)?;
        *builder = Some(update(inner));
        Ok(())
    }

    fn build_monkey(
        builder: &mut Option<MonkeyBuilder>,
        monkeys: &mut Vec<Monkey>,
    ) -> Result<(), Error> {
        let monkey = builder.take().ok_or(Error::MissingContent)?.build()?;
        if monkey.id.0 != monkeys.len() {
            return Err(Error::MisplacedMonkey(monkeys.len(), monkey.id.0));
        }
        monkeys.push(monkey);
        Ok(())
    }

    use aoclib::parse;

    let mut monkeys = Vec::new();
    let mut builder: Option<MonkeyBuilder> = None;

    for line in parse::<InputLine>(input)? {
        match line {
            InputLine::Monkey(id) => {
                let result = build_monkey(&mut builder, &mut monkeys);
                // we don't have a proper builder for the very first monkey
                if !(id.0 == 0 && monkeys.is_empty() && result.is_err()) {
                    result?;
                }
                builder = Some(MonkeyBuilder::default().id(id));
            }
            InputLine::StartingItems(items_str) => {
                let mut items = VecDeque::new();
                for substr in items_str.split(',') {
                    let item: u64 = substr.trim().parse().map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "could not parse monkey items",
                        )
                    })?;
                    items.push_back(item);
                }

                set_or_err(&mut builder, |builder| builder.items(items))?;
            }
            InputLine::Operation(operation) => {
                set_or_err(&mut builder, |builder| builder.operation(operation))?;
            }
            InputLine::Test(test) => {
                set_or_err(&mut builder, |builder| builder.test(test))?;
            }
            InputLine::ConditionOutput(condition_output) => {
                if condition_output.output {
                    set_or_err(&mut builder, |builder| {
                        builder.true_destination(condition_output.destination)
                    })?;
                } else {
                    set_or_err(&mut builder, |builder| {
                        builder.false_destination(condition_output.destination)
                    })?;
                }
            }
            InputLine::Blank => {}
        }
    }
    build_monkey(&mut builder, &mut monkeys)?;

    Ok(monkeys)
}
