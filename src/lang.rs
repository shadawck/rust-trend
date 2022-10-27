//! Represent all langage supported by google.   
//!
//! All langages available [here](https://github.com/shadawck/rust-trend/wiki/Langages)
use strum_macros::{Display, EnumString, EnumVariantNames};

/// Create a new langage.
///
/// Returns a Langage instance.
///
/// # Example
/// ```
/// # use rtrend::Lang;
/// // The returned lang will be set to italian
/// let lang = Lang::IT;
/// ```
#[derive(Eq, PartialEq, Display, Debug, EnumString, Clone, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum Lang {
    AF,
    AR,
    AZ,
    BE,
    BG,
    BN,
    CA,
    CS,
    CY,
    DA,
    DE,
    EL,
    EN,
    EO,
    ES,
    ET,
    EU,
    FA,
    FI,
    FR,
    GA,
    GL,
    GU,
    HI,
    HR,
    HT,
    HU,
    ID,
    IS,
    IT,
    IW,
    JA,
    KA,
    KN,
    KO,
    LA,
    LT,
    LV,
    MK,
    MS,
    MT,
    NL,
    NO,
    PL,
    PT,
    RO,
    RU,
    SK,
    SL,
    SQ,
    SR,
    SV,
    SW,
    TA,
    TE,
    TH,
    TL,
    TR,
    UK,
    UR,
    VI,
    YI,
    #[allow(non_camel_case_types)]
    #[strum(serialize = "zh-CN")]
    ZH_CN,
    #[allow(non_camel_case_types)]
    #[strum(serialize = "zh-TW")]
    ZH_TW,
}
