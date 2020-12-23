use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;

fn main() {
    // state
    let mut ingredient_counts: HashMap<String, i32> = HashMap::new();
    let mut allergen_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    // parse
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        let s = line.unwrap().to_owned();
        let mut parts = s.trim_matches(')').split(" (contains ");
        let mut ingredients: Vec<String> = vec![];
        for ingredient in parts.next().unwrap().split(" ") {
            ingredients.push(ingredient.to_string());
        }
        let mut allergens: Vec<String> = vec![];
        for allergen in parts.next().unwrap().split(", ") {
            allergens.push(allergen.to_string());
        }
        for a in allergens {
            if !allergen_ingredients.contains_key(&a) {
                allergen_ingredients.insert(a, HashSet::from_iter(ingredients.to_vec()));
            } else {
                let other = HashSet::from_iter(ingredients.to_vec());
                let intersect = allergen_ingredients[&a].intersection(&other);
                let set = HashSet::from_iter(intersect.map(|x| x.to_string()));
                allergen_ingredients.insert(a, set);
            }
        }
        for i in ingredients {
            let val = ingredient_counts.get(&i).unwrap_or(&0) + 1;
            ingredient_counts.insert(i, val);
        }
    }
    // evaluate
    let mut mapping = HashMap::new();
    for _ in 0..allergen_ingredients.len() {
        for (allergen, ingredients) in &allergen_ingredients {
            let keys: Vec<String> = mapping.keys().cloned().collect();
            let removed: HashSet<String> = HashSet::from_iter(keys);
            let matched: HashSet<String> = ingredients.difference(&removed).cloned().collect();
            if matched.len() == 1 {
                let ingredient = matched.iter().next().unwrap();
                mapping.insert(ingredient.to_string(), allergen);
            }
        }
    }
    // sort
    let mut mapping_vect: Vec<(&String, &&String)> = mapping.iter().collect();
    mapping_vect.sort_by(|a, b| a.1.cmp(b.1));
    let ingredients: Vec<String> = mapping_vect.into_iter().map(|a| a.0).cloned().collect();
    println!("{}", ingredients.join(","));
}
