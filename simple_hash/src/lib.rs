use std::collections::HashMap;
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut words_map = HashMap::new();
    for word in words {
        *words_map.entry(word).or_insert(0) += 1;
    }
    words_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut result = 0;
    let checker = 1usize;
    for (_, value) in frequency_count {
        if value == &checker {
            result += 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests".to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();

        let frequency_count = word_frequency_counter(words);
        println!("{:?}", frequency_count);
        println!("{}", nb_distinct_words(&frequency_count));
    }
}
