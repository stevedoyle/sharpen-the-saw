// This problem was asked by Facebook.
//
// Mastermind is a two-player game in which the first player attempts to guess the secret code of
// the second. In this version, the code may be any six-digit number with all distinct digits.
//
// Each turn the first player guesses some number, and the second player responds by saying how many
// digits in this number correctly matched their location in the secret code. For example, if the
// secret code were 123456, then a guess of 175286 would score two, since 1 and 6 were correctly
// placed.
//
// Write an algorithm which, given a sequence of guesses and their scores, determines whether there
// exists some secret code that could have produced them.
//
// For example, for the following scores you should return True, since they correspond to the secret
// code 123456:
//
// {175286: 2, 293416: 3, 654321: 0}
//
// However, it is impossible for any key to result in the following scores, so in this case you
// should return False:
//
// {123456: 4, 345678: 4, 567890: 4}

pub struct Guess {
    guess: u32,
    score: u32,
}

pub fn is_secret_found(guesses: &[Guess]) -> bool {
    for secret in 100_000..1_000_000 {
        if validate_secret(secret, guesses) {
            return true;
        }
    }
    false
}

fn validate_secret(secret: u32, guesses: &[Guess]) -> bool {
    for guess in guesses {
        let mut match_count = 0;
        let mut secret_copy = secret;
        let mut guess_copy = guess.guess;
        for _ in 0..6 {
            let secret_digit = secret_copy % 10;
            let guess_digit = guess_copy % 10;
            if secret_digit == guess_digit {
                match_count += 1;
            }
            secret_copy /= 10;
            guess_copy /= 10;
        }
        if match_count != guess.score {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_found() {
        // {175286: 2, 293416: 3, 654321: 0}
        let guesses = vec![
            Guess {
                guess: 175286,
                score: 2,
            },
            Guess {
                guess: 293416,
                score: 3,
            },
            Guess {
                guess: 654321,
                score: 0,
            },
        ];
        let result = is_secret_found(&guesses);
        assert_eq!(result, true);
    }

    #[test]
    fn secret_not_found() {
        // {123456: 4, 345678: 4, 567890: 4}
        let guesses = vec![
            Guess {
                guess: 123456,
                score: 4,
            },
            Guess {
                guess: 345678,
                score: 4,
            },
            Guess {
                guess: 567890,
                score: 4,
            },
        ];
        let result = is_secret_found(&guesses);
        assert_eq!(result, false);
    }
}
