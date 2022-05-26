#![feature(str_split_once)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    let (player_1, player_2) = input.split_once("\n\n").unwrap();

    let player_1_cards: VecDeque<u16> = player_1
        .strip_prefix("Player 1:\n")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let player_2_cards: VecDeque<u16> = player_2
        .strip_prefix("Player 2:\n")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part_1(&player_1_cards, &player_2_cards);
    part_2(&player_1_cards, &player_2_cards);
}

fn part_1(player_1_cards: &VecDeque<u16>, player_2_cards: &VecDeque<u16>) {
    let mut player_1_cards = player_1_cards.clone();
    let mut player_2_cards = player_2_cards.clone();

    while !player_1_cards.is_empty() && !player_2_cards.is_empty() {
        let player_1 = player_1_cards.pop_front().unwrap();
        let player_2 = player_2_cards.pop_front().unwrap();

        if player_1 > player_2 {
            player_1_cards.push_back(player_1);
            player_1_cards.push_back(player_2);
        } else {
            player_2_cards.push_back(player_2);
            player_2_cards.push_back(player_1);
        }
    }

    let winner = if player_1_cards.is_empty() {
        println!("Player 2 WON!");
        player_2_cards
    } else {
        println!("Player 1 WON!");
        player_1_cards
    };

    let score: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * *card as usize)
        .sum();

    println!("Part 1: {}", score);
}

fn part_2(player_1_cards: &VecDeque<u16>, player_2_cards: &VecDeque<u16>) {
    let (_, player_1_cards, player_2_cards) =
        part_2_sub_game(&mut HashSet::new(), player_1_cards, player_2_cards);

    let winner = if player_1_cards.is_empty() {
        println!("Player 2 WON!");
        player_2_cards
    } else {
        println!("Player 1 WON!");
        player_1_cards
    };

    let score: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * *card as usize)
        .sum();

    println!("Part 2: {}", score);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PlayerState {
    player_1: Vec<u16>,
    player_2: Vec<u16>,
}

fn part_2_sub_game(
    start_states: &mut HashSet<PlayerState>,
    player_1_cards: &VecDeque<u16>,
    player_2_cards: &VecDeque<u16>,
) -> (bool, VecDeque<u16>, VecDeque<u16>) {
    let mut player_1_cards = player_1_cards.clone();
    let mut player_2_cards = player_2_cards.clone();

    if !start_states.insert(PlayerState {
        player_1: player_1_cards.iter().cloned().collect(),
        player_2: player_2_cards.iter().cloned().collect(),
    }) {
        println!("Same state, player 1 won!");
        return (true, player_1_cards, player_2_cards);
    }

    while !player_1_cards.is_empty() && !player_2_cards.is_empty() {
        let player_1 = player_1_cards.pop_front().unwrap();
        let player_2 = player_2_cards.pop_front().unwrap();

        let start_subgame =
            player_1_cards.len() >= player_1 as usize && player_2_cards.len() >= player_2 as usize;
        let player_1_win = if start_subgame {
            let (player_1_win, _, _) =
                part_2_sub_game(start_states, &player_1_cards, &player_2_cards);
            player_1_win
        } else {
            player_1 > player_2
        };
        if player_1_win {
            player_1_cards.push_back(player_1);
            player_1_cards.push_back(player_2);
        } else {
            player_2_cards.push_back(player_2);
            player_2_cards.push_back(player_1);
        }
    }

    (!player_1_cards.is_empty(), player_1_cards, player_2_cards)
}
