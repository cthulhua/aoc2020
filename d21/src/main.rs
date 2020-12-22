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
    let allergens_to_ingredients: HashMap<String, HashSet<String>> = all_allergens
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
    let allergic_ingredients: HashSet<String> = allergens_to_ingredients
        .values()
        .fold(HashSet::new(), |acc, set| {
            acc.union(&set).cloned().collect()
        });
    let ingredients: Vec<String> = facts
        .iter()
        .map(|fact| fact.ingredients.clone())
        .flatten()
        .filter(|ingredient| !allergic_ingredients.contains(ingredient))
        .collect();
    dbg!(ingredients.len());
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
