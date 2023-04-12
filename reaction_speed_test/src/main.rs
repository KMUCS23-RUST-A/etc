use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut total_time = Duration::new(0, 0);
    let mut num_rounds = 0;

    loop {
        println!("게임을 시작하려면 '시작'을 입력하세요");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

        if input.trim() != "시작" {
            println!("잘못된 입력입니다.");
            continue;
        }

        let delay = Duration::from_secs(rng.gen_range(1..5));
        println!("{}초 후에 단어가 나타납니다...", delay.as_secs());
        std::thread::sleep(delay);

        let word = generate_random_word();
        println!("단어: {}", word);

        let start_time = Instant::now();
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("입력을 읽을 수 없습니다.");
        let end_time = Instant::now();

        if user_input.trim() != word {
            println!("틀렸습니다!");
            continue;
        }

        let round_time = end_time.duration_since(start_time);
        println!("{}초 걸렸습니다!", round_time.as_secs());
        total_time += round_time;
        num_rounds += 1;

        let avg_time = total_time.as_secs_f32() / (num_rounds as f32);
        println!("평균 시간: {:.2}초", avg_time);
    }
}

fn generate_random_word() -> String {
    let words = vec![
        "apple", "banana", "cherry", "date", "elderberry",
        "fig", "grape", "honeydew", "kiwi", "lemon",
        "mango", "nectarine", "orange", "pear", "quince",
        "raspberry", "strawberry", "tangerine", "watermelon",
    ];
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index].to_owned()
}
