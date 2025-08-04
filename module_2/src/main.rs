fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut i = low;

    while i <= high {
        *total += i;
        i += step;
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    
    let mut max_count = 0;
    let mut max_word = "";

    let words: Vec<&str> = text.split_whitespace().collect();
    let mut count: Vec<&str> = Vec::new();
    let mut same_words: Vec<usize> = Vec::new();

    for word in &words {
        let mut frequent = false;

        for (i, w) in count.iter().enumerate(){
            if *word == *w {
                same_words[i] += 1;
                frequent = true;

                if same_words[i] > max_count {
                    max_count = same_words[i];
                    max_word = *word;
                }
                break;
            }
        }
   

        if !frequent {
            count.push(*word);
            same_words.push(1);

            if max_count < 1 {
                max_count = 1;
                max_word = *word;
            }
        }
    

    }    
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    
    // Sum with Step (Problem)
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

    println!("");

    // Most Frequent Word (Problem)
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}