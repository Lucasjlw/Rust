use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::{stdin, Write};

struct MapValue {
    url: String,
    count: usize,
}

struct Lookup {
    table: HashMap<String, MapValue>,
}

impl Lookup {
    fn new() -> Lookup {
        Lookup {
            table: HashMap::new(),
        }
    }

    fn get(&self, key: &String) -> Option<&MapValue> {
        self.table.get(key)
    }

    fn set(&mut self, key: String, value: MapValue) {
        self.table.insert(key, value);
    }

    fn len(&self) -> usize {
        self.table.len()
    }

    fn has(&self, key: &String) -> bool {
        self.table.contains_key(key)
    }

    fn increment(&mut self, key: &String) {
        if let Some(value) = self.table.get_mut(key) {
            value.count += 1;
        }
    }
}

pub fn main() {
    let mut lookup = Lookup::new();

    loop {
        print_menu();

        let menu_option: u8 = get_user_input()
            .parse()
            .expect("Input should be a number between 1 and 4");

        match menu_option {
            1 => handle_generate_shortcode(&mut lookup),
            2 => handle_lookup_shortcode(&mut lookup),
            3 => handle_check_statistics(&lookup),
            _ => return,
        }

        println!();
    }
}

fn print_menu() {
    println!("Select an option below:");
    println!("1. Generate a shortcode");
    println!("2. Lookup a shortcode");
    println!("3. Check statistics");
    println!("4. Exit");
    println!("Enter your choice: ");
}

fn handle_generate_shortcode(table: &mut Lookup) {
    let url = get_user_input();
    let mut shortcode = get_random_shortcode();

    while (table.has(&shortcode)) {
        shortcode = get_random_shortcode();
    }

    println!("Your shortcode is: {}", &shortcode);

    table.set(shortcode, MapValue { url, count: 0 });
}

fn handle_lookup_shortcode(table: &mut Lookup) {
    let shortcode = get_user_input();

    match table.get(&shortcode) {
        Some(val) => {
            println!("{}", val.url);
            table.increment(&shortcode);
        }
        None => println!("Shortcode does not exist"),
    }
}

fn handle_check_statistics(table: &Lookup) {
    let number_of_urls = table.len();
    let mut most_accessed_count: Option<usize> = None;
    let mut most_accessed_shortcode: Option<String> = None;

    for (shortcode, value) in table.table.iter() {
        if most_accessed_count.is_none() || value.count > most_accessed_count.unwrap() {
            most_accessed_count = Some(value.count);
            most_accessed_shortcode = Some(shortcode.clone());
        }
    }

    println!("We have {} urls", number_of_urls);
    println!(
        "{} is the most accessed shortcode",
        most_accessed_shortcode.unwrap()
    );
}

fn get_user_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Should be a proper input");

    input.trim().to_string()
}

fn get_random_shortcode() -> String {
    let mut rng = thread_rng();

    (0..10).map(|_| rng.sample(Alphanumeric) as char).collect()
}
