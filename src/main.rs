struct WordCounter{
    text: String,

}
impl WordCounter {
fn new(text: &str) -> WordCounter{
    WordCounter {
        text: String::from(text),
    }
}
fn count_words(&self) -> usize{
    let words: Vec<&str> = self.text.split_whitespace().collect();
    words.len()
}
}
fn main() {
   println!("Enter a text please!");
   let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let word_counter = WordCounter::new(&input);
    let word_count = word_counter.count_words();
    println!("Word count is {}!", word_count);
}
