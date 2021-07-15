use core::panic;
use std::fmt;

/// Represent all angage supported by google
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Lang {
    lang: &'static str,
}

impl Lang {
    const SUPPORTED_LANG: &'static [&'static str] = &[
        "af", "ga", "sq", "it", "ar", "ja", "az", "kn", "eu", "ko", "bn", "la", "be", "lv", "bg",
        "lt", "ca", "mk", "zh-CN", "ms", "zh-TW", "mt", "hr", "no", "cs", "fa", "da", "pl", "nl",
        "pt", "en", "ro", "eo", "ru", "et", "sr", "tl", "sk", "fi", "sl", "fr", "es", "gl", "sw",
        "ka", "sv", "de", "ta", "el", "te", "gu", "th", "ht", "tr", "iw", "uk", "hi", "ur", "hu",
        "vi", "is", "cy", "id", "yi",
    ];

    /// Create a new langage.
    ///
    /// For the moment, langage need to be set in lowercase.
    ///
    /// Returns a Langage instance.
    ///
    /// # Example
    /// ```
    /// # use rtrend::Lang;
    /// // The returned lang will be set to italian
    /// let lang = Lang::new("it");
    /// ```
    ///
    /// # Panics
    /// An unsupported langage will panic.
    /// ```should_panic
    /// # use rtrend::Lang;
    /// let lang = Lang::new("zc");
    /// ```
    pub fn new(lang: &'static str) -> Lang {
        Lang {
            lang: Self::check_lang(lang),
        }
    }

    fn check_lang(lang: &'static str) -> &'static str {
        match Self::SUPPORTED_LANG.contains(&lang) {
            true => lang,
            false => panic!("Unsupported langage !"),
        }
    }

    /// List supported langage
    pub fn as_str(&self) -> &'static str {
        self.lang
    }

    /// Convert country to &str
    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_LANG);
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lang)
    }
}
