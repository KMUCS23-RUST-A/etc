use rand::Rng;
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    println!("반응 속도 테스트를 시작합니다!");

    let mut rng = rand::thread_rng();
    let mut total_time = Duration::new(0, 0);
    let mut num_tests = 0;

    loop {
        let wait_time = Duration::from_secs(rng.gen_range(2..5));
        std::thread::sleep(wait_time);

        print!("빨간색 문자열을 입력하세요: ");
        io::stdout().flush().unwrap();

        let start_time = Instant::now();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let end_time = Instant::now();

        let response_time = end_time - start_time;
        if input.trim() != "빨간색" {
            println!("입력이 잘못되었습니다. 테스트를 종료합니다.");
            break;
        } else {
            total_time += response_time;
            num_tests += 1;
            println!("반응 속도: {}ms", response_time.as_millis());
        }
    }

    if num_tests > 0 {
        let avg_time = total_time / num_tests;
        println!("평균 반응 속도: {}ms", avg_time.as_millis());
    }
}