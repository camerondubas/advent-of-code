use primes::{PrimeSet, Sieve};
use std::vec;

use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn lowest_common_multiple(nums: Vec<u64>) -> u64 {
    let mut pset = Sieve::new();
    let mut table = nums.into_iter();
    let mut prime_iter = pset.iter();
    let mut divisors: Vec<u64> = vec![];
    let mut prime = prime_iter.next().unwrap();

    while !table.clone().all(|n| n == 1) {
        if table.clone().any(|n| n % prime == 0) {
            divisors.push(prime);

            table = table
                .map(|n| match n % prime == 0 {
                    true => n / prime,
                    false => n,
                })
                .collect_vec()
                .into_iter();
        } else {
            prime = prime_iter.next().unwrap();
        }
    }

    divisors.iter().product::<u64>()
}
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Operation {
    val: String,
    operator: Operator,
}

impl Operation {
    fn from(raw_operation: &str) -> Operation {
        let mut parts = raw_operation.split(' ');
        parts.next();

        Operation {
            operator: match parts.next().unwrap() {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => panic!("Unknown operator"),
            },
            val: parts.next().unwrap().to_string(),
        }
    }

    fn apply(&self, item: u64) -> u64 {
        match self.operator {
            Operator::Add => item + self.val.parse().unwrap_or(item),
            Operator::Multiply => item * self.val.parse().unwrap_or(item),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Test {
    fn from(raw_test: Vec<&str>) -> Test {
        let mut nums = raw_test
            .iter()
            .map(|s| s.split(' ').last().unwrap().parse::<u64>().unwrap());
        Test {
            divisor: nums.next().unwrap(),
            true_monkey: nums.next().unwrap() as usize,
            false_monkey: nums.next().unwrap() as usize,
        }
    }

    fn run(self, worry_level: u64) -> usize {
        if worry_level % self.divisor == 0 {
            return self.true_monkey;
        } else {
            return self.false_monkey;
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    inspect_count: u64,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn from(raw_monkey: &str) -> Monkey {
        let mut lines = raw_monkey.lines();
        lines.next();

        let items = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let operation = lines.next().unwrap().split('=').nth(1).unwrap().trim();
        let test = lines.collect_vec();
        Monkey {
            inspect_count: 0,
            items,
            operation: Operation::from(operation),
            test: Test::from(test),
        }
    }

    fn inspect(&mut self, idx: usize, reduce_stress: &dyn Fn(u64) -> u64) -> (usize, u64) {
        let item = self.items[idx];
        let mut worry_level = self.operation.apply(item);
        worry_level = reduce_stress(worry_level);

        let target_monkey = self.test.run(worry_level);
        (target_monkey, worry_level)
    }
}

fn part_1(input: &String) -> u64 {
    let monkeys = input
        .split("\n\n")
        .map(|raw| Monkey::from(raw))
        .collect_vec();
    let rounds = 20;
    let reduce_stress = |worry_level: u64| worry_level / 3;

    monkey_business(monkeys, rounds, &reduce_stress)
}

fn part_2(input: &String) -> u64 {
    let monkeys = input
        .split("\n\n")
        .map(|raw| Monkey::from(raw))
        .collect_vec();
    let rounds = 10_000;
    let lcm = lowest_common_multiple(monkeys.iter().map(|m| m.test.divisor).collect_vec());
    let reduce_stress = |worry_level: u64| worry_level % lcm;

    monkey_business(monkeys, rounds, &reduce_stress)
}

fn monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: u64,
    reduce_stress: &dyn Fn(u64) -> u64,
) -> u64 {
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let mut monkey = monkeys[monkey_idx].clone();

            for item_idx in 0..monkey.items.len() {
                let (target_monkey, item) = monkey.inspect(item_idx, &reduce_stress);

                monkeys[monkey_idx].inspect_count += 1;
                monkeys[monkey_idx].items.remove(0);
                monkeys[target_monkey].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspect_count)
}
