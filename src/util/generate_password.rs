use crate::ui::viz::VGeneratePassword;
use rand::{RngCore, seq::SliceRandom};
use std::cmp::max;

// returns a generated password according to the given configuration;
// returns an empty string if the configuration is invalid
pub fn generate_password(config: &VGeneratePassword) -> String {
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBER_OF_LETTERS: usize = 26;
    const DIGITS: &str = "0123456789";
    const NUMBER_OF_DIGITS: usize = 10;

    let mut rng = rand::rng();
    let pw_length: usize = config.length.parse().unwrap_or(15);

    let mut num_special: usize = if config.include_special {
        max(pw_length / 6, 1)
    } else {
        0
    };
    let mut num_digit: usize = if config.include_numbers {
        pw_length / 4
    } else {
        0
    };

    let remainder = pw_length - num_special - num_digit;
    let (num_uppercase, num_lowercase) = match (config.include_lowercase, config.include_uppercase)
    {
        (true, true) => (remainder / 2, remainder - (remainder / 2)),
        (false, true) => (remainder, 0),
        (true, false) => (0, remainder),
        (false, false) => {
            match (config.include_numbers, config.include_special) {
                (true, true) => {
                    num_digit = pw_length / 2;
                    num_special = pw_length - num_digit;
                    (0, 0)
                }
                (true, false) => {
                    num_digit = pw_length;
                    (0, 0)
                }
                (false, true) => {
                    num_special = pw_length;
                    (0, 0)
                }
                (false, false) => {
                    return String::new(); // invalid configuration
                }
            }
        }
    };

    let mut result = String::with_capacity(pw_length * 4);
    for _ in 0..num_lowercase {
        let idx = rng.next_u32() as usize % NUMBER_OF_LETTERS;
        result.push(LOWERCASE.chars().nth(idx).unwrap());
    }
    for _ in 0..num_uppercase {
        let idx = rng.next_u32() as usize % NUMBER_OF_LETTERS;
        result.push(UPPERCASE.chars().nth(idx).unwrap());
    }
    for _ in 0..num_digit {
        let idx = rng.next_u32() as usize % NUMBER_OF_DIGITS;
        result.push(DIGITS.chars().nth(idx).unwrap());
    }
    for _ in 0..num_special {
        let idx = rng.next_u32() as usize % config.specials.chars().count();
        result.push(config.specials.chars().nth(idx).unwrap());
    }

    // Shuffle the result to mix characters
    let mut chars: Vec<char> = result.chars().collect();
    chars.shuffle(&mut rng);
    chars.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::{VGeneratePassword, generate_password};

    #[test]
    fn test_generate_password() {
        let mut config = VGeneratePassword::default();
        test_config(&config);

        for i in 6..=20 {
            config.length = i.to_string();
            test_config(&config);
        }

        config.length = 19.to_string();
        config.specials = "ÄÖÜäöü".to_string();
        test_config(&config);

        config.include_numbers = false;
        config.include_special = false;
        config.include_uppercase = false;
        test_config(&config);
    }

    fn test_config(config: &VGeneratePassword) {
        for _ in 0..100 {
            let pw = generate_password(config);
            assert_eq!(
                pw.chars().count().to_string(),
                config.length,
                "{pw} from {config:?}"
            );
            assert!(
                pw.chars().any(char::is_uppercase) == config.include_uppercase,
                "{pw} from {config:?}"
            );
            assert!(
                pw.chars().any(|c| c.is_ascii_digit()) == config.include_numbers,
                "{pw} from {config:?}"
            );
            assert!(
                pw.chars().any(|c| config.specials.contains(c)) == config.include_special,
                "{pw} from {config:?}"
            );
            println!("{pw}");
        }
    }
}
