fn main() {
    /*
     * Convert strings to pig latin. The first consonant of each word is moved to the end of the word 
     * and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the 
     * end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
     */
    let example = String::from("Hey! This sentence will be converted into a pig latin phrase! Здравствуйте");
    let mut pig_latin = String::new();

    for word in example.split_whitespace()
    {
        let ch = word.chars().next().unwrap();
        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
        {
            pig_latin.push_str(word);
            pig_latin.push_str("-hay ");
        }
        else
        {
            for (i,word_ch) in word.chars().enumerate()
            {
                if i != 0
                {
                    pig_latin.push_str(&word_ch.to_string());
                }
            }
            pig_latin.push_str("-fay ");
        }
    }
    println!("Original: {example}");
    println!("Pig latin: {pig_latin}");
}
