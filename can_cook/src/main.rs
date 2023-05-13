use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

fn main() {
    let mut stash: HashMap<String, i32> = HashMap::new();

    let mut recipes: HashMap<String, HashSet<String>> = HashMap::new();

    recipes.insert("Mushy Peas".to_string(), vec!["Peas"].iter().map(|&s| s.to_string()).collect());
    recipes.insert("Fish & Chips".to_string(), vec!["Fish", "Potato"].iter().map(|&s| s.to_string()).collect());
    recipes.insert("Mushroom Spinach Omelette".to_string(), vec!["Egg", "Mushroom", "Spinach", "Onion", "Cheese"].iter().map(|&s| s.to_string()).collect());
    recipes.insert("Mac & Cheese".to_string(), vec!["Cheese", "Pasta"].iter().map(|&s| s.to_string()).collect());

    loop {
        print!("What do you want to do? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split(' ').collect();

        match parts[0] {
            "Stash" => {
                for ingredient in parts.iter().skip(1) {
                    let count = stash.entry((*ingredient).to_string()).or_insert(0);
                    *count += 1;
                }
            },
            "Cook" => {
                let recipe_name = parts[1..].join(" ");
                if let Some(ingredients) = recipes.get(&recipe_name) {
                    let can_cook = ingredients.iter().all(|i| stash.get(i).unwrap_or(&0) >= &1);
                    if can_cook {
                        println!("You can cook {}!", recipe_name);
                        for ingredient in ingredients {
                            if let Some(count) = stash.get_mut(ingredient) {
                                *count -= 1;
                            }
                        }
                    } else {
                        println!("You can't cook {}! You're missing ingredients.", recipe_name);
                    }
                } else {
                    println!("Unknown recipe!");
                }
            },
            "Get" => {
                match parts[1] {
                    "Stash" => {
                        println!("Your stash contains:");
                        for (ingredient, count) in &stash {
                            if *count > 0 {
                                println!("{}: {}", ingredient, count);
                            }
                        }
                    },
                    "Recipe" => {
                        println!("Your recipes are:");
                        for (recipe, ingredients) in &recipes {
                            println!("{}: {:?}", recipe, ingredients);
                        }
                    },
                    _ => println!("Invalid command!"),
                }
            },
            _ => println!("Invalid command!"),
        }
    }
}