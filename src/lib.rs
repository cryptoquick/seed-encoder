use std::collections::HashMap;

use once_cell::sync::Lazy;
use substring::Substring;

const ENGLISH: &str = include_str!("../words/english.txt");

static ENGLISH_NUM: Lazy<HashMap<String, u16>> = Lazy::new(|| {
    let mut num_map = HashMap::new();

    for (i, word) in ENGLISH.split_ascii_whitespace().enumerate() {
        num_map.insert(word.to_owned(), i as u16 + 1);
    }

    num_map
});

static NUM_ENGLISH: Lazy<HashMap<u16, String>> = Lazy::new(|| {
    let mut num_map = HashMap::new();

    for (i, word) in ENGLISH.split_ascii_whitespace().enumerate() {
        num_map.insert(i as u16 + 1, word.to_owned());
    }

    num_map
});

static ENGLISH_ALPHA: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut alpha_map = HashMap::new();

    for word in ENGLISH.split_ascii_whitespace() {
        alpha_map.insert(word.to_owned(), word.substring(0, 4).to_owned());
    }

    alpha_map
});

static ALPHA_ENGLISH: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut alpha_map = HashMap::new();

    for word in ENGLISH.split_ascii_whitespace() {
        alpha_map.insert(word.substring(0, 4).to_owned(), word.to_owned());
    }

    alpha_map
});

pub enum Plate {
    Alpha,
    Num,
    Unknown,
}

pub fn detect(input: &str) -> Plate {
    let words = input.split(' ');

    if !matches!(words.clone().count(), 12 | 18 | 24) {
        return Plate::Unknown;
    }

    let mut alphas = 0;
    let mut nums = 0;

    for word in words.clone() {
        if word.len() == 4 {
            alphas += 1;
        } else {
            break;
        }
    }

    if !matches!(alphas, 12 | 18 | 24) {
        for word in words {
            match word.parse::<u16>() {
                Ok(result) => {
                    if result <= 2048 {
                        nums += 1;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }
    }

    if matches!(alphas, 12 | 18 | 24) {
        Plate::Alpha
    } else if matches!(nums, 12 | 18 | 24) {
        Plate::Num
    } else {
        Plate::Unknown
    }
}

#[derive(Debug)]
pub enum Error {
    /// Word not found in BIP-39 wordlist when encoding words
    EncodeWordNotFound,
    /// Number not between 1-2048, which is the size of the BIP-39 wordlist
    DecodeNumNotFound,
    /// First 4 letters not found in BIP-39 wordlist when encoding words
    DecodeAlphaNotFound,
}

/// Take words from word list and return numbers of those words
pub fn encode_num(words: &str) -> Result<String, Error> {
    let mut nums = vec![];

    for word in words.split_ascii_whitespace() {
        let result = ENGLISH_NUM.get(word);

        if let Some(num) = result {
            nums.push(num.to_string());
        } else {
            return Err(Error::EncodeWordNotFound);
        }
    }

    Ok(nums.join(" "))
}

/// Take words from word list and return first 4 letters of those words
pub fn encode_alpha(words: &str) -> Result<String, Error> {
    let mut alphas = vec![];

    for word in words.split_ascii_whitespace() {
        let result = ENGLISH_ALPHA.get(word);

        if let Some(alpha) = result {
            alphas.push(alpha.to_string());
        } else {
            return Err(Error::EncodeWordNotFound);
        }
    }

    Ok(alphas.join(" "))
}

/// Take number of word from word list and return full word
pub fn decode_num(nums: &str) -> Result<String, Error> {
    let mut words = vec![];

    for num in nums.split_ascii_whitespace() {
        let result = NUM_ENGLISH.get(num);

        if let Some(word) = result {
            words.push(num.to_string());
        } else {
            return Err(Error::DecodeNumNotFound);
        }
    }

    Ok(words.join(" "))
}

/// Take first 4 letters from word and return full word
pub fn decode_alpha(alphas: &str) -> Result<String, Error> {
    let mut words = vec![];

    for alpha in alphas.split_ascii_whitespace() {
        let result = ALPHA_ENGLISH.get(alpha);

        if let Some(word) = result {
            words.push(word.to_string());
        } else {
            return Err(Error::DecodeAlphaNotFound);
        }
    }

    Ok(words.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORDS: &str = "evidence gate beef bright sample lounge flower culture strategy begin thought thumb start ask river olive joy pause purchase absorb mad jacket error elevator";

    const ALPHAS: &str = "evid gate beef brig samp loun flow cult stra begi thou thum star ask rive oliv joy paus purc abso mad jack erro elev";

    const NUMS: &str = "623 771 161 225 1529 1059 717 429 1719 163 1800 1804 1702 107 1495 1234 965 1292 1394 7 1070 953 615 576";

    #[test]
    fn encodes_nums() {
        let result = encode_num(WORDS).unwrap();
        assert_eq!(result, "623 771 161 225 1529 1059 717 429 1719 163 1800 1804 1702 107 1495 1234 965 1292 1394 7 1070 953 615 576");
    }

    #[test]
    fn encodes_alphas() {
        let result = encode_alpha(WORDS).unwrap();
        assert_eq!(result, "evid gate beef brig samp loun flow cult stra begi thou thum star ask rive oliv joy paus purc abso mad jack erro elev");
    }

    #[test]
    fn decodes_nums() {
        let result = decode_num(NUMS).unwrap();
        assert_eq!(result, "evidence gate beef bright sample lounge flower culture strategy begin thought thumb start ask river olive joy pause purchase absorb mad jacket error elevator");
    }

    #[test]
    fn decodes_alphas() {
        let result = decode_alpha(ALPHAS).unwrap();
        assert_eq!(result, "evidence gate beef bright sample lounge flower culture strategy begin thought thumb start ask river olive joy pause purchase absorb mad jacket error elevator");
    }
}
