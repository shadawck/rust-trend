use core::panic;
use std::fmt;

#[derive(Debug, Clone)]
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

    pub fn as_str(&self) -> &'static str {
        self.lang
    }

    pub fn list() {
        println!("{:#?}", Self::SUPPORTED_LANG);
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lang)
    }
}
