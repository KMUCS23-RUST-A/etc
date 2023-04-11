# Rust Hangman Game

Rust로 구현한 간단한 행맨 게임입니다. 

## 게임 규칙

-  게임이 시작되면, 컴퓨터가 무작위로 단어를 선택합니다.
-  사용자는 한 글자씩 추측하여 단어를 맞춰야 합니다.
-  사용자가 추측한 글자가 단어에 포함되어 있다면, 해당 글자가 단어의 위치에 표시됩니다.
-  사용자가 추측한 글자가 단어에 포함되어 있지 않다면, 행맨 그림에 한 부분이 추가됩니다.
-  사용자가 단어를 맞추거나, 행맨 그림이 완성되면 게임이 종료됩니다.

## 실행 방법

1. Rust를 설치합니다. [Rust 홈페이지](https://www.rust-lang.org/tools/install)에서 설치 방법을 확인할 수 있습니다.
2. 터미널에서 다음 명령어를 실행하여 게임을 실행합니다.

```
cargo run
```

## 게임 화면

```
Guess a letter:
a
Guessed so far: a__l_

  +---+
  |   |
      |
      |
      |
      |
=========

Guess a letter:
e
Guessed so far: a__le

  +---+
  |   |
      |
      |
      |
      |
=========

Guess a letter:
i
Guessed so far: a_ile

  +---+
  |   |
  O   |
      |
      |
      |
=========

Guess a letter:
u
Guessed so far: a_ile

  +---+
  |   |
  O   |
 /    |
      |
      |
=========

Guess a letter:
s
Guessed so far: a_isle

  +---+
  |   |
  O   |
 /    |
      |
      |
=========

Guess a letter:
r
Guessed so far: a_isle

  +---+
  |   |
  O   |
 / \  |
      |
      |
=========

You lose! The word was "aisle".

  +---+
  |   |
  O   |
 / \  |
      |
      |
=========
```

## 
