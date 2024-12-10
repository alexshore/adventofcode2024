use aoc_runner_derive::{aoc, aoc_generator};

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Int = usize;

type Rules = HashMap<Int, HashSet<Int>>;
type Book = Vec<Int>;
type Books = Vec<Book>;
type ParsedInput = (Rules, Books);

#[aoc_generator(day5)]
fn parse_input(input: &str) -> ParsedInput {
    let (rules_input, books_input) = input.split_once("\n\n").unwrap();

    let mut rules: Rules = HashMap::new();

    for l in rules_input.lines() {
        let (key, val) = l.split_once('|').unwrap();

        rules
            .entry(key.parse().unwrap())
            .or_default()
            .insert(val.parse().unwrap());
    }

    let books: Books = books_input
        .lines()
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    (rules, books)
}

fn is_valid_book(rules: &Rules, book: &Book) -> bool {
    let mut found_pages = HashSet::new();

    for page in book {
        let page_rule = rules.get(page).unwrap();

        if !found_pages.is_disjoint(page_rule) {
            return false;
        }

        found_pages.insert(*page);
    }

    true
}

#[aoc(day5, part1)]
fn part_one(input: &ParsedInput) -> Int {
    let (rules, books) = input;

    let mut res = 0;

    for book in books.iter() {
        if is_valid_book(rules, book) {
            res += book[book.len() / 2]
        }
    }

    res
}

#[aoc(day5, part2)]
fn part_two(input: &ParsedInput) -> Int {
    let (rules, mut books) = input.to_owned();

    let mut res = 0;

    for book in books.iter_mut() {
        if !is_valid_book(&rules, book) {
            book.sort_by(|a, b| match rules[a].contains(b) {
                true => Ordering::Less,
                _ => Ordering::Equal,
            });
            res += book[book.len() / 2]
        }
    }

    res
}
