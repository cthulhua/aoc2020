use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);
    let facts: Vec<Statement> = buffered
        .lines()
        .map(|l| Statement::new(&l.unwrap()))
        .collect();
    let all_allergens: HashSet<String> = facts.iter().fold(HashSet::new(), |acc, fact| {
        acc.union(&fact.allergens).cloned().collect()
    });
    let mut allergens_to_ingredients: HashMap<String, HashSet<String>> = all_allergens
        .iter()
        .map(|allergen| {
            (
                allergen.clone(),
                facts
                    .iter()
                    .filter_map(|fact| {
                        if fact.allergens.contains(allergen) {
                            Some(fact.ingredients.clone())
                        } else {
                            None
                        }
                    })
                    .fold(None, |acc, set| match acc {
                        None => Some(set),
                        Some(collected) => Some(collected.intersection(&set).cloned().collect()),
                    }),
            )
        })
        .map(|(allergen, o)| {
            (
                allergen,
                match o {
                    None => HashSet::new(),
                    Some(set) => set,
                },
            )
        })
        .collect();
    let mut canonical_dangerous_ingredients: Vec<(String, String)> = vec![];
    while !allergens_to_ingredients.is_empty() {
        let known_ingredients: HashMap<String, String> = allergens_to_ingredients
            .iter()
            .filter_map(|(k, v)| {
                if v.len() == 1 {
                    Some((v.iter().next().unwrap().clone(), k.clone()))
                } else {
                    None
                }
            })
            .collect();
        for (k, v) in known_ingredients {
            canonical_dangerous_ingredients.push((k.clone(), v));
            allergens_to_ingredients.iter_mut().for_each(|(_, v)| {
                v.remove(&k);
            });
            allergens_to_ingredients = allergens_to_ingredients
                .iter()
                .filter_map(|(k, v)| {
                    if v.is_empty() {
                        None
                    } else {
                        Some((k.clone(), v.clone()))
                    }
                })
                .collect()
        }
    }
    canonical_dangerous_ingredients.sort_by_key(|t| t.1.clone());
    let sorted_vec: Vec<String> = canonical_dangerous_ingredients
        .iter()
        .map(|t| t.0.clone())
        .collect();
    dbg!(sorted_vec.join(","));

    Ok(())
}

#[derive(Debug)]
struct Statement {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
impl Statement {
    fn new(s: &str) -> Self {
        let mut iter = s[..s.len() - 1].split(" (contains ");
        let ingredients = iter.next().unwrap();
        let allergens = iter.next().unwrap();
        let ingredients = ingredients.split_whitespace().map(String::from).collect();
        let allergens = allergens.split(", ").map(String::from).collect();
        Statement {
            ingredients,
            allergens,
        }
    }
}
