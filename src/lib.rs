use rand::Rng;

// a public hello function that takes a name and returns a greeting
pub fn hello(name: &str) -> String {
    // a list user names
    let names = vec!["Wanqian", "Alice", "Bob", "Carol"];

    // check if the name is in the list
    if names.contains(&name) {
        // if it is, return a greeting
        format!("Hello, {}!", name)
    } else {
        // if it isn't, return a warning
        format!("I don't know you!")
    }
}

// a public function that takes a list and returns a list without zeros
pub fn delete_zero(v: &mut Vec<i32>) -> Vec<i32> {
    // create a new list
    let mut new_v = Vec::new();

    // loop through the list
    for i in v {
        // if the element is not zero, add it to the new list
        if *i != 0 {
            new_v.push(*i);
        }
    }
    // return the new list
    new_v
}

// a public function that takes a probability and returns 1 or 0
pub fn coin(probability: f64) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    if random_number < probability {
        return 1;
    } else {
        return 0;
    }
}

// a public function that calculates the mean of a list
pub fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum as f64 / v.len() as f64
}

// a public function that calculates the median of a list
pub fn median(v: &Vec<i32>) -> f64 {
    let mut v = v.clone();
    v.sort();
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        (v[mid - 1] + v[mid]) as f64 / 2.0
    } else {
        v[mid] as f64
    }
}

// a public function that calculates the mode of a list
pub fn mode(v: &Vec<i32>) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;
    for (key, value) in counts {
        if value > max {
            max = value;
            mode = *key;
        }
    }
    mode
}

// a public function that calculates the variance of a list
pub fn variance(v: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    let mean = mean(v);
    for i in v {
        sum += (*i as f64 - mean).powi(2);
    }
    sum / v.len() as f64
}

// a public function that calculates the standard deviation of a list
pub fn std(v: &Vec<i32>) -> f64 {
    variance(v).sqrt()
}

// a public function that calculates chi-square of a 2x2 contingency table
pub fn chi_square(v: &Vec<i32>) -> f64 {
    let n = v[0] + v[1] + v[2] + v[3];
    let e1 = ((v[0] + v[1]) * (v[0] + v[2])) as f64 / n as f64;
    let e2 = ((v[0] + v[1]) * (v[1] + v[3])) as f64 / n as f64;
    let e3 = ((v[2] + v[3]) * (v[0] + v[2])) as f64 / n as f64;
    let e4 = ((v[2] + v[3]) * (v[1] + v[3])) as f64 / n as f64;

    ((v[0] as f64 - e1).powi(2) / e1
        + (v[1] as f64 - e2).powi(2) / e2
        + (v[2] as f64 - e3).powi(2) / e3
        + (v[3] as f64 - e4).powi(2) / e4)
        .sqrt()
}