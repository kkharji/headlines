use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr,EnumString, EnumIter};

#[derive(AsRefStr, EnumString, Debug, Serialize, Deserialize, EnumIter, PartialEq, Clone, Copy, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Country {
    #[display(fmt = "United Arab Emirates")]
    AE,
    #[display(fmt = "Argentina")]
    AR,
    #[display(fmt = "Austria")]
    AT,
    #[display(fmt = "Australia")]
    AU,
    #[display(fmt = "Belgium")]
    BE,
    #[display(fmt = "Bulgaria")]
    BG,
    #[display(fmt = "Brazil")]
    BR,
    #[display(fmt = "Canada")]
    CA,
    #[display(fmt = "Switzerland")]
    CH,
    #[display(fmt = "China")]
    CN,
    #[display(fmt = "Colombia")]
    CO,
    #[display(fmt = "Cuba")]
    CU,
    #[display(fmt = "Czech Republic")]
    CZ,
    #[display(fmt = "Germany")]
    DE,
    #[display(fmt = "Egypt")]
    EG,
    #[display(fmt = "France")]
    FR,
    #[display(fmt = "United Kingdom")]
    GB,
    #[display(fmt = "Greece")]
    GR,
    #[display(fmt = "Hong Kong")]
    HK,
    #[display(fmt = "Hungary")]
    HU,
    #[display(fmt = "Indonesia")]
    ID,
    #[display(fmt = "Ireland")]
    IE,
    #[display(fmt = "India")]
    IN,
    #[display(fmt = "Italy")]
    IT,
    #[display(fmt = "Japan")]
    JP,
    #[display(fmt = "South Korea")]
    KR,
    #[display(fmt = "Lithuania")]
    LT,
    #[display(fmt = "Latvia")]
    LV,
    #[display(fmt = "Morocco")]
    MA,
    #[display(fmt = "Mexico")]
    MX,
    #[display(fmt = "Malaysia")]
    MY,
    #[display(fmt = "Nigeria")]
    NG,
    #[display(fmt = "Netherlands")]
    NL,
    #[display(fmt = "Norway")]
    NO,
    #[display(fmt = "New Zealand")]
    NZ,
    #[display(fmt = "Philippines")]
    PH,
    #[display(fmt = "Poland")]
    PL,
    #[display(fmt = "Portugal")]
    PT,
    #[display(fmt = "Romania")]
    RO,
    #[display(fmt = "Serbia")]
    RS,
    #[display(fmt = "Russia")]
    RU,
    #[display(fmt = "Saudi Arabia")]
    SA,
    #[display(fmt = "Sweden")]
    SE,
    #[display(fmt = "Singapore")]
    SG,
    #[display(fmt = "Slovenia")]
    SI,
    #[display(fmt = "Slovakia")]
    SK,
    #[display(fmt = "Thailand")]
    TH,
    #[display(fmt = "Turkey")]
    TR,
    #[display(fmt = "Taiwan")]
    TW,
    #[display(fmt = "Ukraine")]
    UA,
    #[display(fmt = "United State")]
    US,
    #[display(fmt = "Venezuela")]
    VE,
    #[display(fmt = "South Africa")]
    ZA,
}
