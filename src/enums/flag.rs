//! Flag enum for avatars

use serde::{Deserialize, Serialize};

/// Country flag codes for avatar generation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Flag {
    #[serde(rename = "ad")]
    Ad,
    #[serde(rename = "ae")]
    Ae,
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "ag")]
    Ag,
    #[serde(rename = "ai")]
    Ai,
    #[serde(rename = "al")]
    Al,
    #[serde(rename = "am")]
    Am,
    #[serde(rename = "ao")]
    Ao,
    #[serde(rename = "aq")]
    Aq,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "at")]
    At,
    #[serde(rename = "au")]
    Au,
    #[serde(rename = "aw")]
    Aw,
    #[serde(rename = "ax")]
    Ax,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "ba")]
    Ba,
    #[serde(rename = "bb")]
    Bb,
    #[serde(rename = "bd")]
    Bd,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bf")]
    Bf,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bh")]
    Bh,
    #[serde(rename = "bi")]
    Bi,
    #[serde(rename = "bj")]
    Bj,
    #[serde(rename = "bl")]
    Bl,
    #[serde(rename = "bm")]
    Bm,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bo")]
    Bo,
    #[serde(rename = "bq")]
    Bq,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "bt")]
    Bt,
    #[serde(rename = "bv")]
    Bv,
    #[serde(rename = "bw")]
    Bw,
    #[serde(rename = "by")]
    By,
    #[serde(rename = "bz")]
    Bz,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cc")]
    Cc,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "cf")]
    Cf,
    #[serde(rename = "cg")]
    Cg,
    #[serde(rename = "ch")]
    Ch,
    #[serde(rename = "ci")]
    Ci,
    #[serde(rename = "ck")]
    Ck,
    #[serde(rename = "cl")]
    Cl,
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "cn")]
    Cn,
    #[serde(rename = "co")]
    Co,
    #[serde(rename = "cr")]
    Cr,
    #[serde(rename = "cu")]
    Cu,
    #[serde(rename = "cv")]
    Cv,
    #[serde(rename = "cw")]
    Cw,
    #[serde(rename = "cx")]
    Cx,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "cz")]
    Cz,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "dj")]
    Dj,
    #[serde(rename = "dk")]
    Dk,
    #[serde(rename = "dm")]
    Dm,
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "dz")]
    Dz,
    #[serde(rename = "ec")]
    Ec,
    #[serde(rename = "ee")]
    Ee,
    #[serde(rename = "eg")]
    Eg,
    #[serde(rename = "eh")]
    Eh,
    #[serde(rename = "er")]
    Er,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fj")]
    Fj,
    #[serde(rename = "fk")]
    Fk,
    #[serde(rename = "fm")]
    Fm,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "ge")]
    Ge,
    #[serde(rename = "gf")]
    Gf,
    #[serde(rename = "gg")]
    Gg,
    #[serde(rename = "gh")]
    Gh,
    #[serde(rename = "gi")]
    Gi,
    #[serde(rename = "gl")]
    Gl,
    #[serde(rename = "gm")]
    Gm,
    #[serde(rename = "gn")]
    Gn,
    #[serde(rename = "gp")]
    Gp,
    #[serde(rename = "gq")]
    Gq,
    #[serde(rename = "gr")]
    Gr,
    #[serde(rename = "gs")]
    Gs,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "gw")]
    Gw,
    #[serde(rename = "gy")]
    Gy,
    #[serde(rename = "hk")]
    Hk,
    #[serde(rename = "hm")]
    Hm,
    #[serde(rename = "hn")]
    Hn,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "ht")]
    Ht,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ie")]
    Ie,
    #[serde(rename = "il")]
    Il,
    #[serde(rename = "im")]
    Im,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "io")]
    Io,
    #[serde(rename = "iq")]
    Iq,
    #[serde(rename = "ir")]
    Ir,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "je")]
    Je,
    #[serde(rename = "jm")]
    Jm,
    #[serde(rename = "jo")]
    Jo,
    #[serde(rename = "jp")]
    Jp,
    #[serde(rename = "ke")]
    Ke,
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "kh")]
    Kh,
    #[serde(rename = "ki")]
    Ki,
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "kp")]
    Kp,
    #[serde(rename = "kr")]
    Kr,
    #[serde(rename = "kw")]
    Kw,
    #[serde(rename = "ky")]
    Ky,
    #[serde(rename = "kz")]
    Kz,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "lc")]
    Lc,
    #[serde(rename = "li")]
    Li,
    #[serde(rename = "lk")]
    Lk,
    #[serde(rename = "lr")]
    Lr,
    #[serde(rename = "ls")]
    Ls,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lu")]
    Lu,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "ly")]
    Ly,
    #[serde(rename = "ma")]
    Ma,
    #[serde(rename = "mc")]
    Mc,
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "mf")]
    Mf,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "mh")]
    Mh,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mm")]
    Mm,
    #[serde(rename = "mn")]
    Mn,
    #[serde(rename = "mo")]
    Mo,
    #[serde(rename = "mp")]
    Mp,
    #[serde(rename = "mq")]
    Mq,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "mu")]
    Mu,
    #[serde(rename = "mv")]
    Mv,
    #[serde(rename = "mw")]
    Mw,
    #[serde(rename = "mx")]
    Mx,
    #[serde(rename = "my")]
    My,
    #[serde(rename = "mz")]
    Mz,
    #[serde(rename = "na")]
    Na,
    #[serde(rename = "nc")]
    Nc,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "nf")]
    Nf,
    #[serde(rename = "ng")]
    Ng,
    #[serde(rename = "ni")]
    Ni,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "np")]
    Np,
    #[serde(rename = "nr")]
    Nr,
    #[serde(rename = "nu")]
    Nu,
    #[serde(rename = "nz")]
    Nz,
    #[serde(rename = "om")]
    Om,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pe")]
    Pe,
    #[serde(rename = "pf")]
    Pf,
    #[serde(rename = "pg")]
    Pg,
    #[serde(rename = "ph")]
    Ph,
    #[serde(rename = "pk")]
    Pk,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pm")]
    Pm,
    #[serde(rename = "pn")]
    Pn,
    #[serde(rename = "pr")]
    Pr,
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pw")]
    Pw,
    #[serde(rename = "py")]
    Py,
    #[serde(rename = "qa")]
    Qa,
    #[serde(rename = "re")]
    Re,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "rs")]
    Rs,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "rw")]
    Rw,
    #[serde(rename = "sa")]
    Sa,
    #[serde(rename = "sb")]
    Sb,
    #[serde(rename = "sc")]
    Sc,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "se")]
    Se,
    #[serde(rename = "sg")]
    Sg,
    #[serde(rename = "sh")]
    Sh,
    #[serde(rename = "si")]
    Si,
    #[serde(rename = "sj")]
    Sj,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "so")]
    So,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "ss")]
    Ss,
    #[serde(rename = "st")]
    St,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sx")]
    Sx,
    #[serde(rename = "sy")]
    Sy,
    #[serde(rename = "sz")]
    Sz,
    #[serde(rename = "tc")]
    Tc,
    #[serde(rename = "td")]
    Td,
    #[serde(rename = "tf")]
    Tf,
    #[serde(rename = "tg")]
    Tg,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tj")]
    Tj,
    #[serde(rename = "tk")]
    Tk,
    #[serde(rename = "tl")]
    Tl,
    #[serde(rename = "tm")]
    Tm,
    #[serde(rename = "tn")]
    Tn,
    #[serde(rename = "to")]
    To,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "tt")]
    Tt,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "tw")]
    Tw,
    #[serde(rename = "tz")]
    Tz,
    #[serde(rename = "ua")]
    Ua,
    #[serde(rename = "ug")]
    Ug,
    #[serde(rename = "um")]
    Um,
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "uy")]
    Uy,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "va")]
    Va,
    #[serde(rename = "vc")]
    Vc,
    #[serde(rename = "ve")]
    Ve,
    #[serde(rename = "vg")]
    Vg,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "vn")]
    Vn,
    #[serde(rename = "vu")]
    Vu,
    #[serde(rename = "wf")]
    Wf,
    #[serde(rename = "ws")]
    Ws,
    #[serde(rename = "ye")]
    Ye,
    #[serde(rename = "yt")]
    Yt,
    #[serde(rename = "za")]
    Za,
    #[serde(rename = "zm")]
    Zm,
    #[serde(rename = "zw")]
    Zw,
}

