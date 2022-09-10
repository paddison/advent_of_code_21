use std::{collections::HashMap};
use crate::split_commas;

fn create_spawn_map(fish: Vec<u8>) -> HashMap<u8, u128> {
    let mut spawn_map = HashMap::new();
    for f in fish {
        let amount = spawn_map.entry(f).or_insert(0);
        *amount += 1;
    }
    for i in 0..9 {
        spawn_map.entry(i).or_insert(0);
    }

    spawn_map
}

fn update_spawn_map(spawn_map: &mut  HashMap<u8, u128>) {
    // get 0 values, determine how many will be spawned
    // store 0 values
    // for 1 to 8
    // set map(k - 1) to map(k)
    // add counted 0 to 6, 
    // insert newly spawned
    let zeroes = *spawn_map.get(&0).unwrap();
    for i in 1..9 {
        let current = *spawn_map.get(&i).unwrap();
        let to_update = spawn_map.get_mut(&(i - 1)).unwrap();
        *to_update = current;
    }
    let current = spawn_map.get_mut(&6).unwrap();
    *current += zeroes;
    let to_spawn = spawn_map.get_mut(&8).unwrap();
    *to_spawn = zeroes;
}

fn count_fish(spawn_map: HashMap<u8, u128>) -> u128 {
    let mut sum = 0;
    for i in 0..9 {
        let n = spawn_map.get(&i).unwrap();
        sum += *n;
    }
    sum
}

pub fn solve_6_1(file_name: &str, days: u32) -> u128 {
    let fish = split_commas(file_name);
    let mut spawn_map = create_spawn_map(fish);
    for _ in 0..days {
        update_spawn_map(&mut spawn_map)
    }
    count_fish(spawn_map)
}

#[test]
fn test_spawn_map() {
    let fish = split_commas("data/day_6_test.txt");
    let spawn_map = create_spawn_map(fish);
    assert_eq!(Some(&0), spawn_map.get(&0));
    assert_eq!(Some(&1), spawn_map.get(&1));
    assert_eq!(Some(&1), spawn_map.get(&2));
    assert_eq!(Some(&2), spawn_map.get(&3));
    assert_eq!(Some(&1), spawn_map.get(&4));
    assert_eq!(Some(&0), spawn_map.get(&5));
    assert_eq!(Some(&0), spawn_map.get(&6));
    assert_eq!(Some(&0), spawn_map.get(&7));
    assert_eq!(Some(&0), spawn_map.get(&8));
}

#[test]
fn test_update_spawn_map() {
    let fish = split_commas("data/day_6_test.txt");
    let mut spawn_map = create_spawn_map(fish);
    update_spawn_map(&mut spawn_map);
    assert_eq!(Some(&1), spawn_map.get(&0));
    assert_eq!(Some(&1), spawn_map.get(&1));
    assert_eq!(Some(&2), spawn_map.get(&2));
    assert_eq!(Some(&1), spawn_map.get(&3));
    update_spawn_map(&mut spawn_map);
    assert_eq!(Some(&1), spawn_map.get(&0));
    assert_eq!(Some(&2), spawn_map.get(&1));
    assert_eq!(Some(&1), spawn_map.get(&2));
    assert_eq!(Some(&1), spawn_map.get(&6));
    assert_eq!(Some(&1), spawn_map.get(&8));
}

#[test]
fn test_count_18_days() {
    let fish = split_commas("data/day_6_test.txt");
    let mut spawn_map = create_spawn_map(fish);
    for _ in 0..18 {
        update_spawn_map(&mut spawn_map);
    }
    let population = count_fish(spawn_map);
    assert_eq!(population, 26);
}

#[test]
fn test_count_80_days() {
    let fish = split_commas("data/day_6_test.txt");
    let mut spawn_map = create_spawn_map(fish);
    for _ in 0..80 {
        update_spawn_map(&mut spawn_map);
    }
    let population = count_fish(spawn_map);
    assert_eq!(population, 5934);
}