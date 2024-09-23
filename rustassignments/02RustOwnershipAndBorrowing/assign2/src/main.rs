fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    //let word = "quick";

    let mut max_word: &str = "";
    let mut max_count = 0;
    let mut count = 0;

    for idx in 0..words.len(){
        let word: &str = words[idx];
        // reset counter
        count = 0;
        for jdx in 0..words.len(){
            if words[jdx] == word{
                count += 1;
            }
        }
        
        if count > max_count{
            max_count = count;
            max_word = word;
        }

        //println!("word: {} count:{}, current maximum:{}", word, count, max_count);
    }
    
    return(max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}