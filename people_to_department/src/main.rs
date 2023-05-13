use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts: Vec<&str> = command.trim().split_whitespace().collect();

        match parts[0] {
            "Add" => {
                if parts.len() != 4 {
                    println!("Invalid command! Use: Add $Person to $Department");
                    continue;
                }
                let person = parts[1].to_string();
                let department = parts[3].to_string();

                departments.entry(department.clone()).or_insert(Vec::new()).push(person);
                departments.get_mut(&department).unwrap().sort();
            },
            "Remove" => {
                if parts.len() != 4 {
                    println!("Invalid command! Use: Remove $Person from $Department");
                    continue;
                }
                let person = parts[1].to_string();
                let department = parts[3].to_string();

                if let Some(employees) = departments.get_mut(&department) {
                    if let Some(index) = employees.iter().position(|x| *x == person) {
                        employees.remove(index);
                    } else {
                        println!("Person not found in the department!");
                    }
                } else {
                    println!("Department not found!");
                }
            },
            "List" => {
                let mut departments_sorted: Vec<_> = departments.iter().collect();
                departments_sorted.sort_by(|a, b| a.0.cmp(b.0));
                for (department, employees) in departments_sorted {
                    println!("Department: {}", department);
                    for employee in employees {
                        println!("  {}", employee);
                    }
                }
            },
            "Get" => {
                if parts.len() != 2 {
                    println!("Invalid command! Use: Get $Department");
                    continue;
                }
                let department = parts[1].to_string();

                if let Some(employees) = departments.get(&department) {
                    println!("Employees in {}: ", department);
                    for employee in employees {
                        println!("  {}", employee);
                    }
                } else {
                    println!("Department not found!");
                }
            },
            _ => println!("Invalid command!"),
        }
    }
}