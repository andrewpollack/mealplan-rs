pub type Minutes = u8;

#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}

// TODO: Remove once functions implemented
#[allow(dead_code)]
#[derive(Debug)]
pub struct Meal {
    name: String,
    cuisine_type: String,
    ingredients: Vec<String>,
    difficulty: Difficulty,
    time: Minutes,
}

impl Meal {
    /// Most components of a `Meal` can be inferred by translating a string, except
    /// for `Meal.ingredients`. Although we could separate on commas, I felt more
    /// appropriate to pass in a HashMap `metadata` and a Vec<String> `ingredients`.
    /// I'm open for better ways of passing this information!
    pub fn new(metadata: std::collections::HashMap<String, String>, ingredients: Vec<String>) -> Self {
        Meal {
            name: metadata.get("name").unwrap().to_owned(),
            cuisine_type: metadata.get("cuisine_type").unwrap().to_owned(),
            ingredients,
            difficulty: Difficulty::Easy,
            time: metadata.get("time").unwrap().to_owned().parse::<u8>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::meal::{Difficulty, Meal};

    #[test]
    fn test_create_meal() {
        let freya_meal = Meal {
            name: "Freya's Turkey Sandwich".to_string(),
            cuisine_type: "Feline".to_string(),
            ingredients: vec!["homemade bread".to_string(), "turkey".to_string()],
            difficulty: Difficulty::Easy,
            time: 5,
        };

        assert_eq!(freya_meal.name, "Freya's Turkey Sandwich".to_string());
        assert_eq!(freya_meal.cuisine_type, "Feline".to_string());
        assert_eq!(
            freya_meal.ingredients,
            vec!["homemade bread".to_string(), "turkey".to_string()]
        );
        assert_eq!(freya_meal.difficulty, Difficulty::Easy);
        assert_eq!(freya_meal.time, 5);
    }

    #[test]
    fn test_update_meal() {
        let freya_meal = Meal {
            name: "Freya's Turkey Sandwich".to_string(),
            cuisine_type: "Feline".to_string(),
            ingredients: vec!["homemade bread".to_string(), "turkey".to_string()],
            difficulty: Difficulty::Easy,
            time: 5,
        };

        // Freya's meal now belongs to Andrew.
        let mut andrew_meal = freya_meal;
        andrew_meal.name = "Andrew's Turkey Sandwich".to_string();
        andrew_meal.cuisine_type = "American".to_string();
        andrew_meal.ingredients = vec![
            "homemade bread".to_string(),
            "turkey".to_string(),
            "mustard".to_string(),
        ];
        andrew_meal.difficulty = Difficulty::Medium;
        andrew_meal.time = 2;

        assert_eq!(andrew_meal.name, "Andrew's Turkey Sandwich".to_string());
        assert_eq!(andrew_meal.cuisine_type, "American".to_string());
        assert_eq!(
            andrew_meal.ingredients,
            vec![
                "homemade bread".to_string(),
                "turkey".to_string(),
                "mustard".to_string()
            ]
        );
        assert_eq!(andrew_meal.difficulty, Difficulty::Medium);
        assert_eq!(andrew_meal.time, 2);
    }

    // TODO: Add functions to read from files
}
