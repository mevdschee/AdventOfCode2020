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
    let other = allergen_ingredients
        .values()
        .fold(HashSet::new(), |a, x| a.into_iter().chain(x).collect());
    let set: HashSet<&String> = ingredient_counts.keys().collect();
    let intersect = set.difference(&other);
    let sum = intersect.fold(0, |a, x| a + ingredient_counts.get(x.to_owned()).unwrap());
    println!("{:?}", sum);
}
