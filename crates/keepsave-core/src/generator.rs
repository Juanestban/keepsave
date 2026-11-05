//! Password and passphrase generation backed by the OS random source.

use std::ops::RangeInclusive;

use rand::{RngExt, rand_core::UnwrapErr, rngs::SysRng, seq::SliceRandom};
use secrecy::SecretString;
use zeroize::Zeroizing;

pub const LENGTH_RANGE: RangeInclusive<usize> = 8..=128;
pub const WORDS_RANGE: RangeInclusive<usize> = 3..=20;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>/?~";
const AMBIGUOUS: &str = "0O1lI";

/// EFF large wordlist (7776 words), CC BY 3.0, <https://www.eff.org/dice>.
const WORDLIST: &str = include_str!("../assets/eff_large_wordlist.txt");

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GeneratorError {
  #[error("length must be between {} and {}", LENGTH_RANGE.start(), LENGTH_RANGE.end())]
  InvalidLength,
  #[error("word count must be between {} and {}", WORDS_RANGE.start(), WORDS_RANGE.end())]
  InvalidWordCount,
  #[error("select at least one character set")]
  NoCharacterSets,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharsetOptions {
  pub length: usize,
  pub lowercase: bool,
  pub uppercase: bool,
  pub digits: bool,
  pub symbols: bool,
  pub exclude_ambiguous: bool,
}

impl Default for CharsetOptions {
  fn default() -> Self {
    Self { length: 20, lowercase: true, uppercase: true, digits: true, symbols: true, exclude_ambiguous: false }
  }
}

impl CharsetOptions {
  /// The enabled classes, each already filtered for ambiguous characters.
  fn classes(&self) -> Vec<Vec<char>> {
    [(self.lowercase, LOWERCASE), (self.uppercase, UPPERCASE), (self.digits, DIGITS), (self.symbols, SYMBOLS)]
      .into_iter()
      .filter(|(enabled, _)| *enabled)
      .map(|(_, set)| set.chars().filter(|c| !(self.exclude_ambiguous && AMBIGUOUS.contains(*c))).collect())
      .collect()
  }

  /// Entropy in bits of a password drawn uniformly from the enabled pool.
  pub fn entropy_bits(&self) -> f64 {
    let pool: usize = self.classes().iter().map(Vec::len).sum();
    if pool == 0 {
      return 0.0;
    }
    self.length as f64 * (pool as f64).log2()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassphraseOptions {
  pub words: usize,
  pub separator: String,
}

impl Default for PassphraseOptions {
  fn default() -> Self {
    Self { words: 5, separator: "-".into() }
  }
}

impl PassphraseOptions {
  pub fn entropy_bits(&self) -> f64 {
    self.words as f64 * (wordlist().len() as f64).log2()
  }
}

/// Generates a password with at least one character from every enabled class.
pub fn generate_password(options: &CharsetOptions) -> Result<SecretString, GeneratorError> {
  if !LENGTH_RANGE.contains(&options.length) {
    return Err(GeneratorError::InvalidLength);
  }
  let classes = options.classes();
  if classes.is_empty() {
    return Err(GeneratorError::NoCharacterSets);
  }

  let mut rng = UnwrapErr(SysRng);
  let pool: Vec<char> = classes.concat();
  let mut chars: Zeroizing<Vec<char>> = Zeroizing::new(Vec::with_capacity(options.length));

  for class in &classes {
    chars.push(class[rng.random_range(0..class.len())]);
  }
  while chars.len() < options.length {
    chars.push(pool[rng.random_range(0..pool.len())]);
  }
  chars.shuffle(&mut rng);

  Ok(SecretString::from(chars.iter().collect::<String>()))
}

pub fn generate_passphrase(options: &PassphraseOptions) -> Result<SecretString, GeneratorError> {
  if !WORDS_RANGE.contains(&options.words) {
    return Err(GeneratorError::InvalidWordCount);
  }
  let words = wordlist();
  let mut rng = UnwrapErr(SysRng);
  let picked: Vec<&str> = (0..options.words).map(|_| words[rng.random_range(0..words.len())]).collect();
  Ok(SecretString::from(picked.join(&options.separator)))
}

/// Rough strength of a user-typed password: its length times log2 of the pool
/// implied by the character classes it uses. Optimistic for dictionary words,
/// so treat it as an upper bound for a strength meter, not a guarantee.
pub fn estimate_entropy_bits(password: &str) -> f64 {
  let (mut lower, mut upper, mut digit, mut symbol, mut other) = (false, false, false, false, false);
  for c in password.chars() {
    match c {
      'a'..='z' => lower = true,
      'A'..='Z' => upper = true,
      '0'..='9' => digit = true,
      c if SYMBOLS.contains(c) || c == ' ' => symbol = true,
      _ => other = true,
    }
  }
  let pool = [
    (lower, LOWERCASE.len()),
    (upper, UPPERCASE.len()),
    (digit, DIGITS.len()),
    (symbol, SYMBOLS.len() + 1),
    // Accented letters and other Unicode: a conservative guess.
    (other, 64),
  ]
  .into_iter()
  .filter(|(used, _)| *used)
  .map(|(_, size)| size)
  .sum::<usize>();
  if pool == 0 {
    return 0.0;
  }
  password.chars().count() as f64 * (pool as f64).log2()
}

fn wordlist() -> Vec<&'static str> {
  WORDLIST.lines().filter(|line| !line.is_empty()).collect()
}

#[cfg(test)]
mod tests {
  use secrecy::ExposeSecret;

  use super::*;

  #[test]
  fn password_has_requested_length_and_every_class() {
    let options = CharsetOptions::default();
    for _ in 0..200 {
      let password = generate_password(&options).unwrap();
      let password = password.expose_secret();
      assert_eq!(password.chars().count(), options.length);
      for set in [LOWERCASE, UPPERCASE, DIGITS, SYMBOLS] {
        assert!(password.chars().any(|c| set.contains(c)), "{set} missing");
      }
    }
  }

  #[test]
  fn exclude_ambiguous_removes_them() {
    let options = CharsetOptions { length: 128, exclude_ambiguous: true, ..CharsetOptions::default() };
    for _ in 0..50 {
      let password = generate_password(&options).unwrap();
      assert!(!password.expose_secret().chars().any(|c| AMBIGUOUS.contains(c)));
    }
  }

  #[test]
  fn rejects_invalid_options() {
    let too_short = CharsetOptions { length: 4, ..CharsetOptions::default() };
    assert_eq!(generate_password(&too_short).unwrap_err(), GeneratorError::InvalidLength);

    let empty = CharsetOptions { lowercase: false, uppercase: false, digits: false, symbols: false, ..CharsetOptions::default() };
    assert_eq!(generate_password(&empty).unwrap_err(), GeneratorError::NoCharacterSets);

    let no_words = PassphraseOptions { words: 1, ..PassphraseOptions::default() };
    assert_eq!(generate_passphrase(&no_words).unwrap_err(), GeneratorError::InvalidWordCount);
  }

  #[test]
  fn passphrase_uses_wordlist_and_separator() {
    // Not the default `-`: a few EFF words are hyphenated, like `t-shirt`.
    let options = PassphraseOptions { separator: " ".into(), ..PassphraseOptions::default() };
    let phrase = generate_passphrase(&options).unwrap();
    let words: Vec<&str> = phrase.expose_secret().split(' ').collect();
    assert_eq!(words.len(), 5);
    let list = wordlist();
    assert!(words.iter().all(|word| list.contains(word)));
  }

  #[test]
  fn entropy_matches_pool_size() {
    assert_eq!(wordlist().len(), 7776);
    let digits_only = CharsetOptions { length: 10, lowercase: false, uppercase: false, symbols: false, ..CharsetOptions::default() };
    assert!((digits_only.entropy_bits() - 10.0 * 10f64.log2()).abs() < 1e-9);
    let passphrase = PassphraseOptions::default().entropy_bits();
    assert!((passphrase - 5.0 * 7776f64.log2()).abs() < 1e-9);
  }

  #[test]
  fn estimate_grows_with_length_and_classes() {
    assert_eq!(estimate_entropy_bits(""), 0.0);
    assert!((estimate_entropy_bits("1234") - 4.0 * 10f64.log2()).abs() < 1e-9);
    let lower = estimate_entropy_bits("abcdefgh");
    let mixed = estimate_entropy_bits("abcdEFG1");
    assert!(mixed > lower);
    assert!(estimate_entropy_bits("abcdefghijkl") > lower);
  }
}
