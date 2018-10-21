// ⚡ Pure Rust-tool to brute-force short Lisk addresses.
//
// (c) 2018 by 4fryn <rust@4fry.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate bip39;
extern crate clap;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate timeago;
extern crate ethereum_types;

use std::str;
use std::u64;
use std::thread;
use std::time::Duration;
use bip39::{Language, Mnemonic, MnemonicType};
use clap::{Arg, App};
use chrono::Utc;
use crypto::ed25519;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rand::Rng;
use ethereum_types::H256;

#[derive(Default)]
pub struct Config {
  pub num_threads: usize,
  pub num_length: usize,
  pub mode_fast: bool,
}

// Main entry point
fn main() {
  let mut child_threads = vec![];

  let config = parse_config();
  let l = config.num_length;
  let x = config.mode_fast;

  for i in 0..config.num_threads {
    child_threads.push(thread::spawn(move || {
      brute_force(i, l, x);
    }));
  }

  for c in child_threads {
    let _ = c.join();
  }
}

// Setup help and parse CLI flags
fn parse_config() -> Config {
  let mut config = Config::default();

  let usage = App::new("lsk-shorty")
                      .version("0.0.4")
                      .author("4fryn <rust@4fry.net>")
                      .about("⚡ Pure rust-tool to brute-force short Lisk addresses.")
                      .arg(Arg::with_name("NUM_LENGTH")
                                        .short("l")
                                        .long("target")
                                        .help("Set the target address length you are looking for. (Default: 10)")
                                        .takes_value(true))
                      .arg(Arg::with_name("NUM_THREADS")
                                        .short("t")
                                        .long("threads")
                                        .help("Set the number of threads to generate addresses. (Default: 4)")
                                        .takes_value(true))
                      .arg(Arg::with_name("MODE_FAST")
                                        .short("x")
                                        .long("fast")
                                        .help("Enable fast mode by disabling generation of valid BIP-39 phrases."))
                      .get_matches();

  config.num_length = usage.value_of("NUM_LENGTH")
                              .unwrap_or("10")
                              .parse::<usize>()
                              .expect("NUM_LENGTH should be numeric.");
  config.num_threads = usage.value_of("NUM_THREADS")
                              .unwrap_or("4")
                              .parse::<usize>()
                              .expect("NUM_THREADS should be numeric.");
  config.mode_fast = usage.is_present("MODE_FAST");

  config
}

// Continuously looks for accounts with short addresses
fn brute_force(id: usize, n_target: usize, m_fast: bool) -> bool {

  // Gather some stats
  let mut target: usize = 22;
  let mut counter: u64 = 0;
  let start = Utc::now();

  // Brute-force random seeds until we find a very short one
  while target > n_target {
    counter += 1;
    let (length, phrase, address) = generate_new_account(!m_fast);

    // Only print short phrases to standard-out
    if length < target || length < 12 {
      target = length;
      let duration = Utc::now() - start;
      let elapsed: f64 = duration.num_seconds() as f64;
      let mut speed: f64 = counter as f64;
      speed = speed / elapsed;
      let (seconds, nanos) = calculate_probability_time(speed, target - 1, counter);
      let time_to_target = timeago::format_5chars(Duration::new(seconds, nanos));
      println!(
        "#{:?}\t*** FOUND TARGET {:?}; next target: {:?} in ~{:?}.\t{:?} iterations, {:.3}/s/t",
        id, target, target - 1, time_to_target, counter, speed
      );
      println!("\t{:?}\t{:?}L\t{:?}", length, address, phrase);
    }

    // Print regular progress updates
    if counter % 1_000_000 == 0 {
      let duration = Utc::now() - start;
      let elapsed: f64 = duration.num_seconds() as f64;
      let mut speed: f64 = counter as f64;
      speed = speed / elapsed;
      let (seconds, nanos) = calculate_probability_time(speed, target - 1, counter);
      let time_to_target = timeago::format_5chars(Duration::new(seconds, nanos));
      println!(
        "#{:?}\t\t... still working; next target: {:?} in ~{:?}.\t{:?} iterations, {:.3}/s/t",
        id, target - 1, time_to_target, counter, speed
      );
    }
  }
  println!("#{:?}\t\t ... shutting down thread #{:?}: final target found.", id, id);
  n_target >= target
}

