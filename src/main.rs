use clap::Parser;
#[allow(unused_imports)]
use log::{ info, warn, error, debug, };
use mealplan_rs::meal::{Meal};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Verbose output
   #[arg(long, default_value_t = false)]
   verbose: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.verbose {
        info!("VERBOSE");
    }

    let example_meal_metadata = std::collections::HashMap::from([
        ("name".to_string(), "Freya's Turkey Sandwich".to_string()),
        ("cuisine_type".to_string(), "Feline".to_string()),
        ("time".to_string(), "5".to_string()),
    ]);
    let ingredients = vec!["homemade bread".to_string(), "turkey".to_string()];

    let my_meal = Meal::new(example_meal_metadata, ingredients);

    debug!("{:?}", my_meal)
}