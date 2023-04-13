# Dodge rain Game

Dodge rain Game은 ncurses와 Rust 언어를 이용하여 개발한 터미널 게임입니다. 

## 게임 방법

1. 터미널에서 cargo run 명령어를 입력하여 게임을 실행합니다.
2. 사용자는 왼쪽 화살표 ← 또는 오른쪽 화살표 → 키를 사용하여 "U" 문자를 좌우로 움직일 수 있습니다.
3. 비 "R" 문자가 하늘에서 떨어지는데, 사용자는 "U" 문자와 충돌하지 않도록 피해야 합니다.
4. "R" 문자와 충돌하지 않으면 1점을 얻습니다.
"R" 문자와 충돌하면 -1점을 얻습니다.
사용자는 10점 이하로 점수가 떨어지면 게임이 종료됩니다.

## 개선할 점

1. 사용자가 비에 맞으면 게임이 종료되는 기능 추가 필요.
2. 사용자의 입력이 없어도 비가 동적으로 떨어지는 기능 추가 필요.
3. 점수 반영 기준 강화.
4. 사용자의 예기치 않은 입력에 따른 에러 처리 필요.

## 빌드 및 실행 방법

먼저, Rust 언어를 설치해야 합니다. Rust 공식 홈페이지(https://www.rust-lang.org/tools/install)를 참고하여 설치하시기 바랍니다.

```sh
# 레포지토리를 로컬에 클론합니다.
git clone https://github.com/<username>/rainy-day-game.git

# 클론한 디렉토리로 이동합니다.
cd rainy-day-game

# 게임을 빌드합니다.
cargo build

# 게임을 실행합니다.
cargo run
```

## 코드 시연

![demo](https://user-images.githubusercontent.com/38421491/231823955-6e7bf01c-41bf-4acf-98f0-0cf0396443ba.gif)





