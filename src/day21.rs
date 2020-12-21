use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day21.input");
    let (answer1, answer2) = solve(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve(lines: &[String]) -> (usize, String) {
    let mut parsed_ingredients: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut parsed_allergens: HashMap<String, usize> = HashMap::new();
    let mut claimed_ingredients: HashMap<String, String> = HashMap::new();
    let mut claimed_allergen: HashMap<String, String> = HashMap::new();
    for line in lines {
        let s = line
            .strip_suffix(")")
            .unwrap()
            .split(" (contains ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let allergens = s[1]
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let ingredients = s[0]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        for ingredient in ingredients.iter() {
            *parsed_allergens.entry(ingredient.to_string()).or_insert(0) += 1;
        }
        for allergen in allergens.iter() {
            for ingredient in ingredients.iter() {
                *parsed_ingredients
                    .entry(allergen.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(ingredient.to_string())
                    .or_insert(0) += 1;
            }
        }
    }
    for (_, ingredient) in parsed_ingredients.iter_mut() {
        let max = ingredient
            .iter()
            .fold(0, |acc, (_, t)| if t > &acc { *t } else { acc });
        ingredient.drain_filter(|_, v| *v < max);
    }
    loop {
        if parsed_ingredients.len() == 0 {
            break;
        }
        parsed_ingredients.drain_filter(|allergen, ingredient| {
            ingredient.drain_filter(|i, _| claimed_ingredients.contains_key(i));
            if ingredient.len() == 1 {
                let only_ingredient_left = ingredient.iter().next().unwrap().0;
                claimed_allergen.insert(allergen.to_string(), only_ingredient_left.to_string());
                claimed_ingredients.insert(only_ingredient_left.to_string(), allergen.to_string());
                true
            } else {
                false
            }
        });
    }
    let mut dangerous_list = claimed_allergen
        .iter()
        .map(|(a, i)| (a.to_string(), i.to_string()))
        .collect::<Vec<(String, String)>>();
    dangerous_list.sort();
    let dangerous_str = dangerous_list
        .iter()
        .map(|(_, i)| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let not_dangerous_count = parsed_allergens.iter().fold(0, |acc, (ingredient, count)| {
        if claimed_ingredients.contains_key(ingredient) {
            acc
        } else {
            acc + count
        }
    });
    (not_dangerous_count, dangerous_str)
}

fn read_lines_as_str<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
