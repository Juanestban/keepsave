//! Fluent translations. Every user-visible string goes through [`I18n::tr`].

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

const ENGLISH_FTL: &str = include_str!("../locales/en.ftl");
const SPANISH_FTL: &str = include_str!("../locales/es.ftl");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
  English,
  Spanish,
}

impl Language {
  #[cfg(test)]
  pub const ALL: [Self; 2] = [Self::Spanish, Self::English];

  pub fn code(self) -> &'static str {
    match self {
      Self::English => "en",
      Self::Spanish => "es",
    }
  }

  /// Accepts bare codes and full locales like `es-CO` or `en_US`.
  pub fn from_code(code: &str) -> Option<Self> {
    let primary = code.split(['-', '_']).next()?.to_ascii_lowercase();
    match primary.as_str() {
      "en" => Some(Self::English),
      "es" => Some(Self::Spanish),
      _ => None,
    }
  }

  /// The OS language when we support it, English otherwise.
  pub fn detect() -> Self {
    sys_locale::get_locale().and_then(|locale| Self::from_code(&locale)).unwrap_or(Self::English)
  }

  fn source(self) -> &'static str {
    match self {
      Self::English => ENGLISH_FTL,
      Self::Spanish => SPANISH_FTL,
    }
  }
}

pub struct I18n {
  language: Language,
  bundle: FluentBundle<FluentResource>,
  /// English, for keys missing from the active language.
  fallback: FluentBundle<FluentResource>,
}

impl I18n {
  pub fn new(language: Language) -> Self {
    Self { language, bundle: bundle_for(language), fallback: bundle_for(Language::English) }
  }

  pub fn language(&self) -> Language {
    self.language
  }

  pub fn tr(&self, key: &str) -> String {
    self.format(key, None)
  }

  /// Shorthand for a message with a single argument.
  pub fn tr_arg<'a>(&self, key: &str, name: &'a str, value: impl Into<FluentValue<'a>>) -> String {
    let mut args = FluentArgs::new();
    args.set(name, value);
    self.tr_args(key, &args)
  }

  pub fn tr_args(&self, key: &str, args: &FluentArgs) -> String {
    self.format(key, Some(args))
  }

  fn format(&self, key: &str, args: Option<&FluentArgs>) -> String {
    for bundle in [&self.bundle, &self.fallback] {
      if let Some(pattern) = bundle.get_message(key).and_then(|message| message.value()) {
        let mut errors = Vec::new();
        return bundle.format_pattern(pattern, args, &mut errors).into_owned();
      }
    }
    // A visible key is easier to spot and fix than an empty label.
    key.to_owned()
  }
}

fn bundle_for(language: Language) -> FluentBundle<FluentResource> {
  let id: LanguageIdentifier = language.code().parse().expect("valid language code");
  let resource = FluentResource::try_new(language.source().to_owned()).expect("bundled .ftl parses");
  let mut bundle = FluentBundle::new(vec![id]);
  // egui renders the Unicode isolation marks as boxes.
  bundle.set_use_isolating(false);
  bundle.add_resource(resource).expect("bundled .ftl has no duplicate keys");
  bundle
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeSet;

  use super::*;

  fn keys(source: &str) -> BTreeSet<&str> {
    source
      .lines()
      .filter(|line| line.starts_with(|c: char| c.is_ascii_lowercase()))
      .filter_map(|line| line.split_once(" =").map(|(key, _)| key))
      .collect()
  }

  #[test]
  fn bundles_parse() {
    for language in Language::ALL {
      let i18n = I18n::new(language);
      assert_ne!(i18n.tr("app-name"), "app-name");
    }
  }

  #[test]
  fn both_languages_have_the_same_keys() {
    let english = keys(ENGLISH_FTL);
    let spanish = keys(SPANISH_FTL);
    assert!(!english.is_empty());
    assert_eq!(english.symmetric_difference(&spanish).collect::<Vec<_>>(), Vec::<&&str>::new());
  }

  #[test]
  fn arguments_are_interpolated_without_isolation_marks() {
    let mut args = FluentArgs::new();
    args.set("details", "boom");
    let text = I18n::new(Language::English).tr_args("error-generic", &args);
    assert!(text.contains("boom"));
    assert!(!text.contains('\u{2068}'));
  }

  #[test]
  fn locale_codes() {
    assert_eq!(Language::from_code("es-CO"), Some(Language::Spanish));
    assert_eq!(Language::from_code("en_US"), Some(Language::English));
    assert_eq!(Language::from_code("ES"), Some(Language::Spanish));
    assert_eq!(Language::from_code("fr-FR"), None);
  }

  #[test]
  fn missing_key_falls_back_to_the_key() {
    assert_eq!(I18n::new(Language::Spanish).tr("no-such-key"), "no-such-key");
  }
}
