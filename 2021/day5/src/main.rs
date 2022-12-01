use std::collections::HashMap;

fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    match gunners.values().all(|&g| g == "aye"){
        true => "Fire!".to_string(),
        false => "Shiver me timbers!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::cannons_ready;
    use rand::Rng;
    use std::collections::HashMap;

    const READY_PROBABILITY: f64 = 0.25;
    const HAS_RUNNER_PROBABILITY: f64 = 0.5;

    fn solution(gunners: HashMap<&str, &str>) -> String {
        match gunners.values().filter(|&g| g == &"aye").count() == gunners.len() {
            true => "Fire!".to_string(),
            false => "Shiver me timbers!".to_string(),
        }
    }

    #[test]
    fn should_fire_the_cannons_when_ready() {
        assert_eq!(
            cannons_ready(HashMap::from([
                ("Mike", "aye"),
                ("Joe", "aye"),
                ("Johnson", "aye"),
                ("Peter", "aye"),
            ])),
            "Fire!"
        );
    }

    #[test]
    fn should_shiver_your_timbers_instead_if_not_ready() {
        assert_eq!(
            cannons_ready(HashMap::from([
                ("Mike", "aye"),
                ("Joe", "nay"),
                ("Johnson", "aye"),
                ("Peter", "aye"),
            ])),
            "Shiver me timbers!"
        );
    }

    #[test]
    fn should_give_the_correct_response_to_random_inputs() {
        const TESTS: u8 = 100;

        let mut rng = rand::thread_rng();
        for _ in 0..=TESTS {
            let mut gunners = HashMap::new();

            add_runner(&mut rng, &mut gunners, "Peter");
            add_runner(&mut rng, &mut gunners, "Mike");
            add_runner(&mut rng, &mut gunners, "Joe");
            add_runner(&mut rng, &mut gunners, "Jonson");

            let expected = solution(gunners.clone());
            let actual = cannons_ready(gunners.clone());

            assert_eq!(
                actual,
                expected,
                "Got \"{}\", but expected \"{}\" for the following input: {:?}",
                actual,
                expected,
                gunners.clone(),
            );
        }
    }

    fn add_runner<'a>(rng: &'a mut rand::rngs::ThreadRng, gunners: &'a mut HashMap<&str, &str>, name: &'a str) {
        if rng.gen_bool(HAS_RUNNER_PROBABILITY) {
            if rng.gen_bool(READY_PROBABILITY) {
                gunners.insert(name, "nay");
            } else {
                gunners.insert(name, "aye");
            }
        }
    }
}

fn main() {}