//! Which writing system a locale's prose has to be in.
//!
//! `sr_rs` is why this exists. Nothing in the prompt named its script -
//! `system_message` only interpolates `nativeName`, so the model was told
//! "translate to Српски (sr_rs)" and nothing more. On a long `.md` body that
//! was enough to stay in Cyrillic; on a one-line `meta-desc.txt` it was not,
//! and 97 of 108 descriptions came back in Latin. Latin Serbian is exactly
//! what `sr_latn_rs` already holds, so 33 guides shipped the same meta
//! description on two URLs and an external crawl flagged every one.
//!
//! Two consumers, and they must agree: [`crate::run::build_prompt`] names the
//! script in the prompt, and [`crate::validate::file_problem`] rejects a
//! translation that came back without it.

/// A locale's expected writing system.
pub struct Script {
    /// How the prompt names it, e.g. "Cyrillic".
    pub name: &'static str,
    /// Inclusive codepoint ranges that count as "in this script".
    ranges: &'static [(u32, u32)],
}

impl Script {
    /// Does `text` contain at least one character in this script?
    pub fn present_in(&self, text: &str) -> bool {
        text.chars()
            .any(|c| self.ranges.iter().any(|(lo, hi)| (*lo..=*hi).contains(&(c as u32))))
    }
}

const CYRILLIC: Script = Script {
    name: "Cyrillic",
    ranges: &[(0x0400, 0x04FF), (0x0500, 0x052F)],
};

const GREEK: Script = Script {
    name: "Greek",
    ranges: &[(0x0370, 0x03FF), (0x1F00, 0x1FFF)],
};

const HEBREW: Script = Script {
    name: "Hebrew",
    ranges: &[(0x0590, 0x05FF)],
};

// Kana plus Han: a Japanese sentence can be Han-heavy, so requiring kana
// specifically would flag legitimate output.
const JAPANESE: Script = Script {
    name: "Japanese (kana and kanji)",
    ranges: &[(0x3040, 0x309F), (0x30A0, 0x30FF), (0x4E00, 0x9FFF)],
};

const HANGUL: Script = Script {
    name: "Hangul",
    ranges: &[(0xAC00, 0xD7AF), (0x1100, 0x11FF), (0x3130, 0x318F)],
};

const HAN: Script = Script {
    name: "Chinese characters",
    ranges: &[(0x4E00, 0x9FFF), (0x3400, 0x4DBF)],
};

/// The script `locale` must be written in, or `None` when it uses the Latin
/// alphabet and there is nothing to assert.
///
/// Simplified vs traditional Chinese share a codepoint block, so `zh_cn` and
/// `zh_tw` get the same [`HAN`] entry: the check only asserts "this is Chinese
/// characters, not English", which is the failure mode actually seen.
pub fn expected_for(locale: &str) -> Option<&'static Script> {
    match locale {
        "bg_bg" | "ru_ru" | "uk_ua" | "sr_rs" => Some(&CYRILLIC),
        "el_gr" => Some(&GREEK),
        "he" => Some(&HEBREW),
        "ja_jp" => Some(&JAPANESE),
        "ko_kr" => Some(&HANGUL),
        "zh_cn" | "zh_tw" => Some(&HAN),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serbian_cyrillic_is_distinguished_from_serbian_latin() {
        let cyr = expected_for("sr_rs").expect("sr_rs has an expected script");
        assert!(cyr.present_in("Додајте коментаре на Gatsby сајт."));
        // The exact string that shipped on 33 guides.
        assert!(!cyr.present_in("Dodajte komentare na Gatsby sajt."));
    }

    #[test]
    fn latin_locales_assert_nothing() {
        assert!(expected_for("fr_fr").is_none());
        assert!(expected_for("sr_latn_rs").is_none());
        assert!(expected_for("en_us").is_none());
    }

    #[test]
    fn each_script_accepts_its_own_language() {
        let cases = [
            ("ja_jp", "GatsbyJS サイトにコメントを追加する"),
            ("ko_kr", "Gatsby 사이트에 댓글을 추가합니다"),
            ("zh_cn", "在 Gatsby 网站中添加评论"),
            ("el_gr", "Προσθέστε σχόλια σε έναν ιστότοπο"),
            ("he", "הוסיפו תגובות לאתר"),
            ("bg_bg", "Добавяне на коментари към вашия сайт"),
        ];
        for (locale, text) in cases {
            let script = expected_for(locale).expect("locale has an expected script");
            assert!(script.present_in(text), "{locale} should accept its own text");
            assert!(
                !script.present_in("Add comments to a Gatsby site."),
                "{locale} should reject English"
            );
        }
    }
}