// Calculate time of probability to find next target in seconds
fn calculate_probability_time(current_speed: f64, next_target: usize, current_iteration: u64) -> (u64, u32) {
  let mut probability: f64 = 10u32.pow(21u32 - next_target as u32).into();
  probability = probability - current_iteration as f64;
  let time_to_target = probability / current_speed;
  let seconds: u64 = time_to_target.trunc() as u64;
  let nanos: u32 = (time_to_target.fract() * 1_000_000f64).trunc() as u32;
  return (seconds, nanos);
}

// Generate new random account with or without BIP-39 seed
fn generate_new_account(bip39: bool) -> (usize, String, u64) {

  let phrase;
  let public_key;

  if bip39 {
    // > "When a user creates an account, a BIP39 mnemonics (the passphrase) is
    //    generated for the user."
    let mnemonic_type = MnemonicType::Type12Words;
    let language = Language::English;
    let mnemonic = match Mnemonic::new(mnemonic_type, language, "") {
      Ok(b) => b,
      Err(e) => {
        println!("e: {}", e);
        return (std::usize::MAX, "".to_string(), std::u64::MAX);
      }
    };
    phrase = mnemonic.get_string();

    // > "This passphrase is hashed using the SHA-256 hash function into a
    //    256-bits string."
    let mut seed = Sha256::new();
    seed.input_str(&phrase);
    let mut bytes = vec![0; seed.output_bytes()];
    seed.result(&mut bytes);

    // > "This hash is subsequently used as a seed in ed25519 to generate the
    //    private key and derives its public key."
    let (_priv, _publ) = ed25519::keypair(&bytes);
    public_key = H256(_publ);
  } else {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    let (_priv, _publ) = ed25519::keypair(&bytes);
    let _phrase = match str::from_utf8(&_priv) {
      Ok(k) => k,
      Err(e) => {
        panic!("e: {}", e);
      }
    };
    phrase = _phrase.to_string();
    public_key = H256(_publ);
  }

  // > "An address or the wallet ID is derived from the public key. The public
  //    key is hashed using SHA-256 then the first 8 bytes of the hash are
  //    reversed.
  let mut hash = Sha256::new();
  hash.input(&public_key);
  let reversed = [
      &hash.result_str()[14..16],
      &hash.result_str()[12..14],
      &hash.result_str()[10..12],
      &hash.result_str()[8..10],
      &hash.result_str()[6..8],
      &hash.result_str()[4..6],
      &hash.result_str()[2..4],
      &hash.result_str()[0..2],
  ].join("");

  // > "The account ID is the numerical representation of those 8 bytes,
  //    with the ’L’ character appended at the end.
  let numeric = match u64::from_str_radix(&reversed, 16) {
    Ok(n) => n,
    Err(e) => {
      panic!("e: {}", e);
    }
  };
  let length: usize = numeric.to_string().len() + 1;
  return (length, phrase, numeric);
}

#[test]
fn test_allways_succeed() {
  assert_eq!(1 + 1, 2);
}

#[test]
fn test_brute_force_shutdown() {
  assert!(brute_force(0, 18, true));
  assert!(brute_force(1, 20, false));
}

#[test]
fn test_probability_calculation() {
  let (seconds, nanos) = calculate_probability_time(133.7, 13, 1337);
  assert_eq!(seconds, 747933);
  assert_eq!(nanos, 156320);
}

#[test]
fn test_mnemonic_generic() {
  let entropy = &[0x33, 0xE4, 0x6B, 0xB1, 0x3A, 0x74, 0x6E, 0xA4, 0x1C, 0xDD, 0xE4, 0x5C, 0x90, 0x84, 0x6A, 0x79];
  let mnemonic = Mnemonic::from_entropy(entropy, MnemonicType::for_key_size(128).unwrap(), Language::English, "").unwrap();
  assert_eq!("crop cash unable insane eight faith inflict route frame loud box vibrant", mnemonic.as_str());
}

#[test]
fn test_mnemonic_generator() {
  let (length, phrase, _address) = generate_new_account(true);

  // should be 2 <= lenght <= 22
  assert!(length >= 2);
  assert!(length <= 22);

  // should be valid mnemonic
  let language = Language::English;
  let _mnemonic = match Mnemonic::from_string(phrase, language, "".to_string()) {
    Ok(_) => (),
    Err(e) => {
      panic!("{:?}", e);
    }
  };
}
