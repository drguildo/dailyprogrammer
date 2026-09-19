use std::collections::BTreeMap;

fn main() {
    let output_1 = calculate_price_change(
        "4
CarriageBolt 45
Eyebolt 50
Washer 120
Rivet 10
CarriageBolt 45
Eyebolt 45
Washer 140
Rivet 10",
    );
    println!("Sample Output 1");
    for (item, change) in &output_1 {
        print!("{} ", item);
        if change.is_positive() {
            println!("+{}", change);
        } else {
            println!("{}", change);
        }
    }

    println!();

    let output_2 = calculate_price_change(
        "3
2DNail 3
4DNail 5
8DNail 10
8DNail 11
4DNail 5
2DNail 2",
    );
    println!("Sample Output 2");
    for (item, change) in &output_2 {
        print!("{} ", item);
        if change.is_positive() {
            println!("+{}", change);
        } else {
            println!("{}", change);
        }
    }
}

fn calculate_price_change(price_list: &str) -> BTreeMap<String, i32> {
    let mut original_prices: BTreeMap<String, i32> = BTreeMap::new();
    let mut price_changes: BTreeMap<String, i32> = BTreeMap::new();

    let mut num_lines = price_list.lines().next().unwrap().parse::<usize>().unwrap();
    if num_lines == 0 {
        return price_changes;
    }
    for line in price_list.lines().skip(1) {
        let mut split_line = line.split_whitespace();
        let item_name: &str = split_line.next().unwrap();
        let item_price: i32 = split_line.next().unwrap().parse().unwrap();

        if num_lines > 0 {
            original_prices.insert(item_name.to_string(), item_price);
            num_lines -= 1;
        } else {
            if let Some(original_price) = original_prices.get(item_name) {
                if *original_price != item_price {
                    price_changes.insert(item_name.to_string(), item_price - original_price);
                }
            }
        }
    }

    price_changes
}
