use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ascii::AsciiExt;

extern crate rand;

use rand::distributions::{IndependentSample, Range};

fn load_words() -> Result<Vec<String>, io::Error> {
  let f = try!(File::open("/usr/share/dict/words"));
  let file = BufReader::new(&f);
  let mut words: Vec<String> = Vec::new();

  for line in file.lines() {
    let l = line.unwrap();
    let linebytes = l.chars();
    if l.chars().count() == 7 {
      let mut all_lowercase = true;
      for c in linebytes {
        if !c.is_lowercase() || !c.is_ascii() {
          all_lowercase = false
        }
      }
      if all_lowercase {
        words.push(l.clone());
      }
    }
  }

  Ok(words)
}

fn main() {
  let words = match load_words() {
    Ok(w) => w,
    Err(e) => {
      println!("error loading words: {}", e);
      std::process::exit(1);
    },
  };
  
  let numwords = words.len();
  let between = Range::new(0, numwords);
  
  let mut rng = rand::thread_rng();
  let master = &words[between.ind_sample(&mut rng)];
  
  let mut guessed: [char; 7] = ['-'; 7];
  let mut lives = 9;
  
  while lives > 0 {
    let guess_str: String = guessed.iter().cloned().collect();
    println!("current guess: {}", guess_str);
    let mut this_guess = String::new();
    println!("guess a letter or the complete word:");
    io::stdin().read_line(&mut this_guess)
        .expect("Failed to read line");
    let this_guess = this_guess.trim();
    if this_guess.len() == 0 {
      println!("resigning");
      std::process::exit(0);
    } else  if this_guess.len() == 1 {
      let guess_char = this_guess.chars().nth(0).unwrap();
      let mut already_guessed = false;
      for x in 0..7 {
        if guessed[x] == guess_char {
          already_guessed = true;
        }
      }
      if already_guessed {
        lives = lives - 1;
        println!("already guessed {}, lives now {}!", guess_char, lives);
      } else {
        let mut good_guess = false;
        for x in 0..7 {
          if guess_char == master.chars().nth(x).unwrap() {
            guessed[x] = guess_char;
            good_guess = true;
          }
        }
        if good_guess {
          println!("guessed a letter, wahey!");
          let updated_guess: String = guessed.iter().cloned().collect();
          if &updated_guess == master {
            println!("you won! (word is {})", master);
            std::process::exit(0);
          }
        } else {
          lives = lives - 1;
          println!("letter isn't in the word, lives now {}!", lives);
        }
      }
    } else if this_guess.len() == 7 {
      if this_guess == master {
        println!("you guessed the word! ({})", master);
        std::process::exit(0);
      } else {
        println!("nope, that's not right!");
        lives = lives - 1;
      }
    } else {
      println!("that wasn't a proper move. Enter 1 or 7 chars.");
    }
  }
  
  println!("end of game, you lost. The word was {}.", master);
}
