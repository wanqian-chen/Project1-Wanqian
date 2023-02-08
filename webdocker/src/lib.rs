/* A library that returns back random fruit */

use rand::Rng;

// create a const array of 10 fruits
pub const FRUITS: [&str; 10] = [
    "apple",
    "banana",
    "cherry",
    "grape",
    "lemon",
    "orange",
    "pear",
    "pineapple",
    "strawberry",
    "watermelon",
];

// a public function that returns a random fruit
pub fn random_fruit() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FRUITS.len());
    FRUITS[random_index].to_string()
}