impl AsRef<str> for Flag {
    fn as_ref(&self) -> &str {
        match self {
            Flag::Ad => "ad",
            Flag::Ae => "ae",
            Flag::Af => "af",
            Flag::Ag => "ag",
            Flag::Ai => "ai",
            Flag::Al => "al",
            Flag::Am => "am",
            Flag::Ao => "ao",
            Flag::Aq => "aq",
            Flag::Ar => "ar",
            Flag::As => "as",
            Flag::At => "at",
            Flag::Au => "au",
            Flag::Aw => "aw",
            Flag::Ax => "ax",
            Flag::Az => "az",
            Flag::Ba => "ba",
            Flag::Bb => "bb",
            Flag::Bd => "bd",
            Flag::Be => "be",
            Flag::Bf => "bf",
            Flag::Bg => "bg",
            Flag::Bh => "bh",
            Flag::Bi => "bi",
            Flag::Bj => "bj",
            Flag::Bl => "bl",
            Flag::Bm => "bm",
            Flag::Bn => "bn",
            Flag::Bo => "bo",
            Flag::Bq => "bq",
            Flag::Br => "br",
            Flag::Bs => "bs",
            Flag::Bt => "bt",
            Flag::Bv => "bv",
            Flag::Bw => "bw",
            Flag::By => "by",
            Flag::Bz => "bz",
            Flag::Ca => "ca",
            Flag::Cc => "cc",
            Flag::Cd => "cd",
            Flag::Cf => "cf",
            Flag::Cg => "cg",
            Flag::Ch => "ch",
            Flag::Ci => "ci",
            Flag::Ck => "ck",
            Flag::Cl => "cl",
            Flag::Cm => "cm",
            Flag::Cn => "cn",
            Flag::Co => "co",
            Flag::Cr => "cr",
            Flag::Cu => "cu",
            Flag::Cv => "cv",
            Flag::Cw => "cw",
            Flag::Cx => "cx",
            Flag::Cy => "cy",
            Flag::Cz => "cz",
            Flag::De => "de",
            Flag::Dj => "dj",
            Flag::Dk => "dk",
            Flag::Dm => "dm",
            Flag::Do => "do",
            Flag::Dz => "dz",
            Flag::Ec => "ec",
            Flag::Ee => "ee",
            Flag::Eg => "eg",
            Flag::Eh => "eh",
            Flag::Er => "er",
            Flag::Es => "es",
            Flag::Et => "et",
            Flag::Fi => "fi",
            Flag::Fj => "fj",
            Flag::Fk => "fk",
            Flag::Fm => "fm",
            Flag::Fo => "fo",
            Flag::Fr => "fr",
            Flag::Ga => "ga",
            Flag::Gb => "gb",
            Flag::Gd => "gd",
            Flag::Ge => "ge",
            Flag::Gf => "gf",
            Flag::Gg => "gg",
            Flag::Gh => "gh",
            Flag::Gi => "gi",
            Flag::Gl => "gl",
            Flag::Gm => "gm",
            Flag::Gn => "gn",
            Flag::Gp => "gp",
            Flag::Gq => "gq",
            Flag::Gr => "gr",
            Flag::Gs => "gs",
            Flag::Gt => "gt",
            Flag::Gu => "gu",
            Flag::Gw => "gw",
            Flag::Gy => "gy",
            Flag::Hk => "hk",
            Flag::Hm => "hm",
            Flag::Hn => "hn",
            Flag::Hr => "hr",
            Flag::Ht => "ht",
            Flag::Hu => "hu",
            Flag::Id => "id",
            Flag::Ie => "ie",
            Flag::Il => "il",
            Flag::Im => "im",
            Flag::In => "in",
            Flag::Io => "io",
            Flag::Iq => "iq",
            Flag::Ir => "ir",
            Flag::Is => "is",
            Flag::It => "it",
            Flag::Je => "je",
            Flag::Jm => "jm",
            Flag::Jo => "jo",
            Flag::Jp => "jp",
            Flag::Ke => "ke",
            Flag::Kg => "kg",
            Flag::Kh => "kh",
            Flag::Ki => "ki",
            Flag::Km => "km",
            Flag::Kn => "kn",
            Flag::Kp => "kp",
            Flag::Kr => "kr",
            Flag::Kw => "kw",
            Flag::Ky => "ky",
            Flag::Kz => "kz",
            Flag::La => "la",
            Flag::Lb => "lb",
            Flag::Lc => "lc",
            Flag::Li => "li",
            Flag::Lk => "lk",
            Flag::Lr => "lr",
            Flag::Ls => "ls",
            Flag::Lt => "lt",
            Flag::Lu => "lu",
            Flag::Lv => "lv",
            Flag::Ly => "ly",
            Flag::Ma => "ma",
            Flag::Mc => "mc",
            Flag::Md => "md",
            Flag::Me => "me",
            Flag::Mf => "mf",
            Flag::Mg => "mg",
            Flag::Mh => "mh",
            Flag::Mk => "mk",
            Flag::Ml => "ml",
            Flag::Mm => "mm",
            Flag::Mn => "mn",
            Flag::Mo => "mo",
            Flag::Mp => "mp",
            Flag::Mq => "mq",
            Flag::Mr => "mr",
            Flag::Ms => "ms",
            Flag::Mt => "mt",
            Flag::Mu => "mu",
            Flag::Mv => "mv",
            Flag::Mw => "mw",
            Flag::Mx => "mx",
            Flag::My => "my",
            Flag::Mz => "mz",
            Flag::Na => "na",
            Flag::Nc => "nc",
            Flag::Ne => "ne",
            Flag::Nf => "nf",
            Flag::Ng => "ng",
            Flag::Ni => "ni",
            Flag::Nl => "nl",
            Flag::No => "no",
            Flag::Np => "np",
            Flag::Nr => "nr",
            Flag::Nu => "nu",
            Flag::Nz => "nz",
            Flag::Om => "om",
            Flag::Pa => "pa",
            Flag::Pe => "pe",
            Flag::Pf => "pf",
            Flag::Pg => "pg",
            Flag::Ph => "ph",
            Flag::Pk => "pk",
            Flag::Pl => "pl",
            Flag::Pm => "pm",
            Flag::Pn => "pn",
            Flag::Pr => "pr",
            Flag::Ps => "ps",
            Flag::Pt => "pt",
            Flag::Pw => "pw",
            Flag::Py => "py",
            Flag::Qa => "qa",
            Flag::Re => "re",
            Flag::Ro => "ro",
            Flag::Rs => "rs",
            Flag::Ru => "ru",
            Flag::Rw => "rw",
            Flag::Sa => "sa",
            Flag::Sb => "sb",
            Flag::Sc => "sc",
            Flag::Sd => "sd",
            Flag::Se => "se",
            Flag::Sg => "sg",
            Flag::Sh => "sh",
            Flag::Si => "si",
            Flag::Sj => "sj",
            Flag::Sk => "sk",
            Flag::Sl => "sl",
            Flag::Sm => "sm",
            Flag::Sn => "sn",
            Flag::So => "so",
            Flag::Sr => "sr",
            Flag::Ss => "ss",
            Flag::St => "st",
            Flag::Sv => "sv",
            Flag::Sx => "sx",
            Flag::Sy => "sy",
            Flag::Sz => "sz",
            Flag::Tc => "tc",
            Flag::Td => "td",
            Flag::Tf => "tf",
            Flag::Tg => "tg",
            Flag::Th => "th",
            Flag::Tj => "tj",
            Flag::Tk => "tk",
            Flag::Tl => "tl",
            Flag::Tm => "tm",
            Flag::Tn => "tn",
            Flag::To => "to",
            Flag::Tr => "tr",
            Flag::Tt => "tt",
            Flag::Tv => "tv",
            Flag::Tw => "tw",
            Flag::Tz => "tz",
            Flag::Ua => "ua",
            Flag::Ug => "ug",
            Flag::Um => "um",
            Flag::Us => "us",
            Flag::Uy => "uy",
            Flag::Uz => "uz",
            Flag::Va => "va",
            Flag::Vc => "vc",
            Flag::Ve => "ve",
            Flag::Vg => "vg",
            Flag::Vi => "vi",
            Flag::Vn => "vn",
            Flag::Vu => "vu",
            Flag::Wf => "wf",
            Flag::Ws => "ws",
            Flag::Ye => "ye",
            Flag::Yt => "yt",
            Flag::Za => "za",
            Flag::Zm => "zm",
            Flag::Zw => "zw",
        }
    }
}