use std::time::{Duration, Instant};

use ncurses::*;
use rand::Rng;

fn main() {
    // ncurses 초기화
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    // 화면 크기 설정
    let height = 20;
    let width = 40;
    resize_term(height, width);

    // 게임 변수 초기화
    let mut rng = rand::thread_rng();
    let mut score = 0;
    let mut bucket_pos = width / 2 - 1;
    let mut drop_pos = 1;
    let mut drop_col = rng.gen_range(0..width);
    let mut last_drop_time = Instant::now();

    loop {
        // 화면 지우기
        clear();

        // 바구니 그리기
        mvprintw(height - 1, bucket_pos - 1, "^^^");

        // 빗물 그리기
        let now = Instant::now();
        if now.duration_since(last_drop_time) >= Duration::from_millis(250) {
            last_drop_time = now;

            if drop_pos == height - 2 {
                if (bucket_pos - drop_col).abs() <= 1 {
                    score += 1;
                } else {
                    score -= 1;
                }
                drop_pos = 0;
                drop_col = rng.gen_range(0..width);
            } else {
                drop_pos += 1;
            }
        }
        mvprintw(drop_pos, drop_col, "*");

        // 스코어 표시
        mvprintw(0, 0, &format!("Score: {}", score));

        // 화면 업데이트
        refresh();

        // 사용자 입력 처리
        match getch() {
            KEY_LEFT => bucket_pos = bucket_pos.saturating_sub(1),
            KEY_RIGHT => bucket_pos = bucket_pos.saturating_add(1),
            _ => {}
        }

        // 바구니가 화면 바깥으로 벗어나지 않도록 처리
        bucket_pos = bucket_pos.max(1).min(width - 3);

        // 게임 오버 조건
        if score < -10 {
            break;
        }
    }

    // ncurses 종료
    endwin();
}
