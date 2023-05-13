use std::env;

enum Currency {
    Knut,
    Sickle,
    Galleon,
}

fn convert_currency(amount: f64, from: Currency, to: Currency) -> f64 {
    let conversion_rates = [
        [1.0, 1.0 / 29.0, 1.0 / 493.0],
        [29.0, 1.0, 1.0 / 17.0],
        [493.0, 17.0, 1.0],
    ];

    let from_index = match from {
        Currency::Knut => 0,
        Currency::Sickle => 1,
        Currency::Galleon => 2,
    };

    let to_index = match to {
        Currency::Knut => 0,
        Currency::Sickle => 1,
        Currency::Galleon => 2,
    };

    amount * conversion_rates[from_index][to_index]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        println!("Usage: cargo run -- <amount> --from <currency> --to <currency>");
        return;
    }
    
    for arg in &args {
        println!("{}", arg);
    }

    let amount: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount provided");
            return;
        }
    };

    let from_currency = match args[3].as_str() {
        "knut" => Currency::Knut,
        "sickle" => Currency::Sickle,
        "galleon" => Currency::Galleon,
        _ => {
            println!("Invalid 'from' currency provided");
            return;
        }
    };

    let to_currency = match args[5].as_str() {
        "knut" => Currency::Knut,
        "sickle" => Currency::Sickle,
        "galleon" => Currency::Galleon,
        _ => {
            println!("Invalid 'to' currency provided");
            return;
        }
    };

    let converted_amount = convert_currency(amount, from_currency, to_currency);

    println!("{} {} -> {} {}", amount, args[3], converted_amount, args[5]);
}