use rand::Rng;
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    println!("반응 속도 테스트를 시작합니다!");

    let mut rng = rand::thread_rng();
    let mut total_time = Duration::new(0, 0);
    let mut num_tests = 0;

    let colors = [
        "\x1b[0;31m", // 빨간색
        "\x1b[0;32m", // 초록색
        "\x1b[0;33m", // 노란색
        "\x1b[0;34m", // 파란색
        "\x1b[0;35m", // 자주색
        "\x1b[0;36m", // 청록색
        "\x1b[0;37m", // 회색
    ];

    let strings = ["빨간색", "초록색", "노란색", "파란색", "자주색", "청록색", "회색"];

    loop {
        let wait_time = Duration::from_secs(rng.gen_range(2..5));
        std::thread::sleep(wait_time);

        let color = colors[rng.gen_range(0..colors.len())];
        let string = strings[rng.gen_range(0..strings.len())];

        print!("{}{}{}\x1b[0m 문자열을 입력하세요: ", color, string, color);
        io::stdout().flush().unwrap();

        let start_time = Instant::now();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let end_time = Instant::now();

        let response_time = end_time - start_time;
        if input.trim() != string {
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
